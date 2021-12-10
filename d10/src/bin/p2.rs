use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn line_invalid(line: &str)-> Vec<String> {
    let mut vals: Vec<&str> = Vec::new();
    let mut backs: HashMap<&str,&str> = HashMap::new();
    let mut result:Vec<String> = Vec::new();
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
                        return result;
                    }else{
                        vals.pop();
                    }
                },
                None => {continue;}
            }
        }
    }
    while vals.is_empty() == false {
        result.push(String::from(*backs.get(vals.pop().unwrap()).unwrap()));
    }
    result
}

fn main() {
    let mut bad_val= BufReader::new(File::open("input.txt").unwrap()).lines()
        .into_iter().map(|l| l.unwrap()).map(|l| line_invalid(&l))
        .filter(|list| list.is_empty()==false).map(|list|{
            let mut score: u64 = 0;
            for s in list {
                score*=5;
                if s == ")"{
                    score+=1;
                }else if s== "]"{
                    score+=2;
                }else if s== "}"{
                    score+=3;
                }else if s== ">"{
                    score+=4;
                }
            }
            score
        }).collect::<Vec<u64>>();
    bad_val.sort();
    println!("VAL: {}",bad_val.get(bad_val.len()/2).unwrap());
}
