use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let positions: Vec<u32> =BufReader::new(File::open("input.txt").unwrap())
        .lines().flat_map(|l|{
            l.unwrap().split(",").map(|s| String::from(s)).collect::<Vec<String>>().clone()
        }).map(|d| d.parse::<u32>().unwrap()).collect();

    let mut sums_of_dists: Vec<u32> = Vec::new();

    for i in 0..positions.len(){
        let mut sum = 0;
        for j in 0..positions.len(){
            sum += (*positions.get(i).unwrap() as i32 - *positions.get(j).unwrap() as i32).abs() as u32;
        }
        sums_of_dists.push(sum);
    }

    sums_of_dists.sort();

    println!("Shortest total Distance {}",sums_of_dists.get(0).unwrap());
}
