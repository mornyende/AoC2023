use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::String;
//use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> i32 {
    if let Ok(lines) = read_lines("./input.txt") {

        let mut current: i32 = 0;
        let mut total: i32 = 0;
        for line in lines {
            if let Ok(n) = line {
                current += 1;
                let t: Vec<&str> = n.split(": ").collect();
                let game: Vec<&str> = t[1].split(" | ").collect();
                let scratch: Vec<i32> = game[0].split(" ").map(|x| x.parse::<i32>().unwrap_or(0)).filter(|x| x != &0i32).collect();
                let winners: Vec<i32> = game[1].split(" ").map(|x| x.parse::<i32>().unwrap_or(0)).filter(|x| x != &0i32).collect();

                let mut sum = 0;
                let mut points = 0;
                for i in scratch {
                    if winners.contains(&i) {
                        if points==0 {points+=1;sum+=1;}
                        else {points = points*2;sum+=1;}
                       
                    }
                }

                total += points;
            }
        }
        return total;
    }
    else {return 0;}
}

fn part_2() -> i32 {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut map = HashMap::new();
        for i in 1..200 {
            map.insert(i,1);
        }
        

        let mut current: i32 = 0;
        let mut total: i32 = 0;
        for line in lines {
            if let Ok(n) = line {
                current += 1;
                let t: Vec<&str> = n.split(": ").collect();
                let game: Vec<&str> = t[1].split(" | ").collect();
                let scratch: Vec<i32> = game[0].split(" ").map(|x| x.parse::<i32>().unwrap_or(0)).filter(|x| x != &0i32).collect();
                let winners: Vec<i32> = game[1].split(" ").map(|x| x.parse::<i32>().unwrap_or(0)).filter(|x| x != &0i32).collect();

                let mut sum = 0;
                let mut points = 0;
                for i in scratch {
                    if winners.contains(&i) {
                        if points==0 {points+=1;sum+=1;}
                        else {points = points*2;sum+=1;}
                       
                    }
                }

                total += points*map.get(&current).unwrap_or(&0i32);

                for i in 1..(sum+1) {
                    map.insert(current+i, map.get(&(&current+&i)).unwrap_or(&0i32)+map.get(&current).unwrap_or(&0i32));                        
                }

            }
        }

        let mut sum = 0;
        for i in 1..200 {
            sum += map.get(&i).unwrap_or(&0i32);
        }
        return sum;
    }
    else {return 0;}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}