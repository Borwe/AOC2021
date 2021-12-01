use std::fs::File;
use std::rc::Rc;
use std::io::{BufRead, BufReader};

fn main(){
    let file = File::open("input.txt").unwrap();
    let lines: Rc<Vec<i64>> = Rc::new(BufReader::new(file).lines()
        .into_iter().map(|x| String::from_iter(x).as_str()
                         .parse::<i64>().unwrap()).collect());
    let length = lines.len();

    let mut count = 0;
    let mut prev_sum:i64 = (&lines[0..3]).into_iter().sum();
    let mut items_left = length-1;

    for x in 1..=(length-1){
        if items_left >= 3 {
            let range_2 = &lines[x..(x+3)];
            let sum_range_2:i64 = range_2.into_iter().sum();
            if sum_range_2>prev_sum {
                count = count + 1;
            }
            prev_sum = sum_range_2;
        }else{
            break;
        }
        items_left= items_left - 1;
    }

    println!("ANSWER IS: {}",count);
}
