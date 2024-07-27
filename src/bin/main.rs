use cda_cache_sim::get_input::get_input;

#[derive(Debug, PartialEq, Eq)]
enum CacheType {
    FullyAssociative(u32),
    DirectMapped(u32),
    SetAssociative(u32), // u32 here is the n in n-way
                         // in other-words, it represents the set-size exponent
}

#[derive(Debug, PartialEq, Eq)]
enum ReplacementPolicy {
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
struct UserInput {
    cache_size_exp: u32,
    line_size_exp: u32,
    cache_type: CacheType,
    replacement_policy: ReplacementPolicy,
}

impl UserInput {
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
}

fn all_user_input() -> UserInput{
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

fn main() {
    println!("{:#?}", all_user_input())
}
