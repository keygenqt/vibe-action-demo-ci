// SPDX-FileCopyrightText: Copyright 2025 keygenqt <email@keygenqt.com>
// SPDX-License-Identifier: MIT

/// Add two integers.
#[allow(dead_code)]
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }
}
