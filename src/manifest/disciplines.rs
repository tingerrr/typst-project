//! Typst package disciplines.

use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

// taken from:
// https://github.com/typst/packages/blob/aac865d4463dd00d7bafc05f31362db27b054309/DISCIPLINES.md

/// A package discipline, indicating the target audience of the package.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Discipline {
    Agriculture,
    Anthropology,
    Archaeology,
    Architecture,
    Biology,
    Business,
    Chemistry,
    Communication,
    ComputerScience,
    Design,
    Drawing,
    Economics,
    Education,
    Engineering,
    Fashion,
    Film,
    Geography,
    Geology,
    History,
    Journalism,
    Law,
    Linguistics,
    Literature,
    Mathematics,
    Medicine,
    Music,
    Painting,
    Philosophy,
    Photography,
    Physics,
    Politics,
    Psychology,
    Sociology,
    Theater,
    Theology,
    Transportation,
}

impl Discipline {
    /// An ordered array of all variants of [Discipline].
    pub const ALL: [Self; 36] = [
        Self::Agriculture,
        Self::Anthropology,
        Self::Archaeology,
        Self::Architecture,
        Self::Biology,
        Self::Business,
        Self::Chemistry,
        Self::Communication,
        Self::ComputerScience,
        Self::Design,
        Self::Drawing,
        Self::Economics,
        Self::Education,
        Self::Engineering,
        Self::Fashion,
        Self::Film,
        Self::Geography,
        Self::Geology,
        Self::History,
        Self::Journalism,
        Self::Law,
        Self::Linguistics,
        Self::Literature,
        Self::Mathematics,
        Self::Medicine,
        Self::Music,
        Self::Painting,
        Self::Philosophy,
        Self::Photography,
        Self::Physics,
        Self::Politics,
        Self::Psychology,
        Self::Sociology,
        Self::Theater,
        Self::Theology,
        Self::Transportation,
    ];

    /// Converts a [Discipline] into it's kebab-case text representation.
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Agriculture => "agriculture",
            Self::Anthropology => "anthropology",
            Self::Archaeology => "archaeology",
            Self::Architecture => "architecture",
            Self::Biology => "biology",
            Self::Business => "business",
            Self::Chemistry => "chemistry",
            Self::Communication => "communication",
            Self::ComputerScience => "computer-science",
            Self::Design => "design",
            Self::Drawing => "drawing",
            Self::Economics => "economics",
            Self::Education => "education",
            Self::Engineering => "engineering",
            Self::Fashion => "fashion",
            Self::Film => "film",
            Self::Geography => "geography",
            Self::Geology => "geology",
            Self::History => "history",
            Self::Journalism => "journalism",
            Self::Law => "law",
            Self::Linguistics => "linguistics",
            Self::Literature => "literature",
            Self::Mathematics => "mathematics",
            Self::Medicine => "medicine",
            Self::Music => "music",
            Self::Painting => "painting",
            Self::Philosophy => "philosophy",
            Self::Photography => "photography",
            Self::Physics => "physics",
            Self::Politics => "politics",
            Self::Psychology => "psychology",
            Self::Sociology => "sociology",
            Self::Theater => "theater",
            Self::Theology => "theology",
            Self::Transportation => "transportation",
        }
    }
}

/// An error returned when parsing a [Discipline] failed.
#[derive(Debug)]
pub struct ParseDisciplineError {
    value: String,
}

impl Display for ParseDisciplineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown discipline '{}'", self.value)
    }
}

impl std::error::Error for ParseDisciplineError {}

impl FromStr for Discipline {
    type Err = ParseDisciplineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "agriculture" => Self::Agriculture,
            "anthropology" => Self::Anthropology,
            "archaeology" => Self::Archaeology,
            "architecture" => Self::Architecture,
            "biology" => Self::Biology,
            "business" => Self::Business,
            "chemistry" => Self::Chemistry,
            "communication" => Self::Communication,
            "computer-science" => Self::ComputerScience,
            "design" => Self::Design,
            "drawing" => Self::Drawing,
            "economics" => Self::Economics,
            "education" => Self::Education,
            "engineering" => Self::Engineering,
            "fashion" => Self::Fashion,
            "film" => Self::Film,
            "geography" => Self::Geography,
            "geology" => Self::Geology,
            "history" => Self::History,
            "journalism" => Self::Journalism,
            "law" => Self::Law,
            "linguistics" => Self::Linguistics,
            "literature" => Self::Literature,
            "mathematics" => Self::Mathematics,
            "medicine" => Self::Medicine,
            "music" => Self::Music,
            "painting" => Self::Painting,
            "philosophy" => Self::Philosophy,
            "photography" => Self::Photography,
            "physics" => Self::Physics,
            "politics" => Self::Politics,
            "psychology" => Self::Psychology,
            "sociology" => Self::Sociology,
            "theater" => Self::Theater,
            "theology" => Self::Theology,
            "transportation" => Self::Transportation,
            _ => {
                return Err(ParseDisciplineError {
                    value: s.to_owned(),
                })
            }
        })
    }
}

impl Display for Discipline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
