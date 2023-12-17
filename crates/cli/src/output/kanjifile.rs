use crate::input::{
    kanjidic2::{self, Character, Kanjidic2},
    kradfile::Kradfile,
};
use jadata::kanjifile::{Kanji, Kanjifile, Position, Reading, ReadingKind};
use std::collections::{HashMap, HashSet};

/// Fills the kanjifile skeleton with data.
pub fn fill_skeleton(skeleton: &mut Kanjifile, kd2: Kanjidic2, kf: Kradfile) {
    let mut skeleton_map = skeleton
        .kanji
        .iter_mut()
        .map(|k| (k.kanji.clone(), k))
        .collect::<HashMap<_, _>>();

    let mut seen_ids = HashSet::new();
    let mut seen_kanji = HashSet::new();
    for kanji in kd2.character {
        let kanji_skeleton = skeleton_map
            .get_mut(&kanji.literal)
            .map(|k| &mut **k)
            .unwrap_or_else(|| panic!("no skeleton for {}", kanji.literal));

        if kanji.literal.chars().count() != 1 {
            panic!("multi-codepoint literal {}", kanji.literal);
        }
        if !seen_kanji.insert(kanji.literal.clone()) {
            panic!("repeated kanji {}", kanji.literal);
        }
        if !seen_ids.insert(kanji_skeleton.id) {
            panic!("repeated id {}", kanji_skeleton.id);
        }
        fill_in_kanji(kanji, kanji_skeleton, &kf.kanji_to_components);
    }
    skeleton.header.kanjidic2_version = kd2.header.file_version;
}

fn fill_in_kanji(
    kanji: Character,
    skeleton: &mut Kanji,
    kanji_to_components: &HashMap<String, Vec<String>>,
) {
    let mut meanings = vec![];
    let mut readings = vec![];
    for rmg in kanji.reading_meaning.into_iter().flat_map(|rm| rm.rmgroup) {
        meanings.extend(handle_meanings(rmg.meaning));
        readings.extend(handle_readings(rmg.reading));
    }
    meanings.sort();
    readings.sort_by(|l, r| l.reading.cmp(&r.reading));

    skeleton.name = skeleton.name.take().or_else(|| meanings.first().cloned());
    skeleton.components = kanji_to_components
        .get(&kanji.literal)
        .cloned()
        .unwrap_or_default();
    skeleton.readings = readings;
    skeleton.meanings = meanings;
}

fn handle_meanings(meanings: Vec<kanjidic2::Meaning>) -> impl Iterator<Item = String> {
    meanings
        .into_iter()
        .filter(|m| m.m_lang.is_none())
        .map(|m| m.value)
}

fn handle_readings(readings: Vec<kanjidic2::Reading>) -> impl Iterator<Item = Reading> {
    readings.into_iter().filter_map(|r| {
        let kind = match r.r_type.as_str() {
            "ja_on" => ReadingKind::Onyomi,
            "ja_kun" => ReadingKind::Kunyomi,
            _ => return None,
        };
        let position = if r.value.starts_with('-') {
            Some(Position::Suffix)
        } else if r.value.ends_with('-') {
            Some(Position::Prefix)
        } else {
            None
        };
        let (reading, okurigana) = if let Some((reading, okurigana)) = r.value.split_once('.') {
            (
                reading.trim_matches('-').to_string(),
                Some(okurigana.trim_matches('-').to_string()),
            )
        } else {
            (r.value.trim_matches('-').to_string(), None)
        };
        Some(Reading {
            kind,
            reading,
            okurigana,
            position,
        })
    })
}
