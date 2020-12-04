use std::fs;
use std::path::Path;

pub fn toboggan_trajectory_p1() -> Result<i32, ()> {
    let contents = fs::read_to_string(Path::new("./input/input_day_three_p1.txt")).unwrap();
    let array_row: Vec<&str> = contents.split_whitespace().collect();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for el in array_row.into_iter() {
        let mut temp_vec: Vec<char> = Vec::new();
        for il in el.chars() {
            temp_vec.push(il);
        }
        matrix.push(temp_vec);
    }
    let mut counter = 0;
    // println!("{}", &matrix.len());
    let mut start_pos = 0 as usize;
    for i in 0..*&matrix.len() as usize {
        // println!("{}", &matrix[i][3*i]);
        let row = &matrix[i].len();
        if &start_pos >= row {
            start_pos = start_pos % row;
        }
        if &matrix[i][start_pos] == &'#' {
            counter+=1;
        }
        start_pos+=3;
    }
    return Ok(counter);
}

pub fn toboggan_trajectory_p2() -> Result<i64, ()> {
    // index 1 is to the right index 0 is down
    let slopes: Vec<Vec<usize>> = vec![vec![1,1], vec![1,3], vec![1,5], vec![1, 7], vec![2,1]];
    let mut total: Vec<i64> = Vec::new();

    for slope in slopes {
        let contents = fs::read_to_string(Path::new("./input/input_day_three_p1.txt")).unwrap();
        let array_row: Vec<&str> = contents.split_whitespace().collect();
        let mut matrix: Vec<Vec<char>> = Vec::new();
        for el in array_row.into_iter() {
            let mut temp_vec: Vec<char> = Vec::new();
            for il in el.chars() {
                temp_vec.push(il);
            }
            matrix.push(temp_vec);
        }
        let mut counter: i64 = 0;
        // println!("{}", &matrix.len());
        let mut start_pos = 0 as usize;
        let mut start_altitude = 0 as usize;
        for i in 0..*&matrix.len() as usize {
            // println!("{}", &matrix[i][3*i]);
            let row = &matrix[i].len();
            if start_altitude >= *&matrix.len() {
                break;
            }
            if &start_pos >= row {
                start_pos = start_pos % row;
            }
            if &matrix[start_altitude][start_pos] == &'#' {
                counter+=1;
            }
            start_pos+=&slope[1];
            start_altitude+=&slope[0];
        }
        total.push(counter);

    }
    println!("{:?}", &total);
    let mut result: i64 = 1;
    for j in total.into_iter() {
        result= result*j;
    }
    return Ok(result)
}