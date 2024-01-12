use crate::input::{
    kanjidic2::{self, Character, Kanjidic2},
    kradfile::Kradfile,
};
use jadata::kanjifile::{Kanji, Kanjifile};
use std::collections::{HashMap, HashSet};

/// Fills the kanjifile skeleton with data.
pub fn fill_skeleton(skeleton: &mut Kanjifile, version: String, kd2: Kanjidic2, kf: Kradfile) {
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
    skeleton.header.version = version;
    skeleton.header.kanjidic2_version = kd2.header.file_version;
}

fn fill_in_kanji(
    kanji: Character,
    skeleton: &mut Kanji,
    kanji_to_components: &HashMap<String, Vec<String>>,
) {
    let mut meanings = vec![];
    for rmg in kanji.reading_meaning.into_iter().flat_map(|rm| rm.rmgroup) {
        meanings.extend(handle_meanings(rmg.meaning));
    }
    meanings.sort();

    skeleton.name = skeleton.name.take().or_else(|| meanings.first().cloned());
    skeleton.components = kanji_to_components
        .get(&kanji.literal)
        .cloned()
        .unwrap_or_default();
    skeleton.meanings = meanings;
}

fn handle_meanings(meanings: Vec<kanjidic2::Meaning>) -> impl Iterator<Item = String> {
    meanings
        .into_iter()
        .filter(|m| m.m_lang.is_none())
        .map(|m| m.value)
}
