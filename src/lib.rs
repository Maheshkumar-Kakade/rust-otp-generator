use rand::Rng;

const DIGITS: &str = "0123456789";
const LOWER_CASE_ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER_CASE_ALPHABETS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SPECIAL_CHARS: &str = "#!&@";

// Errors
const ERR_ZERO_LENGTH: &str = "Length is 0.";
const ERR_ALL_FALSE_FLAGS: &str = "All flags are false. Need at least one option true.";

/// Flags to include or exclude characters in otp
#[derive(Default, PartialEq)]
pub struct Flags {
    pub digits: bool,
    pub lower_case_alphabets: bool,
    pub upper_case_alphabets: bool,
    pub special_chars: bool,
}


impl Flags {
    /// Include digits, alphabets and special characters
    pub fn new() -> Flags {
        Flags {
            digits: true,
            lower_case_alphabets: true,
            upper_case_alphabets: true,
            special_chars: true,
        }
    }
    fn allowed_chars(&self) -> String {
        let mut allowed_chars = String::from("");
        if self.digits {
            allowed_chars.push_str(DIGITS);
        };
        if self.lower_case_alphabets {
            allowed_chars.push_str(LOWER_CASE_ALPHABETS);
        };
        if self.upper_case_alphabets {
            allowed_chars.push_str(UPPER_CASE_ALPHABETS);
        };
        if self.special_chars {
            allowed_chars.push_str(SPECIAL_CHARS);
        };
        allowed_chars
    }
}

/// Returns one time password (OTP) as per given flags.
///
/// # Examples
///
/// ```
/// let flags = otp_generator::Flags {digits : true, ..Default::default()};
/// println!("6 digit Otp = {}", otp_generator::generate(6,&flags).unwrap());
/// 
/// ```
pub fn generate(length: usize, flags: &Flags) -> Result<String, &str> {
    if length == 0 {
        return Err(ERR_ZERO_LENGTH);
    }
    if *flags == Default::default() {
        return Err(ERR_ALL_FALSE_FLAGS);
    }
    let allows_chars = flags.allowed_chars();
    let mut password = String::from("");
    let allows_chars_length = allows_chars.len();
    for _ in 0..length {
        let char_index = rand::thread_rng().gen_range(0..allows_chars_length);
        password.push(allows_chars.chars().nth(char_index).unwrap());
    }
    Ok(password)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let option = Flags::new();
        let password = generate(10, &option).unwrap();
        dbg!("{}", &password);
        assert_eq!(password.len(), 10);
        assert!(password.chars().all(|x| option.allowed_chars().contains(x)));
    }

    #[test]
    fn it_works_only_digit() {
        let option = Flags {
            digits: true,
            ..Default::default()
        };
        let password = generate(6, &option).unwrap();
        dbg!("{}", &password);
        assert_eq!(password.len(), 6);
        assert!(password.chars().all(|x| DIGITS.contains(x)));
    }

    #[test]
    fn it_works_only_alphabets() {
        let option = Flags {
            lower_case_alphabets: true,
            upper_case_alphabets: true,
            ..Default::default()
        };
        let password = generate(10, &option).unwrap();
        dbg!("{}", &password);
        assert_eq!(password.len(), 10);
        assert!(password.chars().all(|x| LOWER_CASE_ALPHABETS.contains(x) || UPPER_CASE_ALPHABETS.contains(x) ));
    }

    #[test]
    fn it_works_for_only_special_chars() {
        let option = Flags {
            special_chars: true,
            ..Default::default()
        };
        let password = generate(5, &option).unwrap();
        dbg!("{}", &password);
        assert_eq!(password.len(), 5);
        assert!(password.chars().all(|x| SPECIAL_CHARS.contains(x) ));
    }

    #[test]
    fn it_return_error_on_zero_length() {
        let option = Flags {
            digits: true,
            ..Default::default()
        };
        let result = generate(0, &option);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(ERR_ZERO_LENGTH)); 
    }

    #[test]
    fn it_return_error_on_all_false_flags() {
        let option = Flags {
            ..Default::default()
        };
        let result = generate(10, &option);
        assert!(result.is_err()); 
    }

}
