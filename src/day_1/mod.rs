use std::fs;
use std::fmt::Error;
use std::path::Path;

pub fn report_repair_p1() -> Result<i32, ()> {
    let contents = fs::read_to_string(Path::new("./input/input_1.txt")).unwrap();
    let array: Vec<&str> = contents.split_whitespace().collect();

    let int_array: Vec<i32> = array.into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for i in &int_array {
        for j in &int_array {
            if i+j == 2020 {
                return Ok(i*j);
            }
        }
    }
    Err(())
}
pub fn report_repair_p2() -> Result<i32, ()> {
    let contents = fs::read_to_string(Path::new("./input/input_1.txt")).unwrap();
    let array: Vec<&str> = contents.split_whitespace().collect();

    let int_array: Vec<i32> = array.into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for i in &int_array {
        for j in &int_array {
            for k in &int_array {
                if i+j+k == 2020 {
                    return Ok(i*j*k)
                }
            }
        }
    }
    Err(())
}