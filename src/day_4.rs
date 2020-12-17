use std::collections::HashMap;
use std::io::BufRead;

#[derive(PartialEq, Debug)]
pub struct Passport {
    // We don't really care about data types because
    // we only need to check whether values are "missing".
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    id: String,
    country_id: String,
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            birth_year: "".to_string(),
            issue_year: "".to_string(),
            expiration_year: "".to_string(),
            height: "".to_string(),
            hair_color: "".to_string(),
            eye_color: "".to_string(),
            id: "".to_string(),
            country_id: "".to_string(),
        }
    }

    pub fn from_map(map: &HashMap<String, String>) -> Passport {
        let mut passport = Passport::new();
        for (key, val) in map.iter() {
            match key {
                _ if key == "ecl" => passport.eye_color.push_str(val),
                _ if key == "pid" => passport.id.push_str(val),
                _ if key == "eyr" => passport.expiration_year.push_str(val),
                _ if key == "hgt" => passport.height.push_str(val),
                _ if key == "hcl" => passport.hair_color.push_str(val),
                _ if key == "byr" => passport.birth_year.push_str(val),
                _ if key == "iyr" => passport.issue_year.push_str(val),
                _ if key == "cid" => passport.country_id.push_str(val),
                _ => {}
            }
        }
        passport
    }

    // Reads a stream of passports into a vector of Passport.
    pub fn from_reader<R: BufRead>(reader: &mut R) -> Vec<Passport> {
        let mut passports: Vec<Passport> = Vec::new();
        loop {
            let mut field_map: HashMap<String, String> = HashMap::new();

            loop {
                // Allocate a buffer to read standard input into.
                let mut buffer = String::new();

                // Read entries from standard input.
                reader
                    .read_line(&mut buffer)
                    .expect("Expected to read data from reader!");

                let line = buffer.trim();
                if line.is_empty() {
                    break;
                }

                // Split input on spaces, and subsequently on colons.
                for fields in line.split(' ') {
                    let mut values = fields.split(':');
                    field_map.insert(
                        values.next().unwrap_or("").to_string(),
                        values.next().unwrap_or("").to_string(),
                    );
                }
            }

            if field_map.is_empty() {
                break;
            }

            passports.push(Passport::from_map(&field_map));
        }
        passports
    }

    /// Checks validity of a passport for exercise 1.
    pub fn is_valid_1(&self) -> bool {
        // A passport is valid if none of the fields, except country_id is undefined.
        // TODO (rikheijdens): would be nice if you could iterate over a struct in Rust :-)
        !self.birth_year.is_empty()
            && !self.issue_year.is_empty()
            && !self.expiration_year.is_empty()
            && !self.height.is_empty()
            && !self.hair_color.is_empty()
            && !self.eye_color.is_empty()
            && !self.id.is_empty()
    }

    /// Checks validty of a passport for exercise 2.
    pub fn is_valid_2(&self) -> bool {
        // Validate birth year.
        match self.birth_year.parse::<i32>() {
            Ok(_n @ 1920..=2002) => {}
            _ => {
                return false;
            }
        }

        // Validate issue year.
        match self.issue_year.parse::<i32>() {
            Ok(_n @ 2010..=2020) => {}
            _ => {
                return false;
            }
        }

        // Validate expiration year.
        match self.expiration_year.parse::<i32>() {
            Ok(_n @ 2020..=2030) => {}
            _ => {
                return false;
            }
        }

        // Validate height.
        if self.height.ends_with("cm") {
            let height = self.height.split("cm").next().unwrap();
            match height.parse::<i32>() {
                Ok(_n @ 150..=193) => {}
                _ => {
                    return false;
                }
            }
        } else if self.height.ends_with("in") {
            let height = self.height.split("in").next().unwrap();
            match height.parse::<i32>() {
                Ok(_n @ 59..=76) => {}
                _ => {
                    return false;
                }
            }
        } else {
            // Height did not validate.
            return false;
        }

        // Validate hair color
        if !self.hair_color.starts_with('#') || self.hair_color.len() != 7 {
            return false;
        }
        let hair_color = self.hair_color.trim_start_matches('#');
        match i64::from_str_radix(hair_color, 16) {
            Ok(_) => {}
            Err(_) => {
                return false;
            }
        }

        let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !valid_colors.contains(&&*self.eye_color) {
            return false;
        }

        // Validate passport id
        if self.id.len() != 9 {
            return false;
        }
        match self.id.parse::<u64>() {
            Ok(_n) => {}
            _ => {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_passport_validity_1() {
        let mut p = Passport {
            eye_color: "gry".to_string(),
            id: "860033327".to_string(),
            expiration_year: "2020".to_string(),
            hair_color: "#fffffd".to_string(),
            birth_year: "1937".to_string(),
            issue_year: "2017".to_string(),
            country_id: "147".to_string(),
            height: "183cm".to_string(),
        };
        assert!(p.is_valid_1());
        p.height = "".to_string();
        assert!(!p.is_valid_1())
    }

    #[test]
    fn test_passport_validity_2() {
        let mut p = Passport {
            eye_color: "grn".to_string(),
            id: "087499704".to_string(),
            expiration_year: "2030".to_string(),
            hair_color: "#623a2f".to_string(),
            birth_year: "1980".to_string(),
            issue_year: "2012".to_string(),
            country_id: "147".to_string(),
            height: "74in".to_string(),
        };

        assert!(p.is_valid_2());

        p.hair_color = "#abd".to_string();
        //p.id = "bla".to_string();
        assert!(!p.is_valid_2());
    }

    #[test]
    fn test_from_reader() {
        let passports = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let mut reader: BufReader<&[u8]> = BufReader::new(passports.as_bytes());
        let passports: Vec<Passport> = Passport::from_reader(&mut reader);
        assert_eq!(passports.len(), 4);
        let expected = Passport {
            eye_color: "gry".to_string(),
            id: "860033327".to_string(),
            expiration_year: "2020".to_string(),
            hair_color: "#fffffd".to_string(),
            birth_year: "1937".to_string(),
            issue_year: "2017".to_string(),
            country_id: "147".to_string(),
            height: "183cm".to_string(),
        };
        assert_eq!(passports[0], expected);
    }
}
