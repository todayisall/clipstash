use super::super::ClipError;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use rocket::{UriDisplayPath, UriDisplayQuery};

#[derive(Debug, Clone, Deserialize, Serialize, From, UriDisplayQuery, UriDisplayPath)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;

        let allowed_chars = ['a', 'b', 'c', 'd', '1', '2', '3', '4'];

        let mut rng = thread_rng();
        let mut short_code = String::with_capacity(10);

        for _ in 0..10 {
            short_code.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling array should have values"),
            )
        }

        Self(short_code)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}

impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        ShortCode(shortcode.to_owned())
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

use rocket::request::FromParam; // turn some data into param
impl<'r> FromParam<'r> for ShortCode {
  type Error = &'r str;
  fn from_param(param: &'r str) -> Result<Self, Self::Error> {
    Ok(ShortCode::from(param))
  }
}