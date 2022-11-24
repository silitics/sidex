use std::str::FromStr;

use thiserror::Error;

fn iter_chars_boundary(input: &str) -> impl '_ + Iterator<Item = (char, bool)> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum State {
        Initial,
        Number,
        Lowercase,
        Uppercase,
        Splitter,
    }

    let mut state = State::Initial;
    input.chars().map(move |c| {
        let (next, boundary) = if c.is_alphabetic() {
            if c.is_lowercase() {
                (
                    State::Lowercase,
                    !matches!(state, State::Lowercase | State::Uppercase | State::Initial),
                )
            } else {
                (
                    State::Uppercase,
                    !matches!(state, State::Uppercase | State::Initial),
                )
            }
        } else if c.is_numeric() {
            (
                State::Number,
                !matches!(state, State::Number | State::Initial),
            )
        } else {
            (State::Splitter, false)
        };
        state = next;
        (c, boundary)
    })
}

/// Converts the *input* string to lowercase.
pub fn to_lowercase(input: &str) -> String {
    input.to_lowercase()
}

/// Converts the *input* string to uppercase.
pub fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

/// Converts the *input* string to mixed case.
fn to_mixed_case(input: &str, capitalize_first: bool) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize = capitalize_first;
    for (c, split) in iter_chars_boundary(input) {
        if c.is_alphanumeric() {
            if capitalize || split {
                result.extend(c.to_uppercase());
                capitalize = false;
            } else {
                result.extend(c.to_lowercase());
            }
        }
    }
    result
}

/// Converts the *input* string to `PascalCase`.
pub fn to_pascal_case(input: &str) -> String {
    to_mixed_case(input, true)
}

/// Converts the *input* string to `camelCase`.
pub fn to_camel_case(input: &str) -> String {
    to_mixed_case(input, false)
}

/// Converts the *input* string to a separated string.
fn to_separated_case(input: &str, separator: char, uppercase: bool) -> String {
    let mut result = String::with_capacity(2 * input.len());
    for (c, split) in iter_chars_boundary(input) {
        if split {
            result.push(separator);
        }
        if c.is_alphanumeric() {
            if uppercase {
                result.extend(c.to_uppercase());
            } else {
                result.extend(c.to_lowercase());
            }
        }
    }
    result
}

/// Converts the *input* string to `snake_case`.
pub fn to_snake_case(input: &str) -> String {
    to_separated_case(input, '_', false)
}

/// Converts the *input* string to `SCREAMING_SNAKE_CASE`.
pub fn to_screaming_snake_case(input: &str) -> String {
    to_separated_case(input, '_', true)
}

/// Converts the *input* string to `kebab-case`.
pub fn to_kebab_case(input: &str) -> String {
    to_separated_case(input, '-', false)
}

/// Converts the *input* string to `SCREAMING_KEBAB_CASE`.
pub fn to_screaming_kebab_case(input: &str) -> String {
    to_separated_case(input, '-', true)
}

#[derive(Debug, Error)]
#[error("Invalid rename function error!")]
pub struct InvalidRenameFunctionError;

#[derive(Debug, Clone)]
pub enum RenameFunction {
    None,
    CamelCase,
    PascalCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

impl RenameFunction {
    pub fn apply_to(&self, input: &str) -> String {
        match self {
            RenameFunction::None => input.to_owned(),
            RenameFunction::CamelCase => to_camel_case(input),
            RenameFunction::PascalCase => to_pascal_case(input),
            RenameFunction::SnakeCase => to_snake_case(input),
            RenameFunction::ScreamingSnakeCase => to_screaming_snake_case(input),
            RenameFunction::KebabCase => to_kebab_case(input),
            RenameFunction::ScreamingKebabCase => to_screaming_kebab_case(input),
        }
    }
}

impl FromStr for RenameFunction {
    type Err = InvalidRenameFunctionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "camelCase" => Ok(Self::CamelCase),
            "PascalCase" => Ok(Self::PascalCase),
            "snake_case" => Ok(Self::SnakeCase),
            "SCREAMING_SNAKE_CASE" => Ok(Self::ScreamingSnakeCase),
            "kebab-case" => Ok(Self::KebabCase),
            "SCREAMING-KEBAB-CASE" => Ok(Self::ScreamingKebabCase),
            _ => Err(InvalidRenameFunctionError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename() {
        for test_case in [
            [
                "TestInput",
                "testInput",
                "test_input",
                "TEST_INPUT",
                "test-input",
                "TEST-INPUT",
            ],
            [
                "Test3Abc",
                "test3Abc",
                "test_3_abc",
                "TEST_3_ABC",
                "test-3-abc",
                "TEST-3-ABC",
            ],
            [
                "Test423Abc",
                "test423Abc",
                "test_423_abc",
                "TEST_423_ABC",
                "test-423-abc",
                "TEST-423-ABC",
            ],
        ] {
            for input in test_case {
                assert_eq!(to_pascal_case(input), test_case[0]);
                assert_eq!(to_camel_case(input), test_case[1]);
                assert_eq!(to_snake_case(input), test_case[2]);
                assert_eq!(to_screaming_snake_case(input), test_case[3]);
                assert_eq!(to_kebab_case(input), test_case[4]);
                assert_eq!(to_screaming_kebab_case(input), test_case[5]);
            }
        }
    }
}
