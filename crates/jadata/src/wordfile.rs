//! Contains the data types for working with the wordfile.

use serde::{Deserialize, Serialize};

/// Models the full contents of the wordfile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wordfile {
    pub header: Header,
    pub words: Vec<Word>,
}

/// Contains metadata about the wordfile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// The version of the wordfile.
    pub version: String,
    /// The version of the JMdict that was used as the base for the wordfile.
    pub jmdict_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    /// A stable identifier for the words within the wordfile.
    pub id: u32,
    /// The identifier (`seq`) for the corresponding word within JMdict, if any.
    pub jmdict_id: Option<u32>,
    /// Different written forms for the same word.
    pub written_forms: Vec<String>,
    /// English translations for the different meanings of the word.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meanings: Vec<String>,
    /// Different readings for the same word.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub readings: Vec<Reading>,
}

/// Information on a single reading for a word.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading {
    /// The reading itself in kana.
    pub reading: String,
    /// The reading split into furigana assigned for each kanji section of the word.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub furigana: Vec<Furigana>,
    /// Indicates whether this reading is usually written using kana.
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub usually_kana: bool,
}

/// Maps a reading to a section of kanji within a word.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Furigana {
    /// The start index for the section of kanji.
    pub start_idx: usize,
    /// The end index for the section of kanji.
    pub end_idx: usize,
    /// The portion of the reading that maps to the section of kanji.
    pub furigana: String,
}
