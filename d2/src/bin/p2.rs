use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct SubMarine{
    horizontal: i32,
    depth: i32,
    aim: i32
}

#[derive(Debug)]
enum Movement{
    Forward(i32),
    Up(i32),
    Down(i32),
    Nothing
}

fn main() {
    let input: Vec<Movement>=BufReader::new(File::open("input.txt").unwrap()).lines().into_iter()
        .map(|x| String::from(x.unwrap())).map(|x| {
            if x.contains("forward"){
                let value: Vec<&str> = x.split(" ").collect();
                let value = value[1].parse::<i32>().unwrap();
                Movement::Forward(value)
            }else if x.contains("down"){
                let value: Vec<&str> = x.split(" ").collect();
                let value = value[1].parse::<i32>().unwrap();
                Movement::Down(value)
            }else if x.contains("up"){
                let value: Vec<&str> = x.split(" ").collect();
                let value = value[1].parse::<i32>().unwrap();
                Movement::Up(value)
            }else{
                Movement::Nothing
            }
        }).collect();

    let mut submarine = SubMarine{ depth: 0, horizontal: 0, aim: 0};

    for x in input {
        match x {
            Movement::Down(x)=> {submarine.aim= submarine.aim+x;},
            Movement::Up(x)=> {submarine.aim= submarine.aim-x;},
            Movement::Forward(x)=> {
                submarine.horizontal = submarine.horizontal+x;
                submarine.depth = submarine.depth + (x*submarine.aim);
            },
            _ => continue
        }
    }

    println!("Submarine position: {:?}",submarine);
    let multiplication = submarine.depth * submarine.horizontal;
    println!("Multiplication is: {}",multiplication);
}
