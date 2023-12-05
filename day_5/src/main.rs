use std::fs;
use regex::Regex;
use std::collections::HashMap;
use Extend;

//part 2
fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let lines = Regex::new(r"\r\n\r\n").unwrap().split(&contents).collect::<Vec<_>>();
    let seeds = lines[0].split(" ").collect::<Vec<_>>();

    //let mut map = HashMap::new();
    //part 1
    let mut out = seeds.clone().iter().map(|x| x.parse::<i64>().unwrap_or(0i64)).collect::<Vec<_>>();

    //part 2
    /* 
    let mut outB = [];
    for i in 1..out.len() {
        if i % 2 == 1 {
            outB.append()
        }
    }*/

    for line in &lines[1..] {
        let mut outnew = out.clone();

        let split_again = Regex::new(r"\r\n").unwrap().split(line).collect::<Vec<_>>();
        let name = split_again.get(0).unwrap().to_string();;
        //map.insert(ind,split_again[1..].to_vec());
//seeds
        for seed_ind in 1..out.len() {//out[1..2].to_vec().iter().enumerate() {
            let seed = out.get(seed_ind).unwrap();
            println!("Name: {name}, Seed: {:?}",seed);
            for range in split_again[1..].to_vec() {
                let range_vec = range.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
                println!("Range: {:?}",range_vec);
                if (seed>=range_vec.get(1).unwrap()) && (*seed<=range_vec.get(1).unwrap()+range_vec.get(2).unwrap()) {
                    //let new = 
                    outnew[seed_ind] = range_vec.get(0).unwrap()-range_vec.get(1).unwrap()+seed;
                    println!("Seed match {:?}", outnew[seed_ind]);
                    break;
                } 
                else {println!("No match")}
                
            }
        }
        out = outnew.clone();
        println!("{:?}",out)
        
    }
    //println!("{:?}",map);
    
    println!("{:?}",out);
    //println!("{:?}",out.iter().skip(1).step_by(2).copied().collect::<Vec<_>>());//.iter().min().unwrap());
    println!("{}",out[1..].iter().min().unwrap());

}