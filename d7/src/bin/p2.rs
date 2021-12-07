use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let positions: Vec<u32> =BufReader::new(File::open("input.txt").unwrap())
        .lines().flat_map(|l|{
            l.unwrap().split(",").map(|s| String::from(s)).collect::<Vec<String>>().clone()
        }).map(|d| d.parse::<u32>().unwrap()).collect();

    let mut positions_ordered = positions.clone();
    positions_ordered.sort();

    let min = positions_ordered.get(0).unwrap();
    let max = positions_ordered.get(positions_ordered.len()-1).unwrap();
    println!("MIN:{} MAX:{}",min,max);

    let mut dists_sums: Vec<u32>=Vec::new();
    for i in *min..=*max{
        let mut sum_at_pos:u32 = 0;
        for j in 0..positions.len(){
            let mut moves = (*positions.get(j).unwrap() as i32 - i as i32).abs();
            let mut fuel_cost = 0;
            let mut fuel_used = 0;
            while moves > 0 {
                fuel_cost+=1;
                fuel_used+=fuel_cost;
                moves-=1;
            }

            sum_at_pos+=fuel_used;
        }
        dists_sums.push(sum_at_pos);
    }

    dists_sums.sort();
    println!("TOTAL SUM: {:?}",dists_sums.get(0).unwrap());
}
