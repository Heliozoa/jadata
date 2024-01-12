use crate::input::jmdict::{JMdict, Sense};
use jadata::wordfile::{Header, Word, Wordfile};
use std::collections::{HashMap, HashSet};
use wana_kana::ConvertJapanese;

/// Creates the kanjifile skeleton that only contains the bare minimum amount of data.
pub fn create(jmdict: JMdict, jmdict_version: String) -> eyre::Result<Wordfile> {
    let mut skeleton = Wordfile {
        header: Header {
            version: "".to_string(),
            jmdict_version: "".to_string(),
            last_word_id: 0,
        },
        words: Vec::new(),
    };
    update(&mut skeleton, jmdict, jmdict_version)?;
    Ok(skeleton)
}

pub fn update(wordfile: &mut Wordfile, jmdict: JMdict, jmdict_version: String) -> eyre::Result<()> {
    // a jmdict entry can correspond to multiple jadata entries
    // a jmdict id + a written form converted to katakana corresponds to a single jadata entry
    let existing_words = std::mem::take(&mut wordfile.words);
    let mut existing_words = existing_words
        .iter()
        .flat_map(|w| {
            // no need to update words that aren't in JMdict
            let jmdict_id = if let Some(jmdict_id) = w.jmdict_id {
                jmdict_id
            } else {
                return None;
            };
            let key = JMdictWordKatakana {
                jmdict_id,
                // all written forms in a single jadata entry are equivalent when converted to katakana, so we can just pick one
                written_form_katakana: w.written_forms[0].to_katakana(),
            };
            let val = w;
            Some((key, val))
        })
        .collect::<HashMap<_, _>>();

    let jmdict_words = process_jmdict(jmdict);
    let mut katana_to_verbatim: HashMap<JMdictWordKatakana, Vec<JMdictWordVerbatim>> =
        HashMap::new();
    for jmdict_word in jmdict_words {
        let jadata_word = JMdictWordKatakana::from_verbatim(&jmdict_word);
        let entry = katana_to_verbatim.entry(jadata_word).or_default();
        entry.push(jmdict_word);
    }

    // here, we remove words that no longer exist
    // while JMdict entries should not disappear, a word may possibly have a written form changed/removed
    // we could leave them in, but we're treating JMdict as the authority here
    let existing_words_keys = existing_words.keys().cloned().collect::<HashSet<_>>();
    let jmdict_words_keys = katana_to_verbatim.keys().cloned().collect::<HashSet<_>>();
    // removed words are ones that exist in the skeleton currently but no longer found in JMdict
    let removed_words_keys = existing_words_keys.difference(&jmdict_words_keys);
    for removed_word_key in removed_words_keys {
        let removed = existing_words.remove(removed_word_key).unwrap();
        tracing::warn!("removing word {removed:#?}");
    }

    // then, we add in the new words
    // new words are ones that exist in JMdict but not in the skeleton
    katana_to_verbatim.retain(|k, _v| !existing_words_keys.contains(k));
    let mut pairs = katana_to_verbatim.into_iter().collect::<Vec<_>>();
    pairs.sort_by(|a, b| {
        a.0.written_form_katakana
            .cmp(&b.0.written_form_katakana)
            .then(a.0.jmdict_id.cmp(&b.0.jmdict_id))
    });
    let last_word_id = &mut wordfile.header.last_word_id;
    let new_words = pairs
        .into_iter()
        .map(|(ja, jm)| {
            let mut written_forms = HashSet::new();
            for jm in jm {
                written_forms.insert(jm.written_form.clone());
            }
            let mut written_forms = written_forms.into_iter().collect::<Vec<_>>();
            written_forms.sort();
            *last_word_id += 1;
            Word {
                id: *last_word_id,
                jmdict_id: Some(ja.jmdict_id),
                written_forms,
                meanings: vec![],
                readings: vec![],
            }
        })
        .collect::<Vec<_>>();

    wordfile.header.jmdict_version = jmdict_version;
    wordfile.words.extend(new_words);

    Ok(())
}

#[derive(Debug)]
struct JMdictWordVerbatim {
    id: u32,
    written_form: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct JMdictWordKatakana {
    jmdict_id: u32,
    written_form_katakana: String,
}

impl JMdictWordKatakana {
    fn from_verbatim(key: &JMdictWordVerbatim) -> Self {
        let written_form_katakana = key.written_form.to_katakana();
        Self {
            jmdict_id: key.id,
            written_form_katakana,
        }
    }
}

// turn jmdict entries into a list of entries with just the id (seq) and written form
// since in jmdict entries can have multiple written forms, the list of entries has duplicate ids
fn process_jmdict(jmdict: JMdict) -> Vec<JMdictWordVerbatim> {
    let mut jmdict_words = vec![];
    for entry in jmdict.entry {
        let ent_seq = entry.ent_seq;
        let id = ent_seq
            .parse()
            .unwrap_or_else(|_| panic!("invalid ent_seq {ent_seq}"));
        if entry.k_ele.is_empty() {
            for rele in &entry.r_ele {
                jmdict_words.push(process_entry(id, &entry.sense, None, rele.reb.clone()));
            }
        } else {
            for kele in entry.k_ele {
                let keb = kele.keb;
                let mut at_least_one = false;
                for rele in &entry.r_ele {
                    if rele.re_restr.is_empty() || rele.re_restr.contains(&keb) {
                        at_least_one = true;
                        jmdict_words.push(process_entry(
                            id,
                            &entry.sense,
                            Some(keb.clone()),
                            rele.reb.clone(),
                        ));
                    }
                }
                if !at_least_one {
                    tracing::warn!("keb {} had no applicable readings", keb);
                }
            }
        }
    }
    jmdict_words
}

// turns a jmdict entry into an entry with just the id (seq) and written form
fn process_entry(id: u32, sense: &[Sense], keb: Option<String>, reb: String) -> JMdictWordVerbatim {
    let keb = keb.unwrap_or_else(|| reb.clone());
    let mut meanings = vec![];
    for s in sense {
        let stagk = s.stagk.is_empty() || s.stagk.contains(&keb);
        let stagr = s.stagr.is_empty() || s.stagr.contains(&reb);
        if stagk && stagr {
            for g in s.gloss.iter().filter_map(|g| {
                if g.lang.is_none() {
                    Some(g.value.clone())
                } else {
                    None
                }
            }) {
                meanings.push(g);
            }
        }
    }
    JMdictWordVerbatim {
        id,
        written_form: keb,
    }
}
