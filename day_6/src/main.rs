use std::fs;
use regex::Regex;

//Part 2
fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let lines = Regex::new(r"\r\n").unwrap().split(&contents).collect::<Vec<_>>();

    let time = lines[0].split_whitespace().collect::<Vec<_>>()[1..].to_vec().join("").parse::<f64>().unwrap_or(0.0);
    let distance = lines[1].split_whitespace().collect::<Vec<_>>()[1..].to_vec().join("").parse::<f64>().unwrap_or(0.0);
    println!("{:?}, {:?}", time, distance);

    let bound_low = (time - (time.powf(2.0) - 4.0*distance).sqrt())/2.0;
    let bound_high = (time + (time.powf(2.0) - 4.0*distance).sqrt())/2.0;
    println!("{bound_low}, {bound_high}");

    let b1 = bound_low.trunc()+1.0;
    let b2 = bound_high.trunc() -((bound_high.trunc()==bound_high) as i32 as f64);
    println!("{b1}-{b2}={}",b2-b1+1.0);
}

//part 1
/*
fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let lines = Regex::new(r"\r\n").unwrap().split(&contents).collect::<Vec<_>>();

    let times = lines[0].split_whitespace().map(|x| x.parse::<f32>().unwrap_or(0f32)).collect::<Vec<_>>()[1..].to_vec();
    let distances = lines[1].split_whitespace().map(|x| x.parse::<f32>().unwrap_or(0f32)).collect::<Vec<_>>()[1..].to_vec();
    println!("{:?}, {:?}", times, distances);

    let mut out: f32 = 1.0;
    for i in 0..times.len() {

        let bound_low = (times[i] - (times[i].powf(2.0) - 4.0*distances[i]).sqrt())/2.0;
        let bound_high = (times[i] + (times[i].powf(2.0) - 4.0*distances[i]).sqrt())/2.0;
        println!("{bound_low}, {bound_high}");

        let b1 = bound_low.trunc()+1.0;
        let b2 = bound_high.trunc() -((bound_high.trunc()==bound_high) as i32 as f32);
        out *= b2-b1+1.0;
    }  
    println!("{out}")
}*/