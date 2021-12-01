use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut old = -1;
    let mut count = 0;
    for line in reader.lines(){
        let new_line = line.unwrap().as_str().parse::<i64>().unwrap();
        if new_line>old {
            count = count + 1;
            println!("{} > {}",new_line,old);
        }
        old = new_line;
    }
    println!("Answer: {}",count-1);
}
