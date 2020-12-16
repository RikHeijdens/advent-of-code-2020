use std::convert::TryFrom;
use std::io;

use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct PasswordPolicy {
    minimum_occurences: i32,
    maximum_occurences: i32,
    value: char,
    password: String,
}

impl PasswordPolicy {
    pub fn is_valid_1(&self) -> bool {
        // Check if the password validates by counting the number of
        // occurrences of 'value' in 'password'.
        let mut count = 0;
        for char in self.password.chars() {
            if char == self.value {
                count += 1
            }
        }
        count >= self.minimum_occurences && count <= self.maximum_occurences
    }

    pub fn is_valid_2(&self) -> bool {
        let mut valid = false;
        for (index, char) in self.password.chars().enumerate() {
            let fixed_index = (index as i32) + 1;
            if fixed_index == self.minimum_occurences {
                valid = char == self.value
            }
            if fixed_index == self.maximum_occurences && char == self.value {
                valid = !valid
            }
        }
        valid
    }
}

impl TryFrom<&str> for PasswordPolicy {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref POLICY_PATTERN: Regex =
                Regex::new(r"^([0-9]+)-([0-9]+) ([a-z0-9]{1}): (.*)$").unwrap();
        }
        match POLICY_PATTERN.captures(&value) {
            None => Err("Input did not match the expected format."),
            Some(captures) => Ok(PasswordPolicy {
                minimum_occurences: captures[1].parse::<i32>().unwrap(),
                maximum_occurences: captures[2].parse::<i32>().unwrap(),
                value: captures[3].parse::<char>().unwrap(),
                password: captures[4].parse::<String>().unwrap(),
            }),
        }
    }
}

/// Reads password policies from standard input.
pub fn read_password_policies() -> Vec<PasswordPolicy> {
    let mut password_policies: Vec<PasswordPolicy> = Vec::new();

    loop {
        // Allocate a buffer to read standard input into.
        let mut buffer = String::new();

        // Read entries from standard input.
        io::stdin()
            .read_line(&mut buffer)
            .expect("Expected to read password policy from stdin.");

        if buffer.is_empty() {
            break;
        }

        let policy = PasswordPolicy::try_from(buffer.trim()).expect(&*format!(
            "Error: '{}' is not a valid password policy.",
            buffer
        ));
        password_policies.push(policy);
    }

    password_policies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from() {
        let input = "1-3 a: abcde";
        let parsed_policy = PasswordPolicy::try_from(input).expect("Expected policy to validate.");
        let policy = PasswordPolicy {
            minimum_occurences: 1,
            maximum_occurences: 3,
            value: 'a',
            password: "abcde".to_string(),
        };
        assert_eq!(parsed_policy, policy);
    }

    #[test]
    fn test_is_valid_1() {
        let policy = PasswordPolicy {
            minimum_occurences: 1,
            maximum_occurences: 3,
            value: 'a',
            password: "abcde".to_string(),
        };
        assert_eq!(policy.is_valid_1(), true);

        let policy = PasswordPolicy {
            minimum_occurences: 1,
            maximum_occurences: 4,
            value: 'c',
            password: "abde".to_string(),
        };
        assert_eq!(policy.is_valid_1(), false);
    }

    #[test]
    fn test_is_valid_2() {
        let policy = PasswordPolicy {
            minimum_occurences: 1,
            maximum_occurences: 3,
            value: 'a',
            password: "abcde".to_string(),
        };
        assert_eq!(policy.is_valid_2(), true);

        let policy = PasswordPolicy {
            minimum_occurences: 1,
            maximum_occurences: 4,
            value: 'c',
            password: "cabcde".to_string(),
        };
        // both index 1, and index 4 are "c" which is invalid
        assert_eq!(policy.is_valid_2(), false);

        let policy = PasswordPolicy {
            minimum_occurences: 1,
            maximum_occurences: 4,
            value: 'c',
            password: "ca".to_string(),
        };
        assert_eq!(policy.is_valid_2(), true);
    }
}
