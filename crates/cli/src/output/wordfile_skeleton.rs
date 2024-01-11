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
        },
        words: Vec::new(),
    };
    update(&mut skeleton, jmdict, jmdict_version)?;
    Ok(skeleton)
}

pub fn update(wordfile: &mut Wordfile, jmdict: JMdict, jmdict_version: String) -> eyre::Result<()> {
    let jmdict_words = process_jmdict(jmdict);
    let mut jadata_word_to_jmdict_words: HashMap<JadataWord, Vec<JMdictWord>> = HashMap::new();
    for jmdict_word in jmdict_words {
        let jadata_word = JadataWord::from_jmdict_word(&jmdict_word);
        let entry = jadata_word_to_jmdict_words.entry(jadata_word).or_default();
        entry.push(jmdict_word);
    }
    let mut pairs = jadata_word_to_jmdict_words.into_iter().collect::<Vec<_>>();
    pairs.sort_by(|a, b| {
        a.0.written_form_katakana
            .cmp(&b.0.written_form_katakana)
            .then(a.0.jmdict_id.cmp(&b.0.jmdict_id))
    });

    let mut last_word_id = wordfile.words.iter().map(|w| w.id).max().unwrap_or(0);
    let words = pairs
        .into_iter()
        .map(|(ja, jm)| {
            let mut written_forms = HashSet::new();
            for jm in jm {
                written_forms.insert(jm.written_form.clone());
            }
            let mut written_forms = written_forms.into_iter().collect::<Vec<_>>();
            written_forms.sort();
            last_word_id += 1;
            Word {
                id: last_word_id,
                jmdict_id: Some(ja.jmdict_id),
                written_forms,
                meanings: vec![],
                readings: vec![],
            }
        })
        .collect::<Vec<_>>();

    wordfile.header.jmdict_version = jmdict_version;
    wordfile.words = words;

    Ok(())
}

#[derive(Debug)]
struct JMdictWord {
    id: u32,
    written_form: String,
}

// uniquely identifies a jadata word
#[derive(Debug, PartialEq, Eq, Hash)]
struct JadataWord {
    jmdict_id: u32,
    written_form_katakana: String,
}

impl JadataWord {
    fn from_jmdict_word(tuple: &JMdictWord) -> Self {
        let written_form_katakana = tuple.written_form.to_katakana();
        Self {
            jmdict_id: tuple.id,
            written_form_katakana,
        }
    }
}

// turn jmdict entries into tuples of id, written form, reading and meanings
fn process_jmdict(jmdict: JMdict) -> Vec<JMdictWord> {
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

fn process_entry(id: u32, sense: &[Sense], keb: Option<String>, reb: String) -> JMdictWord {
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
    JMdictWord {
        id,
        written_form: keb,
    }
}
