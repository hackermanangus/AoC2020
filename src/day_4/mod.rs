use regex::Regex;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn passport_processing_p1() -> Result<i32, ()> {
    let mut contents = fs::read_to_string(Path::new("./input/input_4.txt")).unwrap();
    contents.push_str("\n\n");
    let re = Regex::new(r#"(?sm)(?P<detail>(?:[a-z]+:\S+\s)*\n)"#).unwrap();
    let valid_passports = 0;
    let mut bag: Vec<HashMap<&str, &str>> = Vec::new();
    let mut long_string = String::new();


    for capture in re.captures_iter(&contents) {

        let details = &capture["detail"];
        if details == "\n" {
            long_string.push_str("*")
        }
        else {
            long_string.push_str(details);
        }
        //let temp_vec: Vec<&str> = details.split("\n").collect();

    }
    let temp_vec: Vec<&str> = long_string.split("*").collect();
    //println!("{:?}", temp_vec);
    for entry in temp_vec.into_iter() {
        let mut map:HashMap<&str, &str> = HashMap::new();
        let another_temp_vec = entry.split_whitespace().collect::<Vec<&str>>();
        //println!("{:?}", another_temp_vec);
        for pair in another_temp_vec {
            let last_temp_vec: Vec<&str> = pair.split(":").collect();
            map.insert(last_temp_vec[0], last_temp_vec[1]);
        }
        bag.push(map);
    }
    let mut max_count = bag.len() as i32;
    for passport in bag.into_iter() {
        let important_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl","ecl", "ecl", "pid"];
        for key in important_keys.into_iter() {
            if passport.contains_key(key) {
                continue;
            } else {
                max_count-=1;
                break
            }
        }
    }
    Ok(max_count)
}

pub fn passport_processing_p2() -> Result<i32, ()> {
    let mut contents = fs::read_to_string(Path::new("./input/input_4.txt")).unwrap();
    contents.push_str("\n\n");
    let re = Regex::new(r#"(?sm)(?P<detail>(?:[a-z]+:\S+\s)*\n)"#).unwrap();
    let valid_passports = 0;
    let mut bag: Vec<HashMap<&str, &str>> = Vec::new();
    let mut long_string = String::new();


    for capture in re.captures_iter(&contents) {

        let details = &capture["detail"];
        if details == "\n" {
            long_string.push_str("*")
        }
        else {
            long_string.push_str(details);
        }
        //let temp_vec: Vec<&str> = details.split("\n").collect();

    }
    let temp_vec: Vec<&str> = long_string.split("*").collect();
    //println!("{:?}", temp_vec);
    for entry in temp_vec.into_iter() {
        let mut map:HashMap<&str, &str> = HashMap::new();
        let another_temp_vec = entry.split_whitespace().collect::<Vec<&str>>();
        //println!("{:?}", another_temp_vec);
        for pair in another_temp_vec {
            let last_temp_vec: Vec<&str> = pair.split(":").collect();
            map.insert(last_temp_vec[0], last_temp_vec[1]);
        }
        bag.push(map);
    }
    let mut max_count = bag.len() as i32;
    for passport in bag.into_iter() {
        let important_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl","ecl", "ecl", "pid"];
        for key in important_keys.into_iter() {
            if passport.contains_key(key) {
                continue;
            } else {
                max_count-=1;
                break
            }
        }
    }
    Ok(max_count)
}