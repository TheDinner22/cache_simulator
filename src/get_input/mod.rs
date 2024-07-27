use std::io;

// this function is an example implementation of the 
// generic function, F, in the get_input function
//
// you should use a clousre instead
//
// this function is not used at all for the simulator
//
// here for educational reasons only
fn is_number(input: &String) -> Result<(), String> {
    let can_parse_to_number = input.parse::<u32>().is_ok();
    if can_parse_to_number {
        return Ok(());
    }
    else {
        return Err(format!("when given: {}\nfailed to parse to u32", input));
    }
}

pub fn get_input<F>(prompt: &str, filter: F) -> String 
// the generic type F is a function that takes a reference to a string and returns a result
// If the result is Ok(()), the string we passed in matches the filter and we can return
// if the result is Err(String), the string returned as the Err varient should be a meaningful
// error message (it will be printed to stdout)
where F: Fn(&String) -> Result<(), String>
{
    loop {
        let mut line = String::new();

        println!("{}", prompt);

        // read one line from stdin
        let stdin = io::stdin();
        let read_result = stdin.read_line(&mut line);

        // remove whitespace
        line = line.trim().to_string();

        // reading one line from stdin should never error (if it did, crash)
        if let Err(e) = read_result { panic!("{}", e); }

        // check line against filter, if it passes we are done
        // if it doesn't pass we keep waiting for valid input (and print the error msg)
        match filter(&line) {
            Ok(_) => return line,
            Err(err_msg) => {
                println!("{}", err_msg);
                continue;
            },
        };

    }
}

