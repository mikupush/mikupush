// Copyright 2025 Miku Push! Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum DbError {
    NotFound { message: String },
    Generic { message: String },
    MapError { message: String },
}

impl From<diesel::result::Error> for DbError {
    fn from(error: diesel::result::Error) -> Self {
        DbError::Generic { message: error.to_string() }
    }
}

impl From<r2d2::Error> for DbError {
    fn from(error: r2d2::Error) -> Self {
        DbError::Generic { message: error.to_string() }
    }
}

impl Debug for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::NotFound { message } => write!(f, "DbError::NotFound {{ message: {:?} }}", message),
            DbError::Generic { message } => write!(f, "DbError::Generic {{ message: {:?} }}", message),
            DbError::MapError { message } => write!(f, "DbError::MapError {{ message: {:?} }}", message),
        }
    }
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::NotFound { message } => write!(f, "Not found: {}", message),
            DbError::Generic { message } => write!(f, "Generic error: {}", message),
            DbError::MapError { message } => write!(f, "Map error: {}", message),
        }
    }
}

impl Error for DbError {
}

impl From<uuid::Error> for DbError {
    fn from(value: uuid::Error) -> Self {
        DbError::MapError { message: value.to_string() }
    }
}

impl From<mikupush_common::ParseError> for DbError {
    fn from(value: mikupush_common::ParseError) -> Self {
        DbError::MapError { message: value.to_string() }
    }
}