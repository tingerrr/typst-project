use std::fmt::Display;
use std::str::FromStr;

pub use email_address::{EmailAddress, Error as ParseEmailError};
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};
use thiserror::Error;
use unscanny::Scanner;

pub use self::github_handle::{GitHubHandle, ParseGitHubHandleError};
pub use super::website::{ParseWebsiteError, Website};

pub mod github_handle;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Author {
    pub name: String,
    pub contact: Option<Contact>,
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")?;
        if let Some(contact) = &self.contact {
            match contact {
                Contact::GitHubHandle(handle) => write!(f, " <@{handle}>"),
                Contact::Website(website) => write!(f, " <{website}>"),
                Contact::Email(email) => write!(f, " <{email}>"),
            }?;
        }

        Ok(())
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseAuthorError {
    #[error("invalid contact")]
    InvalidEmailAddress(#[from] ParseEmailError),

    #[error("invalid contact")]
    InvalidGithubHandle(#[from] ParseGitHubHandleError),

    #[error("invalid contact")]
    InvalidWebsite(#[from] ParseWebsiteError),

    #[error("missing '>'")]
    UnclosedContact,

    #[error("no contact between '<' and '>'")]
    EmptyContact,
}

impl FromStr for Author {
    type Err = ParseAuthorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = Scanner::new(s);

        let name = s.eat_until('<');
        let contact = if s.eat_if('<') {
            let contact = s.eat_until('>');

            if contact.is_empty() {
                return Err(ParseAuthorError::EmptyContact);
            }

            if !s.eat_if('>') {
                return Err(ParseAuthorError::UnclosedContact);
            }

            Some(if let Some(contact) = contact.strip_prefix('@') {
                Contact::GitHubHandle(contact.parse()?)
            } else if contact.starts_with("http") {
                Contact::Website(contact.parse()?)
            } else {
                Contact::Email(contact.parse()?)
            })
        } else {
            None
        };

        Ok(Self {
            name: name.trim().to_owned(),
            contact,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Contact {
    GitHubHandle(GitHubHandle),
    Website(Website),
    Email(EmailAddress),
}

impl Serialize for Author {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Author {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // NOTE: taking ownership dosn't give us any advantage for Author because of how from_str is
        // implemented
        struct AuthorVisitor;

        impl<'de> Visitor<'de> for AuthorVisitor {
            type Value = Author;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a name with optional contact")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::Value::from_str(v).map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
            }
        }

        deserializer.deserialize_str(AuthorVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_err, assert_ok};

    #[test]
    fn invalid() {
        assert_err!(
            Author::from_str("Martin <>"),
            ParseAuthorError::EmptyContact
        );
        assert_err!(
            Author::from_str("Martin <@ martin>"),
            ParseAuthorError::InvalidGithubHandle(ParseGitHubHandleError::ContainsInvalidChar(' ')),
        );
        assert_err!(
            Author::from_str("Martin <https://mä>"),
            ParseAuthorError::InvalidWebsite(ParseWebsiteError::ContainsInvalidChar),
        );
        assert_err!(
            Author::from_str("Martin <martin@>"),
            ParseAuthorError::InvalidEmailAddress(ParseEmailError::DomainEmpty),
        );
        assert_err!(
            Author::from_str("Martin <martin@typst.app"),
            ParseAuthorError::UnclosedContact,
        );
    }

    #[test]
    fn valid() {
        assert_ok!(
            Author::from_str("Martin"),
            Author {
                name: "Martin".into(),
                contact: None
            },
        );
        assert_ok!(
            Author::from_str("Martin <@reknih>"),
            Author {
                name: "Martin".into(),
                contact: Some(Contact::GitHubHandle(
                    GitHubHandle::from_str("reknih").unwrap()
                ))
            },
        );
        assert_ok!(
            Author::from_str("Martin <https://mha.ug>"),
            Author {
                name: "Martin".into(),
                contact: Some(Contact::Website(
                    Website::from_str("https://mha.ug").unwrap()
                ))
            },
        );
        assert_ok!(
            Author::from_str("Martin <martin.haug@typst.app>"),
            Author {
                name: "Martin".into(),
                contact: Some(Contact::Email(
                    EmailAddress::from_str("martin.haug@typst.app").unwrap()
                ))
            },
        );
    }
}
