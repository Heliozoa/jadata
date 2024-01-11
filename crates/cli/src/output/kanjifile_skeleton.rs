use crate::input::kanjidic2::Kanjidic2;
use jadata::kanjifile::{Header, Kanji, Kanjifile};
use std::collections::HashSet;

/// Creates the skeleton for a kanjifile that only contains the bare minimum information for each kanji.
pub fn create(kd2: Kanjidic2) -> eyre::Result<Kanjifile> {
    let header = Header {
        version: "".to_string(),
        kanjidic2_version: "".to_string(),
    };
    let mut kf = Kanjifile {
        header,
        kanji: Vec::new(),
    };
    update(&mut kf, kd2)?;
    Ok(kf)
}

/// Updates a kanjifile with new kanji from the Kanjidic2.
pub fn update(kanjifile: &mut Kanjifile, mut kd2: Kanjidic2) -> eyre::Result<()> {
    kd2.character.sort_by(|a, b| a.literal.cmp(&b.literal));
    let existing_kanji = kanjifile
        .kanji
        .iter()
        .map(|k| &k.kanji)
        .collect::<HashSet<_>>();
    let mut last_kanji_id = kanjifile.kanji.iter().map(|k| k.id).max().unwrap_or(0);
    let new_kanji = kd2
        .character
        .into_iter()
        .filter(|c| !existing_kanji.contains(&c.literal))
        .map(|c| {
            last_kanji_id += 1;
            Kanji {
                id: last_kanji_id,
                kanji: c.literal,
                components: vec![],
                name: None,
                meanings: vec![],
                similar: vec![],
            }
        })
        .collect::<Vec<_>>();

    kanjifile.header.version = kd2.header.file_version;
    kanjifile.kanji.extend(new_kanji);

    Ok(())
}
