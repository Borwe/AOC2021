use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct BinCount{
    zero: u32,
    one: u32
}

fn create_epsilon(gamma: Vec<u32>)-> Vec<u32>{
    let mut epsilon: Vec<u32> = Vec::new();
    for x in gamma{
        if x==0{
            epsilon.push(1);
        }else{
            epsilon.push(0);
        }
    }
    epsilon
}

fn turn_to_binary(collection: Vec<u32>)-> u32{
    let len= collection.len()-1 ;
    let mut return_val: u32 = 0;
    let base: u32 = 2;
    for (pos, val) in collection.into_iter().enumerate(){
        return_val = return_val + (val * (base.pow((len as u32)-(pos as u32))));
    }
    return_val
}

fn power_consumption(gamma: Vec<u32>, epsilon: Vec<u32>)-> u32{
    let gamma_val = turn_to_binary(gamma);
    let epsilon_val = turn_to_binary(epsilon);
    gamma_val * epsilon_val
}

fn main() {
    let input: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
        .lines().into_iter().map(|x| String::from(x.unwrap())).collect();

    let mut gamma: Vec<u32> = Vec::new();
    let width = (&input[0]).len()-1;

    for x in 0..=width {
        let mut counter = BinCount{ zero:0, one:0};
        for y in input.clone() {
            let val = y.get(x..x+1).unwrap().parse::<u32>().unwrap();
            if val == 1 {
                counter.one = counter.one+1;
            }else {
                counter.zero = counter.zero+1;
            }
        }
        //fill gama
        if counter.zero > counter.one {
            gamma.push(0);
        }else{
            gamma.push(1);
        }
    }

    let epsilon = create_epsilon(gamma.clone());
    println!("GAMMA: {:?}  EPSILON: {:?}",gamma,epsilon);
    println!("Power Consumption: {}", power_consumption(gamma,epsilon));
}
