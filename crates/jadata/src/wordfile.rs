use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wordfile {
    pub header: Header,
    pub words: Vec<Word>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub version: String,
    pub jmdict_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub id: u32,
    pub jmdict_id: u32,
    pub written_forms: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meanings: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub readings: Vec<Reading>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading {
    pub reading: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub furigana: Vec<Furigana>,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub usually_kana: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Furigana {
    pub start_idx: usize,
    pub end_idx: usize,
    pub furigana: String,
}
