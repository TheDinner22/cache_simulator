mod get_input;

use std::{collections::HashMap, usize};

use get_input::get_input;

#[derive(Debug, PartialEq, Eq)]
enum CacheType {
    FullyAssociative(u32),
    DirectMapped(u32),
    SetAssociative(u32), // u32 here is the n in n-way
                         // in other-words, it represents the number of lines per set
                         //
                         // 2^this number  is the number of lines per set
}

impl CacheType {
    fn set_size_exp(&self) -> u32 {
        match self {
            CacheType::FullyAssociative(x) => *x,
            CacheType::DirectMapped(x) => *x,
            CacheType::SetAssociative(x) => *x,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReplacementPolicy {
    LRU,
    FIFO,
}

impl From<String> for ReplacementPolicy {
    fn from(value: String) -> Self {
        if value.to_lowercase().trim() == "l" {
            ReplacementPolicy::LRU
        }
        else {
            ReplacementPolicy::FIFO
        }
    }
}

// contains all the config from the user in one place
#[derive(Debug)]
pub struct UserInput {
    cache_size_exp: u32,
    line_size_exp: u32,
    cache_type: CacheType,
    replacement_policy: ReplacementPolicy,
}

impl UserInput {
    pub fn replacement_policy(&self) -> ReplacementPolicy {
        self.replacement_policy
    }

    // how I feel when understanding lifetimes: https://en.wikipedia.org/wiki/God
    pub fn break_down_binary_address<'a>(&self, address: &'a String) -> (&'a str, &'a str, &'a str) {
        let number_of_tag_bits = self.tag_size() as usize;
        let number_of_set_bits = self.num_sets_exp() as usize;
        let number_of_offset_bits = self.line_size_exp as usize;
        let total_size = number_of_set_bits + number_of_tag_bits + number_of_offset_bits;
        
        // little sanity check
        assert_eq!(32, address.len());
        assert_eq!(32, total_size);

        let tag = &address[..number_of_tag_bits];
        let set = &address[number_of_tag_bits..number_of_tag_bits+number_of_set_bits];
        let offset = &address[number_of_tag_bits+number_of_set_bits..];

        return (tag, set, offset);
    }

    fn new(cache_size_exp: u32, line_size_exp: u32, cache_type: String, replacement_policy: String) -> Self {
        let cache_type: CacheType = match cache_type.to_lowercase().trim() {
            "fa" => CacheType::FullyAssociative(cache_size_exp - line_size_exp),
            "dm" => CacheType::DirectMapped(0),
            "sa" => {
                let msg = "Enter '1' for 2 lines per set, '2' for 4 lines per set, '3' for 8 lines per set, or '4' for 16 lines per set.";
                let cache_size_exp = get_input(msg, |s| match s.trim() {
                    "1" => Ok(()),
                    "2" => Ok(()),
                    "3" => Ok(()),
                    "4" => Ok(()),
                    _ => Err(format!("{} is not 1, 2, 3, or 4!", s))
                });

                CacheType::SetAssociative(cache_size_exp.parse().unwrap())
            },
            _ => unreachable!("should not happen ever")
        };
        
        UserInput { cache_size_exp, line_size_exp, cache_type, replacement_policy: replacement_policy.into() }
    }

    fn num_lines_exp(&self) -> u32 {
        self.cache_size_exp - self.line_size_exp
    }

    fn num_sets_exp(&self) -> u32 {
        self.num_lines_exp() - self.cache_type.set_size_exp()
    }

    fn num_sets(&self) -> u32 {
        2u32.pow(self.num_sets_exp())
    }

    fn tag_size(&self) -> u32 {
        32 - self.num_sets_exp() - self.line_size_exp
    }

    fn set_size(&self) -> u32 {
        self.num_sets_exp()
    }

    pub fn lines_per_set(&self) -> u32 {
        self.num_lines() / self.num_sets()
    }

    fn offset_size(&self) -> u32 {
        self.line_size_exp
    }

    fn num_lines(&self) -> u32 {
        2u32.pow(self.num_lines_exp())
    }
}

pub fn all_user_input() -> UserInput{
    println!("This is an awesome cache simulator. By Jospeh Goodman.");

    let msg = "Cache size is an exponent of 2.  E.g. if the exponent is 3, the cache is 2 to the 3, or 8 bytes\nEnter the exponent for the cache size:";
    let cache_size_exp = get_input(msg, |s| {
        s.parse::<u32>()
            .map_err(|e| e.to_string()) // if we get an error, make it a string
            .map(|_| ()) // if we get no error, return () instead of the result of parsing (we
                         // don't need the result)
    });

    let msg = "Line size is an exponent of 2.  E.g. if the exponent is 3, the cache is 2 to the 3, or 8 bytes\nEnter the exponent for the line size:";
    let line_size_exp = get_input(msg, |s| {
        s.parse::<u32>()
            .map_err(|e| e.to_string()) // if we get an error, make it a string
            .map(|_| ()) // if we get no error, return () instead of the result of parsing (we
                         // don't need the result)
    });

    let msg = "What is the replacement policy? L or l for LRU, anything else for FIFO";
    let replacement_policy = get_input(msg, |s| match s.to_lowercase().trim() {
        _ => Ok(()),
    });

    let msg = "Is this cache fully associative, direct mapped, or set associative?\n Enter FA, DM, or SA";
    let cache_type = get_input(msg, |s| match s.to_lowercase().trim() {
        "fa" => Ok(()),
        "dm" => Ok(()),
        "sa" => Ok(()),
        _ => Err(format!("{} is not fa, dm, or sa!", s)),
    });

    UserInput::new(cache_size_exp.parse().unwrap(), line_size_exp.parse().unwrap(), cache_type, replacement_policy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_binary_address_with_fa() {
        let test_input = UserInput::new(5, 2, "fa".into(), "l".into());
        // 0x1fffff50
        let binary_address = "00011111111111111111111101010000".to_string();
        let (tag, set, offset) = test_input.break_down_binary_address(&binary_address);
        assert_eq!(tag, "000111111111111111111111010100");
        assert_eq!(set, "");
        assert_eq!(offset, "00");
    }

    #[test]
    fn test_decompose_binary_address_with_dm() {
        let test_input = UserInput::new(5, 2, "dm".into(), "l".into());
        // 0x1fffff50
        let binary_address = "00011111111111111111111101010000".to_string();
        let (tag, set, offset) = test_input.break_down_binary_address(&binary_address);
        assert_eq!(tag, "000111111111111111111111010");
        assert_eq!(set, "100");
        assert_eq!(offset, "00");
    }
}
