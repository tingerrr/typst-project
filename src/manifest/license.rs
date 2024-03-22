use std::cmp::Ordering;
use std::str::FromStr;

use serde::de::{Deserializer, Visitor};
use serde::{Deserialize, Serialize};
use spdx::{Expression, ParseError};
use thiserror::Error;

use crate::define_formatting;

fn is_valid_license(s: &str) -> Result<Expression, ParseLicenseError> {
    let expr = Expression::parse(s)?;

    for requirement in expr.requirements() {
        let Some(id) = requirement.req.license.id() else {
            return Err(ParseLicenseError::ContainsReferencer);
        };

        if !id.is_osi_approved() {
            return Err(ParseLicenseError::NotOSIApproved);
        }
    }

    Ok(expr)
}

#[derive(Clone)]
pub struct License(Expression);

impl PartialEq for License {
    fn eq(&self, other: &Self) -> bool {
        let (this, other): (&str, &str) = (self.0.as_ref(), other.0.as_ref());
        this == other
    }
}

impl Eq for License {}

impl PartialOrd for License {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (this, other): (&str, &str) = (self.0.as_ref(), other.0.as_ref());
        this.partial_cmp(other)
    }
}

impl Ord for License {
    fn cmp(&self, other: &Self) -> Ordering {
        let (this, other): (&str, &str) = (self.0.as_ref(), other.0.as_ref());
        this.cmp(other)
    }
}

impl std::ops::Deref for License {
    type Target = Expression;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<Expression> for License {
    fn as_ref(&self) -> &Expression {
        &self.0
    }
}

impl std::convert::AsRef<str> for License {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::convert::From<License> for Expression {
    fn from(value: License) -> Self {
        value.0
    }
}

#[derive(Debug, Error)]
pub enum ParseLicenseError {
    #[error("invalid license expression")]
    Expression(#[from] ParseError),

    #[error("must not contain referencer")]
    ContainsReferencer,

    #[error("must be OSI-approved")]
    NotOSIApproved,
}

impl std::str::FromStr for License {
    type Err = ParseLicenseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expr = is_valid_license(s)?;
        Ok(Self(expr))
    }
}

define_formatting!(License);

impl Serialize for License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_ref())
    }
}

impl<'de> Deserialize<'de> for License {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LicenseVisitor;

        impl<'de> Visitor<'de> for LicenseVisitor {
            type Value = License;

            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "an OSI-approved license expression")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                License::from_str(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(LicenseVisitor)
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
