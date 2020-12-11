use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use regex::Regex;

pub fn passport_processing_p1() -> Result<i32, ()> {
    let mut contents = fs::read_to_string(Path::new("./input/input_4.txt")).unwrap();
    contents.push_str("\n\n");
    let re = Regex::new(r#"(?sm)(?P<detail>(?:[a-z]+:\S+\s)*\n)"#).unwrap();
    let mut bag: Vec<HashMap<&str, &str>> = Vec::new();
    let mut long_string = String::new();


    for capture in re.captures_iter(&contents) {
        let details = &capture["detail"];
        if details == "\n" {
            long_string.push_str("*")
        } else {
            long_string.push_str(details);
        }
        //let temp_vec: Vec<&str> = details.split("\n").collect();
    }
    let temp_vec: Vec<&str> = long_string.split("*").collect();
    //println!("{:?}", temp_vec);
    for entry in temp_vec.into_iter() {
        let mut map: HashMap<&str, &str> = HashMap::new();
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
        let important_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "ecl", "pid"];
        for key in important_keys.into_iter() {
            if passport.contains_key(key) {
                continue;
            } else {
                max_count -= 1;
                break;
            }
        }
    }
    Ok(max_count)
}

pub fn passport_processing_p2() -> Result<i32, ()> {
    let mut contents = fs::read_to_string(Path::new("./input/input_4.txt")).unwrap();
    contents.push_str("\n\n");
    let re = Regex::new(r#"(?sm)(?P<detail>(?:[a-z]+:\S+\s)*\n)"#).unwrap();
    let mut bag: Vec<HashMap<&str, &str>> = Vec::new();
    let mut long_string = String::new();


    for capture in re.captures_iter(&contents) {
        let details = &capture["detail"];
        if details == "\n" {
            long_string.push_str("*")
        } else {
            long_string.push_str(details);
        }
        //let temp_vec: Vec<&str> = details.split("\n").collect();
    }
    let temp_vec: Vec<&str> = long_string.split("*").collect();
    //println!("{:?}", temp_vec);
    for entry in temp_vec.into_iter() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        let another_temp_vec = entry.split_whitespace().collect::<Vec<&str>>();
        //println!("{:?}", another_temp_vec);
        for pair in another_temp_vec {
            let last_temp_vec: Vec<&str> = pair.split(":").collect();
            map.insert(last_temp_vec[0], last_temp_vec[1]);
        }
        bag.push(map);
    }
    println!("{:?}", &bag.len());
    let mut max_count = bag.len() as i32;
    for passport in bag.into_iter() {
        let ik = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        for i in ik.into_iter() {
            match i {
                "byr" => {
                    if let Some(T) = passport.get(i) {
                        let value = T.parse::<i32>().unwrap();
                        if value >= 1920 && value <= 2002 {
                            continue;
                        } else {
                            max_count -= 1;
                            break;
                        }
                    } else {
                        max_count -= 1;
                        break;
                    }
                }
                "iyr" => {
                    if let Some(T) = passport.get(i) {
                        let value = T.parse::<i32>().unwrap();
                        if value >= 2010 && value <= 2020 {
                            continue;
                        } else {
                            max_count -= 1;
                            break;
                        }
                    } else {
                        max_count -= 1;
                        break;
                    }
                }
                "eyr" => {
                    if let Some(T) = passport.get(i) {
                        let value = T.parse::<i32>().unwrap();
                        if value >= 2020 && value <= 2030 {
                            continue;
                        } else {
                            max_count -= 1;
                            break;
                        }
                    } else {
                        max_count -= 1;
                        break;
                    }
                }
                "hgt" => {
                    if let Some(T) = passport.get(i) {
                        let value = T.to_string();
                        //println!("{}", &value);
                        let rex = Regex::new(r#"(?P<size>[0-9]+)(?P<msm>[a-z]+)"#).unwrap();
                        for res in rex.captures_iter(&value) {
                            let size = &res["size"].parse::<i32>().unwrap();
                            let msm = &res["msm"];
                            if msm == "cm" {
                                if size >= &150 && size <= &193 {
                                    continue;
                                } else {
                                    max_count -= 1;
                                    break;
                                }
                            } else if msm == "in" {
                                if size >= &59 && size <= &76 {
                                    continue;
                                } else {
                                    max_count -= 1;
                                    break;
                                }
                            } else {
                                max_count -= 1;
                                break;
                            }
                        }
                    } else {
                        max_count -= 1;
                        break;
                    }
                }
                "hcl" => {
                    if let Some(T) = passport.get(i) {
                        let value = T.to_string();
                        let rex = Regex::new(r#"#[0-9,a-f]{6}"#).unwrap();
                        if rex.find(&value) != None {
                            //println!("{:?}", rex.find(&value));
                            continue;
                        } else {
                            max_count -= 1;
                            break;
                        }
                    } else {
                        max_count -= 1;
                        break;
                    }
                }
                "ecl" => {
                    if let Some(T) = passport.get(i) {
                        let arr = vec!["amb".to_string(), "blu".to_string(), "brn".to_string(), "gry".to_string(), "grn".to_string(), "hzl".to_string(), "oth".to_string()];
                        let value = T.to_string();
                        if arr.contains(&value) {
                            continue;
                        } else {
                            max_count -= 1;
                            break;
                        }
                    } else {
                        max_count -= 1;
                        break;
                    }
                }
                "pid" => {
                    if let Some(T) = passport.get(i)
                    {
                        let value = T.to_string();
                        //println!("{}", &value);
                        if value.len() == 9 as usize {
                            continue;
                        } else {
                            max_count -= 1;
                            break;
                        }
                    } else {
                        max_count -= 1;
                        break;
                    }
                }
                _ => {
                    max_count-=1;
                    break
                }
            }
        }
    }
// for (i, passport) in bag.into_iter().enumerate() {
//     println!("{:?}", &passport);
//     let ik = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
//     if passport.get(ik[0]) != None {
//         let value = passport.get(ik[0]).unwrap().parse::<i32>().unwrap();
//         if value >= 1920 && value <= 2020 {
//             continue;
//         } else {
//             max_count -= 1;
//         }
//     } else {
//         max_count -= 1;
//     }
//     if passport.get(ik[1]) != None {
//         let value = passport.get(ik[1]).unwrap().parse::<i32>().unwrap();
//         if value >= 2010 && value <= 2020 {
//             continue;
//         } else {
//             max_count -= 1;
//         }
//     } else {
//         max_count -= 1;
//     }
//     if passport.get(ik[2])  != None {
//         let value = passport.get(ik[2]).unwrap().parse::<i32>().unwrap();
//         if value >= 2020 && value <= 2030 {
//             continue;
//         } else {
//             max_count -= 1;
//         }
//     } else {
//         max_count -= 1;
//     }
//     if passport.get(ik[3])  != None {
//         let value = passport.get(ik[3]).unwrap().to_string();
//         let rex = Regex::new(r#"(?P<size>[0-9]+)(?P<msm>[a-z]+)"#).unwrap();
//         for res in rex.captures_iter(&value) {
//             let size = &res["size"].parse::<i32>().unwrap();
//             let msm = &res["size"];
//             if msm == "cm" {
//                 if size >= &150 && size <= &193 {
//                     continue;
//                 } else {
//                     max_count -= 1;
//                 }
//             } else if msm == "in" {
//                 if size >= &59 && size <= &76 {
//                     continue;
//                 } else {
//                     max_count -= 1;
//                 }
//             } else {
//                 max_count -= 1;
//             }
//         }
//     } else {
//         max_count -= 1;
//     }
//     if passport.get(ik[4]) != None {
//         let value = passport.get(ik[4]).unwrap().to_string();
//         let rex = Regex::new(r#"#[0-9,a-f]{6}"#).unwrap();
//         if rex.find(&value) != None {
//             continue
//         } else {
//             max_count -= 1;
//         }
//     } else {
//         max_count -= 1;
//     }
//     if passport.get(ik[5])  != None {
//         let arr = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
//         let value = passport.get(ik[5]).unwrap().to_string();
//         if arr.contains(&&***&&*&value) {
//             continue
//         } else {
//             max_count -= 1;
//         }
//     } else {
//         max_count -= 1;
//     }
//     if passport.get(ik[6])  != None {
//         let value = passport.get(ik[6]).unwrap().to_string();
//         println!("{}", &value);
//         if value.len() == 9 as usize {
//             continue
//         } else {
//             max_count -= 1;
//         }
//     } else {
//         max_count -= 1;
//     }
// }
//println!("{:?}", &bag);
    Ok(max_count)
}