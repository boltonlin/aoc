use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn find_santa(santas_path: &str) -> i32 {
    let mut floor_num = 0;
    for paren in santas_path.split("").filter(|c| *c != "") {
        if paren == "(" {
            floor_num += 1;
        } else {
            floor_num -= 1;
        }
    }
    floor_num
}

fn find_santa_nice(santas_path: &str) -> i32 {
    santas_path
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn find_basement_entry(santas_path: &str) -> i16 {
    let mut floor_num = 0;
    for (i, paren) in santas_path.chars().enumerate() {
        if paren == '(' {
            floor_num += 1;
        } else if paren == ')' {
            floor_num -= 1;
        }

        if floor_num == -1 {
            return (i + 1) as i16;
        }
    }
    -1
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut f = File::open(file_path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    println!("{}", find_santa_nice(buffer.as_str()));
    println!("{}", find_basement_entry(buffer.as_str()));

    Ok(())
}

#[test]
fn it_works() {
    let test1 = "(())";
    assert_eq!(find_santa(test1), 0);
}

#[test]
fn test_zero() {
    let test1 = "()()";
    assert_eq!(find_santa(test1), 0);
}

#[test]
fn test_three() {
    let test1 = "(((";
    let test2 = "(()(()(";
    assert_eq!(find_santa(test1), 3);
    assert_eq!(find_santa(test2), find_santa(test1));
}
