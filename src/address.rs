// Copyright (C) 2024, Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Debug)]
pub struct Address {
    pub hex: String,
    pub wall: u32,
    pub shelf: u32,
    pub volume: u32,
    pub page: u32,
}

impl Display for Address {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(
            formatter,
            "{}:{}:{}:{}:{}",
            self.hex, self.wall, self.shelf, self.volume, self.page
        )
    }
}
