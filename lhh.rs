#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use structified_enum::structify;
pub struct ErrorType(i32);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ErrorType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ErrorType {
    #[inline]
    fn eq(&self, other: &ErrorType) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for ErrorType {}
#[automatically_derived]
impl ::core::cmp::Eq for ErrorType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
    }
}
impl ErrorType {
    pub const NEED_LOGIN: Self = Self(1);
    pub const COMMON: Self = Self(0);
    pub const fn new(value: i32) -> Self {
        Self(value)
    }
    pub const fn value(self) -> i32 {
        self.0
    }
}
pub enum ErrorTypeParseError {
    UnrecognizedValue(i32),
    UnrecognizedString(String),
}
#[automatically_derived]
impl ::core::fmt::Debug for ErrorTypeParseError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ErrorTypeParseError::UnrecognizedValue(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "UnrecognizedValue",
                    &__self_0,
                )
            }
            ErrorTypeParseError::UnrecognizedString(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "UnrecognizedString",
                    &__self_0,
                )
            }
        }
    }
}
impl ::std::error::Error for ErrorTypeParseError {}
impl ::std::fmt::Display for ErrorTypeParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            ErrorTypeParseError::UnrecognizedValue(v) => {
                f.write_fmt(
                    format_args!(
                        "{0} Parse Unrecognized value: {1}",
                        "ErrorTypeParseError",
                        v,
                    ),
                )
            }
            ErrorTypeParseError::UnrecognizedString(s) => {
                f.write_fmt(
                    format_args!(
                        "{0} Parse Unrecognized string: {1}",
                        "ErrorTypeParseError",
                        s,
                    ),
                )
            }
        }
    }
}
impl ::core::convert::From<i32> for ErrorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::core::convert::From<ErrorType> for i32 {
    fn from(value: ErrorType) -> Self {
        value.0
    }
}
impl ::core::convert::TryFrom<ErrorType> for String {
    type Error = ErrorTypeParseError;
    fn try_from(value: ErrorType) -> ::std::result::Result<String, Self::Error> {
        match value {
            ErrorType::NEED_LOGIN => Ok("NEED_LOGIN".to_string()),
            ErrorType::COMMON => Ok("COMMON".to_string()),
            _ => Err(ErrorTypeParseError::UnrecognizedValue(value.0)),
        }
    }
}
impl ::core::convert::TryFrom<&str> for ErrorType {
    type Error = ErrorTypeParseError;
    fn try_from(value: &str) -> ::std::result::Result<ErrorType, Self::Error> {
        match value {
            "NEED_LOGIN" => Ok(ErrorType::NEED_LOGIN),
            "COMMON" => Ok(ErrorType::COMMON),
            _ => Err(ErrorTypeParseError::UnrecognizedString(value.to_string())),
        }
    }
}
const _: () = {
    pub enum ErrorType {
        COMMON = 0,
        NEED_LOGIN = 1,
    }
};
fn main() {}
