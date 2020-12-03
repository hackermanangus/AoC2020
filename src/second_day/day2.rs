use std::fmt::Error;
use std::fs;
use std::path::Path;
use regex::Regex;

pub fn psw_philosophy_p1() -> Result<i32, ()> {

    let mut counter = 0;
    // Unwrap because it should never go wrong
    let contents = fs::read_to_string(Path::new("./input/input_day_two_p1.txt")).unwrap();

    let re = Regex::new(r#"(?sm)(?P<low_freq>[0-9]+)-(?P<high_freq>[0-9]+) (?P<letter>[a-z]): (?P<pws>[a-z]+)"#).unwrap();

    for result in re.captures_iter(&contents) {
        let mut temp_inc = 0;
        let low_freq = &result["low_freq"].parse::<i32>().unwrap();
        let high_freq = &result["high_freq"].parse::<i32>().unwrap();

        let letter = &result["letter"].parse::<char>().unwrap();
        let pws = &result["pws"];
        println!("{} {} {} {}", &low_freq, &high_freq, &letter, &pws);
        for char in pws.chars() {
            if &char == letter {
                temp_inc+=1;
            }
        }
        if low_freq <= &temp_inc && &temp_inc <= high_freq {
            counter+=1;
        }
    }

    return Ok(counter)
}

//noinspection ALL,DuplicatedCode
pub fn psw_philosophy_p2() -> Result<u32, ()> {

    let mut counter = 0;
    // Unwrap because it should never go wrong
    let contents = fs::read_to_string(Path::new("./input/input_day_two_p1.txt")).unwrap();

    let re = Regex::new(r#"(?sm)(?P<low_freq>[0-9]+)-(?P<high_freq>[0-9]+) (?P<letter>[a-z]): (?P<pws>[a-z]+)"#).unwrap();

    for result in re.captures_iter(&contents) {
        let low_freq = &result["low_freq"].parse::<usize>().unwrap();
        let high_freq = &result["high_freq"].parse::<usize>().unwrap();

        let letter = &result["letter"].parse::<char>().unwrap();
        let pws = &result["pws"];
        let char_arr: Vec<char> = pws.chars().collect();
        if &char_arr[low_freq-1] == letter && &char_arr[high_freq-1] != letter {
            println!("{} {} {} {}", &low_freq, &high_freq, &letter, &pws);
            counter+=1;
        }
        else if &char_arr[low_freq-1] != letter && &char_arr[high_freq-1] == letter {
            println!("{} {} {} {}", &low_freq, &high_freq, &letter, &pws);
            counter+=1;
        }
        else {
        }
    }


    return Ok(counter)
}