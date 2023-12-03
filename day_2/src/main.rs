fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut v = Vec::new();
        //Red -> Green -> Blue
        let limits = [12,13,14];
        for line in lines {
            if let Ok(n) = line {
                println!(n.split(";"))
            }
        }
        let sum: i32 = v.iter().sum();
        println!("{}", sum);
    }
}

struct game {
    red: i32;
    green: i32;
    blue: i32;
}
/*impl game {
    fn add(&self) -> &self {

    }
}*/


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}