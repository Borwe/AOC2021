use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    let outputs = BufReader::new(File::open("input.txt").unwrap())
        .lines().into_iter()
        .map(|x| String::from(x.unwrap().split("|").into_iter().last().unwrap()))
        .flat_map(|x| x.split(" ").map(|y| String::from(y)).collect::<Vec<String>>())
        .filter(|x| x.len()>0)
        .filter(|x| {
            let mut contains_values: HashSet<String>= HashSet::new();
            let mut count_uniques = 0;
            for i in 0..x.len(){
                if contains_values.insert(String::from(x.get(i..i+1).unwrap())) == true {
                    count_uniques+=1;
                }
            }

            if count_uniques==2 || count_uniques==4 || count_uniques==3 || count_uniques==7 {
                true
            }else{
                false
            }
        })
        .collect::<Vec<String>>();
    println!("OUTPUTS: {:?}",outputs.len());
}
