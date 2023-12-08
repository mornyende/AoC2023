use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let lines = Regex::new(r"\r\n\r\n").unwrap().split(&contents).collect::<Vec<_>>();
    let directions = lines[0].chars().collect::<Vec<_>>();

    //println!("{:?}",directions);

    let map_lines = Regex::new(r"\r\n").unwrap().split(lines[1]).collect::<Vec<_>>();
    //println!("{:?}",map_lines);

    let mut locations = Vec::<_>::new();
    let mut map = HashMap::new();
    for i in map_lines {
        if &i[2..3] == "A" {locations.push(i[0..3].to_string())}
        map.insert(i[0..3].to_string(),[i[7..10].to_string(),i[12..15].to_string()]);
    }
    let p1 = part1(map.clone(),directions.clone());
    println!("{p1}");

    //Part 2
    let mut location_distances = Vec::<_>::new();

    for loc in locations {
        let mut location = loc;
        let mut i: i64 = 0;
        while location.ends_with('Z') == false {
            if directions[(i%directions.len() as i64) as usize] == 'R' {
                let m = map.get(&location).unwrap();
                location = m.get(1).unwrap().to_string();
                //println!("{:?}",location)
            }
            else {
                let m = map.get(&location).unwrap();
                location = m.get(0).unwrap().to_string();
                //println!("{:?}",location)
            }
            i += 1;
        }
        location_distances.push(i)
    }
    println!("{:?}",location_distances)
}

//Part 1
fn part1(map: HashMap<String, [String; 2]>, directions: Vec<char>) -> i64 {
    let mut i: i64 = 0;
    let mut location = String::from("AAA");
    while location != String::from("ZZZ") {
        if directions[(i%directions.len() as i64) as usize] == 'R' {
            let m = map.get(&location).unwrap();
            location = m.get(1).unwrap().to_string();
            //println!("{:?}",location)
        }
        else {
            let m = map.get(&location).unwrap();
            location = m.get(0).unwrap().to_string();
            //println!("{:?}",location)
        }
        i += 1;
    }
    i
}


/*
    while true {
        if directions[(i%directions.len() as i64) as usize] == 'R' {
            for loc in locations.iter_mut() {
                let m = map.get(loc).unwrap();
                *loc = m.get(1).unwrap().to_string();
                //println!("{:?}",loc)
            } 
        }
        else {
            for loc in locations.iter_mut() {
                let m = map.get(loc).unwrap();
                *loc = m.get(0).unwrap().to_string();
                //println!("{:?}",loc)
            }
        }
        i += 1;
        if locations.iter().all(|s| s.ends_with('Z')) {break}
    } */