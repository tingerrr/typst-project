#[macro_export]
macro_rules! define_conversions {
    ($type:ident, $err_type:ident, $validator:ident) => {
        impl ::std::ops::Deref for $type {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::convert::AsRef<str> for $type {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl ::std::convert::From<$type> for String {
            fn from(value: $type) -> Self {
                value.0
            }
        }

        impl ::std::convert::TryFrom<String> for $type {
            type Error = $err_type;

            fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
                $validator(&value)?;
                Ok(Self(value))
            }
        }

        impl ::std::str::FromStr for $type {
            type Err = $err_type;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $validator(s)?;
                Ok(Self(s.to_owned()))
            }
        }
    };
}

#[macro_export]
macro_rules! define_serde {
    ($type:ident, $err_type:ident, $validator:ident, $expecting:literal) => {
        impl ::serde::ser::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                struct TypeVisitor;

                impl<'de> ::serde::de::Visitor<'de> for TypeVisitor {
                    type Value = $type;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        write!(f, $expecting)
                    }

                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        $validator(&v).map_err(|_| {
                            E::invalid_value(::serde::de::Unexpected::Str(&v), &self)
                        })?;
                        Ok($type(v))
                    }
                }

                deserializer.deserialize_string(TypeVisitor)
            }
        }
    };
}

#[macro_export]
macro_rules! define_formatting {
    ($type:ident) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}

#[macro_export]
macro_rules! assert_ok {
    ($res:expr $(,)?) => {
        assert!($res.is_ok());
    };
    ($given:expr, $expected:expr $(,)?) => {
        match $given {
            Ok(ok) => assert_eq!(ok, $expected),
            Err(err) => panic!("expected ok, got error\n ok: {:?}\nerr: {err:?}", $expected),
        }
    };
}

#[macro_export]
macro_rules! assert_err {
    ($res:expr $(,)?) => {
        assert!($res.is_err());
    };
    ($given:expr, $expected:expr $(,)?) => {
        match $given {
            Err(err) => assert_eq!(err, $expected),
            Ok(ok) => panic!("expected err, got ok\nerr: {:?}\n ok: {ok:?}", $expected),
        }
    };
}
