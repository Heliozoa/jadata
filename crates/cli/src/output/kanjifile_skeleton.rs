use crate::input::kanjidic2::Kanjidic2;
use jadata::kanjifile::{Header, Kanji, Kanjifile};

pub fn create(mut kd2: Kanjidic2) -> eyre::Result<Kanjifile> {
    kd2.character.sort_by(|a, b| a.literal.cmp(&b.literal));
    let kanji = kd2
        .character
        .into_iter()
        .enumerate()
        .map(|(i, c)| Kanji {
            id: i as u16 + 1,
            kanji: c.literal,
            components: vec![],
            name: None,
            readings: vec![],
            meanings: vec![],
            similar: vec![],
        })
        .collect();
    let header = Header {
        version: "".to_string(),
        kanjidic2_version: kd2.header.file_version,
    };
    let kf = Kanjifile { header, kanji };
    Ok(kf)
}
