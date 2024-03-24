//! Typst package disciplines.

use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};

// taken from:
// https://github.com/typst/packages/blob/aac865d4463dd00d7bafc05f31362db27b054309/DISCIPLINES.md

/// A package discipline, indicating the target audience of the package.
#[non_exhaustive]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, IntoStaticStr,
)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
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
        self.into()
    }
}
