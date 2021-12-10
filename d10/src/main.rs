use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn line_invalid(line: &str)-> String {
    let mut vals: Vec<&str> = Vec::new();
    let mut backs: HashMap<&str,&str> = HashMap::new();
    backs.insert("{","}");
    backs.insert("(",")");
    backs.insert("<",">");
    backs.insert("[","]");

    for i in 0..line.len(){
        let c = line.get(i..i+1).unwrap();
        if backs.contains_key(c) {
            //increment vals
            vals.push(c);
        }else{
            match vals.last() {
                Some(x) => {
                    let c2 = *backs.get(x).unwrap();
                    if c != c2{
                        return String::from(c);
                    }else{
                        vals.pop();
                    }
                },
                None => {continue;}
            }
        }
    }
    if line.len()%2 != 0 {
        return String::from("x");
    }
    String::from('x')
}

fn main() {
    let bad_val: u32 = BufReader::new(File::open("input.txt").unwrap()).lines()
        .into_iter().map(|l| l.unwrap()).map(|l| line_invalid(&l))
        .filter(|c| *c!=String::from('x')).map(|c| {
            if c == ")"{
                3
            }else if c == "]"{
                57
            }else if c == "}"{
                1197
            }else if c == ">"{
                25137
            }else{
                0
            }
        }).sum();

    println!("VAL: {:?}",bad_val);
}
