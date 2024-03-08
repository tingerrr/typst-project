//! Typst package categories.

use std::fmt::{Debug, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

// taken from:
// https://github.com/typst/packages/blob/aac865d4463dd00d7bafc05f31362db27b054309/CATEGORIES.md

/// A packages category.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    /// Building blocks for documents. This includes boxes, layout elements,
    /// marginals, icon packs, color palettes, and more.
    Components,

    /// Packages producing compelling visual representations of data,
    /// information, and models.
    Visualization,

    /// Tools for managing semantic information and references. Examples could
    /// be glossaries and bibliographic tools.
    Model,

    /// Primitives and helpers to achieve advanced layouts and set up a page
    /// with headers, margins, and multiple content flows.
    Layout,

    /// Packages that transform text and strings or are focused on fonts.
    Text,

    /// Tools for localization and internationalization as well as dealing with
    /// different scripts and languages in the same document.
    Languages,

    /// Packages/libraries focused on the programmatic aspect of Typst, useful
    /// for automating documents.
    Scripting,

    /// Integrations with third-party tools and formats. In particular, this
    /// includes packages that embed a third-party binary as a plugin.
    Integration,

    /// Auxiliary packages/tools, for example for creating compatibility and
    /// authoring packages.
    Utility,

    /// Unique uses of Typst that are not necessarily practical, but always
    /// entertaining.
    Fun,

    /// Long-form fiction and non-fiction books with multiple chapters.
    Book,

    /// A multipage informational or investigative document focused on a single
    /// topic. This category contains templates for tech reports, homework,
    /// proposals and more.
    Report,

    /// A scientific treatment on a research question. Usually published in a
    /// journal or conference proceedings.
    Paper,

    /// A final long-form deliverable concluding an academic degree.
    Thesis,

    /// A large-scale graphics-heavy presentation of a topic. A poster is
    /// intended to give its reader a first overview over a topic at a glance.
    Poster,

    /// Graphics-heavy, small leaflets intended for massive circulation and to
    /// inform or convince.
    Flyer,

    /// Slides for a projected, oral presentation.
    Presentation,

    /// A résumé or curriculum vitæ presenting the author's professional
    /// achievements in a compelling manner.
    Cv,

    /// Staples for the day-to-day in an office, such as a letter or an invoice.
    Office,
}

impl Category {
    /// An ordered array of all variants of [Category].
    pub const ALL: [Self; 19] = [
        Self::Book,
        Self::Components,
        Self::Cv,
        Self::Flyer,
        Self::Fun,
        Self::Integration,
        Self::Languages,
        Self::Layout,
        Self::Model,
        Self::Office,
        Self::Paper,
        Self::Poster,
        Self::Presentation,
        Self::Report,
        Self::Scripting,
        Self::Text,
        Self::Thesis,
        Self::Utility,
        Self::Visualization,
    ];

    /// An ordered array of variants which describe the functionailty a package provides.
    pub const FUNCTIONAL: [Self; 10] = [
        Self::Components,
        Self::Fun,
        Self::Integration,
        Self::Languages,
        Self::Layout,
        Self::Model,
        Self::Scripting,
        Self::Text,
        Self::Utility,
        Self::Visualization,
    ];

    /// An ordered array of variants which are related to publication. These are commonly used for
    /// template packages.
    pub const PUBLICATION: [Self; 9] = [
        Self::Book,
        Self::Cv,
        Self::Flyer,
        Self::Office,
        Self::Paper,
        Self::Poster,
        Self::Presentation,
        Self::Report,
        Self::Thesis,
    ];

    /// Converts a [Category] into it's kebab-case text representation.
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Components => "components",
            Self::Visualization => "visualization",
            Self::Model => "model",
            Self::Layout => "layout",
            Self::Text => "text",
            Self::Languages => "languages",
            Self::Scripting => "scripting",
            Self::Integration => "integration",
            Self::Utility => "utility",
            Self::Fun => "fun",
            Self::Book => "book",
            Self::Report => "report",
            Self::Paper => "paper",
            Self::Thesis => "thesis",
            Self::Poster => "poster",
            Self::Flyer => "flyer",
            Self::Presentation => "presentation",
            Self::Cv => "cv",
            Self::Office => "office",
        }
    }
}

/// An error returned when parsing a [Category] failed.
#[derive(Debug)]
pub struct ParseCategoryError {
    value: String,
}

impl Display for ParseCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown category '{}'", self.value)
    }
}

impl std::error::Error for ParseCategoryError {}

impl FromStr for Category {
    type Err = ParseCategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "components" => Self::Components,
            "visualization" => Self::Visualization,
            "model" => Self::Model,
            "layout" => Self::Layout,
            "text" => Self::Text,
            "languages" => Self::Languages,
            "scripting" => Self::Scripting,
            "integration" => Self::Integration,
            "utility" => Self::Utility,
            "fun" => Self::Fun,
            "book" => Self::Book,
            "report" => Self::Report,
            "paper" => Self::Paper,
            "thesis" => Self::Thesis,
            "poster" => Self::Poster,
            "flyer" => Self::Flyer,
            "presentation" => Self::Presentation,
            "cv" => Self::Cv,
            "office" => Self::Office,
            _ => {
                return Err(ParseCategoryError {
                    value: s.to_owned(),
                })
            }
        })
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
