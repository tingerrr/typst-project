use unicode_ident::{is_xid_continue, is_xid_start};

use crate::{define_conversions, define_formatting, define_serde};

fn is_valid_ident(s: &str) -> Result<(), ParseIdentError> {
    fn is_id_start(c: char) -> bool {
        is_xid_start(c) || c == '_'
    }

    fn is_id_continue(c: char) -> bool {
        is_xid_continue(c) || c == '_' || c == '-'
    }

    let mut chars = s.chars();
    if chars
        .next()
        .is_some_and(|c| is_id_start(c) && chars.all(is_id_continue))
    {
        Ok(())
    } else {
        Err(ParseIdentError::ContainsInvalidChar)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(String);

#[derive(Debug, thiserror::Error)]
pub enum ParseIdentError {
    #[error("url contained invalid byte")]
    ContainsInvalidChar,
}

define_formatting!(Ident);
define_conversions!(Ident, ParseIdentError, is_valid_ident);
define_serde!(Ident, ParseIdentError, is_valid_ident, "an identifier");

#[cfg(test)]
mod test {
    // TODO
}
