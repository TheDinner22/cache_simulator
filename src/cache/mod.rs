use std::collections::HashMap;
use std::fs::read_to_string;

use crate::user_input::{self, UserInput};

fn hex_str_to_binary_str(hex_str: &str) -> String {
    hex_str
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'a' => "1010",
            'b' => "1011",
            'c' => "1100",
            'd' => "1101",
            'e' => "1110",
            'f' => "1111",
            _ => unreachable!("given {}", c)
        })
        .fold(String::new(), |acc, e| acc + e)
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    pub addy: String,
    pub last_access: std::time::Instant,
    pub birthday: std::time::Instant,
    pub num_accesses: u32,
}

#[derive(Debug)]
pub struct Cache<'a> {
    cache: HashMap<String, HashMap<String, Line>>,
    context: &'a UserInput,
}

impl<'a> Cache<'a> {
    pub fn new(user_input: &'a UserInput) -> Self {
        Cache {
            cache: HashMap::new(),
            context: user_input,
        }
    }

    fn contains(&self, binary_address: &String) -> bool {
        let (tag, set, _offset) = self.context.break_down_binary_address(binary_address);

        let set_found = self.cache.get(set).is_some();
        if !set_found {
            return false;
        }

        let line_found = self.cache.get(set).unwrap().get(tag).is_some();
        return line_found;
    }

    // set must be in cache or we panic
    fn empty_space(&self, set: &str) -> bool {
        let lines_in_set = self.cache.get(set).expect("set must exist").len() as u32;
        let lines_per_set = self.context.lines_per_set();
        assert!(lines_in_set <= lines_per_set);
        return lines_in_set < lines_per_set;
    }

    fn write(&mut self, binary_address: &String) -> bool {
        // break address down into set, tag, offset
        let (tag, set, _) = self.context.break_down_binary_address(binary_address);

        // does it already exist? if yes we are done
        if self.contains(binary_address) {
            // update last accessed feild
            self.cache.get_mut(set).unwrap().get_mut(tag).unwrap().last_access = std::time::Instant::now();
            self.cache.get_mut(set).unwrap().get_mut(tag).unwrap().num_accesses += 1;

            return true;
        }

        // does the set exist yet? If not create it as empty
        let set_already_exists = self.cache.get(set).is_some();
        if !set_already_exists {
            self.cache.insert(set.into(), HashMap::new());
            // sanity check (making sure we don't have too many sets)
            assert!(self.cache.len() <= 2usize.pow(set.len() as u32))
        }

        // is there an empty space? if so, Insert there
        if self.empty_space(set) {
            self.cache.get_mut(set).unwrap().insert(
                tag.into(),
                Line {
                    addy: binary_address.clone(),
                    last_access: std::time::Instant::now(),
                    birthday: std::time::Instant::now(),
                    num_accesses: 0,
                },
            );
            return false;
        }

        // determine which line to replace (which takes the replacement_policy into account)
        let tag_to_replace: String = match self.context.replacement_policy() {
            user_input::ReplacementPolicy::LRU => {
                let tag_of_line_to_replace = self
                    .cache
                    .get(set)
                    .unwrap()
                    .into_iter()
                    .min_by_key(|(_tag, line)| line.last_access)
                    .unwrap()
                    .0;
                tag_of_line_to_replace.into()
            }
            user_input::ReplacementPolicy::FIFO => {
                let tag_of_line_to_replace = self
                    .cache
                    .get(set)
                    .unwrap()
                    .into_iter()
                    .min_by_key(|(_tag, line)| line.birthday)
                    .unwrap()
                    .0;
                tag_of_line_to_replace.into()
            }
        };

        // make the replacement
        self.cache.get_mut(set).unwrap().remove(&tag_to_replace).expect("cant remove something that is not there");
        self.cache.get_mut(set).unwrap().insert(
            tag.into(),
            Line {
                addy: binary_address.into(),
                last_access: std::time::Instant::now(),
                birthday: std::time::Instant::now(),
                num_accesses: 0,
            },
        );
        return false;
    }

    pub fn simulate_trace_file(&mut self, filepath: &str) {
        let mut counter = 0;
        let mut hits = 0;

        for line in read_to_string(filepath)
            .expect(&format!("{} is not a valid file path", filepath))
            .lines()
        {
            let line = line.trim().to_lowercase();
            counter += 1; // just counting the number of lines in the file

            // get ls and address from the line
            let mut line_iter = line.split(' ').take(2);
            let ls = line_iter.next().unwrap();
            let hex_addy = line_iter
                .next()
                .unwrap()
                .strip_prefix("0x")
                .expect("hex addresses should start with 0x");
            let binary_addy = hex_str_to_binary_str(hex_addy);
            if ls == "l" || ls == "s"{
                if self.write(&binary_addy) {hits+=1;}
            } else {
                unreachable!("we should only get l or s. Got {}", ls);
            }
        }
        dbg!(&self.cache);
        dbg!(&counter);
        dbg!(&hits);
        dbg!(hits as f64 /counter as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_binary_address(){
        let hex_addy = "0x123456789ABCDeF";
        let actual = hex_str_to_binary_str(hex_addy.strip_prefix("0x").unwrap().to_lowercase().trim());
        let expected = "000100100011010001010110011110001001101010111100110111101111".to_string();
        assert_eq!(actual, expected);
    }
}
