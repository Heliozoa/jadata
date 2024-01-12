use crate::input::{jmdict::JMdict, kanjidic2::Kanjidic2};
use jadata::kanjifile::{Header, Kanji, Kanjifile};
use std::{collections::HashSet, ops::Range};

/// Creates the skeleton for a kanjifile that only contains the bare minimum information for each kanji.
pub fn create(kd2: Kanjidic2, jmdict: JMdict) -> eyre::Result<Kanjifile> {
    let header = Header {
        version: "".to_string(),
        kanjidic2_version: "".to_string(),
    };
    let mut kf = Kanjifile {
        header,
        kanji: Vec::new(),
    };
    update(&mut kf, kd2, jmdict)?;
    Ok(kf)
}

/// Updates a kanjifile with new kanji from the Kanjidic2.
pub fn update(kanjifile: &mut Kanjifile, kd2: Kanjidic2, jmdict: JMdict) -> eyre::Result<()> {
    let existing_kanji = kanjifile
        .kanji
        .iter()
        .map(|k| &k.kanji)
        .collect::<HashSet<_>>();
    let new_kanji_from_kanjidic = kd2
        .character
        .into_iter()
        .filter(|c| !existing_kanji.contains(&c.literal))
        .map(|c| c.literal)
        .collect::<HashSet<_>>();

    let kebs = jmdict
        .entry
        .into_iter()
        .flat_map(|e| e.k_ele)
        .map(|k| k.keb)
        .collect::<Vec<_>>();
    let new_kanji_from_jmdict = kebs
        .iter()
        .flat_map(|keb| keb.chars())
        .inspect(|c| {
            if *c == '仝' {
                tracing::info!("here")
            }
        })
        .filter(|char| {
            let char_s = char.to_string();
            is_kanji(*char)
                && !existing_kanji.contains(&char_s)
                && !new_kanji_from_kanjidic.contains(&char_s)
        })
        .inspect(|c| {
            if *c == '仝' {
                tracing::info!("there")
            }
        })
        .map(|c| c.to_string())
        .collect::<HashSet<_>>();

    let mut new_kanji = new_kanji_from_kanjidic
        .into_iter()
        .chain(new_kanji_from_jmdict.into_iter().inspect(|char| {
            tracing::info!(
                "found new kanji in JMdict not present in the kanjidic2/skeleton: '{char}'"
            )
        }))
        .collect::<Vec<_>>();
    new_kanji.sort();
    let mut last_kanji_id = kanjifile.kanji.iter().map(|k| k.id).max().unwrap_or(0);
    let new_kanji = new_kanji.into_iter().map(|kanji| {
        last_kanji_id += 1;
        Kanji {
            id: last_kanji_id,
            kanji,
            name: None,
            components: vec![],
            meanings: vec![],
            similar: vec![],
        }
    });
    kanjifile.header.kanjidic2_version = kd2.header.file_version;
    kanjifile.kanji.extend(new_kanji);

    Ok(())
}

fn is_kanji(c: char) -> bool {
    let c = c as u32;
    CJK.contains(&c)
        || CJK_EXT_B.contains(&c)
        || CJK_EXT_C.contains(&c)
        || CJK_EXT_D.contains(&c)
        || CJK_EXT_E.contains(&c)
        || CJK_EXT_F.contains(&c)
        || CJK_EXT_G.contains(&c)
        || CJK_EXT_H.contains(&c)
}

const CJK: Range<u32> = 0x4E00..0x9FFF;
const CJK_EXT_B: Range<u32> = 0x20000..0x2A6DF;
const CJK_EXT_C: Range<u32> = 0x2A700..0x2B73F;
const CJK_EXT_D: Range<u32> = 0x2B740..0x2B81F;
const CJK_EXT_E: Range<u32> = 0x2B820..0x2CEAF;
const CJK_EXT_F: Range<u32> = 0x2CEB0..0x2EBEF;
const CJK_EXT_G: Range<u32> = 0x30000..0x3134F;
const CJK_EXT_H: Range<u32> = 0x31350..0x323AF;
