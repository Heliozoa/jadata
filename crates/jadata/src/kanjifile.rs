use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kanjifile {
    pub header: Header,
    pub kanji: Vec<Kanji>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub version: String,
    pub kanjidic2_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kanji {
    pub id: u16,
    pub kanji: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub readings: Vec<Reading>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meanings: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub similar: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading {
    pub reading: String,
    pub kind: ReadingKind,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okurigana: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Position {
    Prefix,
    Suffix,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ReadingKind {
    Onyomi,
    Kunyomi,
}
