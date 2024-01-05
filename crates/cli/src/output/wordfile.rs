use crate::input::{
    jmdict::{JMdict, REle, Sense},
    jmdict_furigana,
};
use eyre::Context;
use jadata::wordfile::{Furigana, Reading, Word, Wordfile};
use std::collections::HashMap;

/// Fills the wordfile skeleton with data.
pub fn fill_skeleton(
    skeleton: &mut Wordfile,
    jmdict: JMdict,
    jmdict_version: String,
    furigana: Vec<jmdict_furigana::Furigana>,
) -> eyre::Result<()> {
    let furigana = process_furigana(furigana);
    let mut skeleton_map: HashMap<u32, Vec<&mut Word>> = HashMap::new();
    for word in skeleton.words.iter_mut() {
        let entry = skeleton_map.entry(word.jmdict_id).or_default();
        entry.push(word);
    }
    let jmdict_words = process_jmdict(jmdict, &furigana)?;
    for jmdict_word in jmdict_words {
        let jmdict_id = jmdict_word.jmdict_id;
        let words = skeleton_map
            .get_mut(&jmdict_id)
            .unwrap_or_else(|| panic!("missing jmdict_id {jmdict_id}"));
        for word in words {
            if !word.written_forms.contains(&jmdict_word.written_form) {
                continue;
            }
            word.meanings = jmdict_word.meanings.clone();
            if let Some(reading) = &jmdict_word.reading {
                word.readings.push(Reading {
                    furigana: jmdict_word.furigana.clone(),
                    reading: reading.clone(),
                    usually_kana: jmdict_word.usually_kana,
                });
            }
        }
    }

    skeleton.header.jmdict_version = jmdict_version;
    Ok(())
}

// word, reading -> furigana
fn process_furigana(
    furigana: Vec<jmdict_furigana::Furigana>,
) -> HashMap<(String, String), Vec<Furigana>> {
    furigana
        .into_iter()
        .map(|f| {
            let key = (f.text, f.reading);
            let mut furigana = vec![];
            let mut start_idx = 0;
            for ruby in f.furigana {
                let end_idx = start_idx + ruby.ruby.len();
                if let Some(rt) = ruby.rt {
                    furigana.push(Furigana {
                        start_idx,
                        end_idx,
                        furigana: rt,
                    });
                }
                start_idx = end_idx;
            }
            (key, furigana)
        })
        .collect()
}

fn process_jmdict(
    jmdict: JMdict,
    furigana: &HashMap<(String, String), Vec<Furigana>>,
) -> eyre::Result<Vec<JMdictWord>> {
    let mut tuples = vec![];
    for entry in jmdict.entry {
        let jmdict_id = entry.ent_seq.parse().wrap_err("invalid id")?;
        if entry.k_ele.is_empty() {
            for rele in &entry.r_ele {
                tuples.push(process_jmdict_word(
                    jmdict_id,
                    furigana,
                    &entry.sense,
                    None,
                    rele,
                    false,
                ));
            }
        } else {
            for kele in entry.k_ele {
                let rare_written_form = kele.ke_inf.iter().any(|s| s == "rarely-used kanji form");
                let keb = kele.keb;
                for rele in &entry.r_ele {
                    if rele.re_restr.is_empty() || rele.re_restr.contains(&keb) {
                        tuples.push(process_jmdict_word(
                            jmdict_id,
                            furigana,
                            &entry.sense,
                            Some(keb.clone()),
                            rele,
                            rare_written_form,
                        ));
                    }
                }
            }
        }
    }
    Ok(tuples)
}

fn process_jmdict_word(
    jmdict_id: u32,
    furigana: &HashMap<(String, String), Vec<Furigana>>,
    sense: &[Sense],
    keb: Option<String>,
    rele: &REle,
    rare_written_form: bool,
) -> JMdictWord {
    let reb = rele.reb.clone();
    let keb = keb.unwrap_or_else(|| reb.clone());
    let tuple = (keb.clone(), reb.clone());
    let furigana = furigana.get(&tuple).cloned().unwrap_or_default();
    let mut usually_kana = rare_written_form;
    let mut meanings = vec![];
    for s in sense {
        if s.misc
            .iter()
            .any(|m| m == "word usually written using kana alone")
        {
            usually_kana = true;
        }
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
        jmdict_id,
        written_form: keb.clone(),
        reading: if keb == reb { None } else { Some(reb) },
        furigana,
        meanings,
        usually_kana,
    }
}

#[derive(Debug)]
struct JMdictWord {
    jmdict_id: u32,
    written_form: String,
    reading: Option<String>,
    furigana: Vec<Furigana>,
    meanings: Vec<String>,
    usually_kana: bool,
}
