use std::env;
use std::fs;
use regex::Regex;

//part 2
fn main() {
    let mut contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let symbol = b'*';
    let split_str = contents.as_bytes();

    let indices = split_str
    .iter()
    .enumerate()
    .filter(|(_, r)| **r==symbol)//(!r.is_ascii_digit()) && (r != &&b'.'))
    .map(|(index, _)| index)
    .collect::<Vec<_>>();

    let re = Regex::new(r"[0-9]+").unwrap();
    //let matches = re.find_iter(&contents)
    //println!("{:?}", matches)
    
    let mut sum: i64 = 0;

    let mut i = 0;
    for index in indices {
        let matches = re.find_iter(&contents);
        //if i==200 {break;}
        let mut yes = 0;
        let mut prod = 1;
        println!("New: {:?}",index);
        for mat in matches {
            if ((index+142 <= mat.end()) && (index+142 >= mat.start())) ||
            ((index-142 <= mat.end()) && (index-142 >= mat.start())) ||
            ((index+143 <= mat.end()) && (index+143 >= mat.start())) ||
            ((index-143 <= mat.end()) && (index-143 >= mat.start())) ||
            ((index+141 <= mat.end()) && (index+141 >= mat.start())) ||
            ((index-141 <= mat.end()) && (index-141 >= mat.start())) ||
            ((index+1 <= mat.end()) && (index+1 >= mat.start())) ||
            ((index-1 <= mat.end()) && (index-1 >= mat.start())) 
            //(index + 1 == mat.start()) || (index - 1 == mat.end()) 
             {
                print!("Matched: {:?} at indices ({}, {})  ", mat.as_str(), mat.start(), mat.end());
                println!("YAY {:?}",index);yes+=1;prod=prod*mat.as_str().parse().unwrap_or(1)}
        }
        if yes == 2 {sum+=prod;println!("ADDED: SUM={sum}");}
        i += 1;
    }

    println!("{sum}");

}

//part 1
/*fn main() {
    let mut contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    //contents = contents.replace("\r", "");
    //contents = contents.replace("\n", "");

    let symbols = "/*@%$+=-#&".as_bytes();
    let split_str = contents.as_bytes();
    //println!("{:?}",symbols.as_bytes());
    //println!("{:?}",".".bytes());
    //println!("{:?}","0123456789".bytes());
    //println!("With text:\n{:?}",contents.chars());
    let indices = split_str
    .iter()
    .enumerate()
    .filter(|(_, r)| symbols.contains(r))//(!r.is_ascii_digit()) && (r != &&b'.'))
    .map(|(index, _)| index)
    .collect::<Vec<_>>();

    //println!("{:?}",indices);
    //let ind = &indices.iter().map(|value| *value + 1).collect::<Vec<_>>();
    //println!("{:?}",ind);
    let mut comparison_indices =  &mut indices.iter().map(|value| *value + 1).collect::<Vec<_>>();
    comparison_indices.extend(&indices.iter().map(|value| *value - 1).collect::<Vec<_>>());
    comparison_indices.extend(&indices.iter().map(|value| *value + 141).collect::<Vec<_>>());
    comparison_indices.extend(&indices.iter().map(|value| *value - 141).collect::<Vec<_>>());
    comparison_indices.extend(&indices.iter().map(|value| *value + 142).collect::<Vec<_>>());
    comparison_indices.extend(&indices.iter().map(|value| *value - 142).collect::<Vec<_>>());
    comparison_indices.extend(&indices.iter().map(|value| *value + 143).collect::<Vec<_>>());
    comparison_indices.extend(&indices.iter().map(|value| *value - 143).collect::<Vec<_>>());

    //let number_indices: <Vec<_>> = [];

    let re = Regex::new(r"[0-9]+").unwrap();
    let matches = re.find_iter(&contents);
    //println!("{:?}", matches)
    
    let mut sum: i32 = 0;

    let mut i = 0;
    for mat in matches {
        //if i==200 {break;}
        let mut yes = 0;
        for place in mat.start()..mat.end() {
            if comparison_indices.contains(&place) {
                print!("Matched: {:?} at indices ({}, {})  ", mat.as_str(), mat.start(), mat.end());
                println!("YAY {:?}",place);yes=1;}
        }
        if yes == 1 {sum+= mat.as_str().parse().unwrap_or(0);}
        i += 1;
        //if mat.start()
        //println!("Matched: {:?} at indices ({}, {})", mat.as_str(), mat.start(), mat.end());
    }

    println!("{sum}");
    /* 
    for index in indices {
        /*if split_str.iter().nth(index+141).unwrap_or(&0u8).is_ascii_digit() ||
        split_str.iter().nth(index+140).unwrap_or(&0u8).is_ascii_digit() ||
        split_str.iter().nth(index+1).unwrap_or(&0u8).is_ascii_digit() ||
        split_str.iter().nth(index-1).unwrap_or(&0u8).is_ascii_digit() {print!("YES")}*/
        print!("{:?}  ",split_str.iter().nth(index));
        println!("{:?}",split_str.iter().nth(index-140));
        //println!("{:?}",split_str.iter().nth(index+142));
        //print!("{:?}",split_str.iter().nth(index+1));
        //print!("{:?}",split_str.iter().nth(index-1));
    }*/

}*/*/