//! Contains the data types for working with the kanjifile.

use serde::{Deserialize, Serialize};

/// Models the full contents of the kanjifile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kanjifile {
    pub header: Header,
    pub kanji: Vec<Kanji>,
}

/// Contains metadata about the kanjifile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// The version of the kanjifile.
    pub version: String,
    /// The version of the kanjidic2 that was used as the base for the kanjifile.
    pub kanjidic2_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kanji {
    /// A stable identifier for the kanji within the kanjifile.
    pub id: u16,
    /// The kanji itself.
    pub kanji: String,
    /// The components of the kanji.
    /// Note that these are not canonical or "official", but may be helpful nonetheless.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<String>,
    /// A name of the kanji.
    /// Note that this is name exists only to associate the character with some English name
    /// to help with retaining it in memory and is not in any way official.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A list of translated meanings for the kanji.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meanings: Vec<String>,
    /// A list of kanji that are visually similar to this kanji.
    /// For example, 人 and 入 are often confused by learners.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub similar: Vec<String>,
}
