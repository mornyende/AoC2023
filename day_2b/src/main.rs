use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::string::String;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        //let mut v = Vec::new();
        //Red -> Green -> Blue
        let limits: [i32; 3] = [12,13,14];
        let re_game = Regex::new(r"Game \d+: ").unwrap();
        let mut sum: i32 = 0;

        for line in lines {
            if let Ok(game) = line {
                let prefix = re_game.find(&game).map(|m| m.as_str()).unwrap_or("");
                let game_f = game.strip_prefix(prefix);
                let t: Vec<&str> = game_f.unwrap_or(" ").split("; ").collect();
                let mut map = HashMap::new();
                map.insert("red", 0);
                map.insert("green", 0);
                map.insert("blue", 0);
                println!("{:?}",map);
                println!("{:?}",t);
                for cubes in t {
                    let set: Vec<&str> = cubes.split(", ").collect();
                    let test = create_map(set);

                    if map.get("red").unwrap() < test.get("red").unwrap_or(&0i32) {map.insert("red",*test.get("red").unwrap_or(&0i32));};
                    
                    if map.get("green").unwrap() < test.get("green").unwrap_or(&0i32) {map.insert("green",*test.get("green").unwrap_or(&0i32));};
                    
                    if map.get("blue").unwrap() < test.get("blue").unwrap_or(&0i32) {map.insert("blue",*test.get("blue").unwrap_or(&0i32));};
                    
                    println!("{:?}", test.get("red"));
                }
                println!("{:?}",map);
                sum += map.get("red").unwrap_or(&0i32)*map.get("blue").unwrap_or(&0i32)*map.get("green").unwrap_or(&0i32);
            }
        }
        println!("{}",sum);
    }
}

fn create_map(input: Vec<&str>) -> HashMap<String, i32> {
    let mut map = HashMap::new();

    for item in input {
        let parts: Vec<&str> = item.split_whitespace().collect();
        let value = parts[0].parse::<i32>().unwrap();
        let key = parts[1].to_string();

        map.insert(key, value);
    }

    map
}

/*struct Game {
    red: i32,
    green: i32,
    blue: i32,
}
impl game {
    fn add(&self) -> &self {

    }
}*/


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}