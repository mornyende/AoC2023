use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::String;
use regex::Regex;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut v = Vec::new();
        
        for line in lines {
            if let Ok(n) = line {
                let first = str_to_digit(first(n.clone()));
                let last = str_to_digit(last(n));
                //println!("{}, {last}",first);
                v.push(first*10+last)
            }
        }
        let sum: i32 = v.iter().sum();
        println!("{}", sum);
    }
}

//First number
fn first(s: String) -> String {
    let list: [&str; 20] = ["0","zero","1","one","2","two", "3","three",
    "4","four","5","five","6","six","7","seven","8","eight","9","nine"];

    let l = s.len();
    for i in 0..l {
        for el in list.iter() {
            if i+el.len()<=l {
                if s[i..i+el.len()]==el.to_string() {return el.to_string()};
            }
        }
    }
    return "".to_string()
}

//Last number
fn last(s: String) -> String {
    let list: [&str; 20] = ["0","zero","1","one","2","two", "3","three",
    "4","four","5","five","6","six","7","seven","8","eight","9","nine"];

    let l = s.len();
    for i in 0..l {
        for el in list.iter() {
            if i+el.len()<=l {
                if s[l-i-el.len()..l-i]==el.to_string() {return el.to_string()};
            }
        }
    }
    return "".to_string()
}

//String to digit
fn str_to_digit(s: String) -> i32 {
    let mut out: String = s;
    let number_list = ["zero","one","two","three","four","five","six","seven","eight","nine"];
    for (i, e) in number_list.iter().enumerate() {
        let re = Regex::new(e).unwrap();
        out = re.replace(&out, i.to_string()).to_string();
    }
    let num: i32 = match out.parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    return num
}

//Get input
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


/*use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::String;
use regex::Regex;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        let mut v = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(n) = line {
                let temp: i32 = get_numbers(str_to_digit(n));
                println!("{}",temp);
                v.push(temp)
            }
        }
        let sum: i32 = v.iter().sum();
        println!("{}", sum);
    }
}

fn get_numbers(s:String) -> i32 {
    let mut out = String::from("");
    for c in s.chars() {
        if c.is_digit(10) {
            out.push(c);
            break;
        }
    }
    for c in s.chars().rev() {
        if c.is_digit(10) {
            out.push(c);
            break;
        }
    }
    let out2: i32 = match out.parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    return out2
}

fn str_to_digit(s: String) -> String {
    //let list = s.split_inclusive(&['1','2','3','4','5','6','7','8','9','0'][..]);
    //println!("{:?}",list);
    let mut out: String = s;
    let re = Regex::new("two").unwrap();
    out = re.replace_all(&out, "2").to_string();
    let re = Regex::new("nine").unwrap();
    out = re.replace_all(&out, "9").to_string();
    let re = Regex::new("eight").unwrap();
    out = re.replace_all(&out, "8").to_string();
    let number_list = ["zero","one","two","three","four","five","six","seven","eight","nine"];
    for (i, e) in number_list.iter().enumerate() {
        let re = Regex::new(e).unwrap();
        out = re.replace_all(&out, i.to_string()).to_string();
    }
    return out
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}*/