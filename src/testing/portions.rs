// finshir: A coroutines-driven Low & Slow traffic sender, written in Rust
// Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>
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
//
// For more information see <https://github.com/Gymmasssorla/finshir>.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io;
use std::path::Path;

use serde_json;

pub type ReadPortionsResult = Result<Vec<Vec<u8>>, ReadPortionsError>;

const EMPTY_SPACES_COUNT: usize = 100;

/// If `file` is some, this function reads data portions from the specified
/// file. Otherwise, it generates empty spaces.
pub fn get_portions<P: AsRef<Path>>(file: Option<P>) -> ReadPortionsResult {
    if let Some(file) = file {
        read_portions(file)
    } else {
        Ok(gen_portions())
    }
}

/// Extracts data portions from a specified file.
fn read_portions<P: AsRef<Path>>(path: P) -> ReadPortionsResult {
    let file = File::open(path).map_err(ReadPortionsError::ReadFailed)?;

    Ok(serde_json::from_reader::<_, Vec<String>>(file)
        .map_err(ReadPortionsError::JsonParseFailed)?
        .into_iter()
        .map(String::into_bytes)
        .collect())
}

/// Generates empty spaces as default data portions.
fn gen_portions() -> Vec<Vec<u8>> {
    let mut spaces = Vec::with_capacity(EMPTY_SPACES_COUNT);
    spaces.resize(EMPTY_SPACES_COUNT, vec![b' ']);
    spaces
}

#[derive(Debug)]
pub enum ReadPortionsError {
    /// Used when the function cannot read file content.
    ReadFailed(io::Error),

    /// Used when the function cannot parse JSON structure.
    JsonParseFailed(serde_json::Error),
}

impl Display for ReadPortionsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ReadPortionsError::ReadFailed(err) => err.fmt(f),
            ReadPortionsError::JsonParseFailed(err) => err.fmt(f),
        }
    }
}

impl Error for ReadPortionsError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the portions are generated by `gen_portions()`.
    fn check_spaces(spaces: Vec<Vec<u8>>) {
        assert_eq!(spaces.len(), EMPTY_SPACES_COUNT);
        assert!(spaces.len() <= spaces.capacity());

        for space in spaces {
            assert_eq!(space.len(), 1);
            assert_eq!(space[0], b' ');
        }
    }

    /// Checks that the portions are taken from `files/test.json`.
    fn check_file(portions: Vec<Vec<u8>>) {
        assert_eq!(portions.len(), 4);
        assert!(portions.len() <= portions.capacity());

        assert_eq!(portions[0].as_slice(), b"abc def g");
        assert_eq!(portions[1].as_slice(), b"ghi kkl j");
        assert_eq!(portions[2].as_slice(), b"mno pqr e");
        assert_eq!(portions[3].as_slice(), b"stu vwx f");
    }

    /// The `get_options()` function must choose the right variants.
    #[test]
    fn gets_portions() {
        check_file(get_portions(Some("files/test.json")).expect("Failed to parse JSON"));
        check_spaces(get_portions::<&str>(None).expect("get_portions(none) failed"));
    }
}
