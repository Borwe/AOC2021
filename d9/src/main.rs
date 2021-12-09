use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let data = BufReader::new(File::open("input.txt").unwrap())
        .lines().into_iter().map(|l| l.unwrap().chars().into_iter()
                 .map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut smalls :Vec<u32> = Vec::new();

    for i in 0..data.len(){
        let mut top: Option<&Vec<u32>> = None;
        if (i as i32 -1) >= 0 {
            top = data.get(i-1);
        }
        let current_line: Vec<u32> = data.get(i).unwrap().clone();
        let mut bottom: Option<&Vec<u32>> = None;
        if (i+1) <= data.len() {
            bottom = data.get(i+1);
        }
        for j in 0..current_line.len(){
            let current_item = current_line.get(j).unwrap();
            let up: i32 = match top {
                Some(x) => *x.get(j).unwrap() as i32,
                None => -1
            };
            let down: i32 = match bottom {
                Some(x) => *x.get(j).unwrap() as i32,
                None => -1
            };
            let mut right: i32 = -1;
            if (j+1)<current_line.len() {
                right=*current_line.get(j+1).unwrap() as i32;
            }
            let mut left: i32 = -1;
            if (j as i32 -1)>=0 {
                left=*current_line.get(j-1).unwrap() as i32;
            }

            if left !=-1 && right != -1 && up != -1 && down != -1 {
                if *current_item < left as u32 && *current_item < up as u32 && *current_item < down as u32 && *current_item < right as u32 {
                    smalls.push(*current_item);
                    continue;
                }
            }

            // no up
            if left !=-1 && right != -1 && up == -1 && down != -1 {
                if *current_item < left as u32 && *current_item < down as u32 && *current_item < right as u32 {
                    smalls.push(*current_item);
                    continue;
                }
            }

            // no down
            if left !=-1 && right != -1 && up != -1 && down == -1 {
                if *current_item < left as u32 && *current_item < up as u32 && *current_item < right as u32 {
                    smalls.push(*current_item);
                    continue;
                }
            }

            // no left
            if left ==-1 && right != -1 && up != -1 && down != -1 {
                if *current_item < up as u32 && *current_item < down as u32 && *current_item < right as u32 {
                    smalls.push(*current_item);
                    continue;
                }
            }

            // no right
            if left !=-1 && right == -1 && up != -1 && down != -1 {
                if *current_item < left as u32 && *current_item < up as u32 && *current_item < down as u32 {
                    smalls.push(*current_item);
                    continue;
                }
            }

            // no left and up
            if left ==-1 && right != -1 && up == -1 && down != -1 {
                if *current_item < down as u32 && *current_item < right as u32 {
                    smalls.push(*current_item);
                    continue;
                }
            }

            // no left and down
            if left ==-1 && right != -1 && up != -1 && down == -1 {
                if *current_item < up as u32 && *current_item < right as u32 {
                    smalls.push(*current_item);
                    continue;
                }
            }

            //no right up
            if left !=-1 && right == -1 && up == -1 && down != -1 {
                if *current_item < left as u32 && *current_item < down as u32 {
                    smalls.push(*current_item);
                    continue;
                }
            }

            //no right  down
            if left !=-1 && right == -1 && up != -1 && down == -1 {
                if *current_item < left as u32 && *current_item < up as u32{
                    smalls.push(*current_item);
                    continue;
                }
            }
        }
    }

    println!("SUM: {}",smalls.into_iter().map(|x| x+1).sum::<u32>());
}
