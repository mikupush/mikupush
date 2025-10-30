// Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
// Copyright (C) 2025  Miku Push! Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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