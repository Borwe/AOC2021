use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::cell::RefCell;

#[derive(Debug,PartialEq, Eq, Hash)]
struct Point {
    i: i32,
    j: i32
}

fn traverse_from_sink_deep(loc: Point, data: &Vec<Vec<u32>>,points_passed: &RefCell<Vec<Point>>){
    if loc.i < 0 || loc.i >= data.len() as i32 || loc.j < 0 || loc.j > data.get(loc.i as usize).unwrap().len() as i32 {
        return;
    }
    let item: i32 = match data.get(loc.i as usize) {
        Some(l) => match l.get(loc.j as usize) {
            Some (x) =>  *x as i32,
            None => -1
        },
        None => -1
    };
    if item == -1 || item == 9 {
        return;
    }
    points_passed.borrow_mut().push(Point{
        i:loc.i,
        j: loc.j
    });
    // go right
    let right = Point{
        i:loc.i,
        j:loc.j+1
    };
    if points_passed.borrow().contains(&right) == false {
        traverse_from_sink_deep(right,data,points_passed);
    }
    // go left
    let left = Point{
        i:loc.i,
        j:loc.j-1
    };
    if points_passed.borrow().contains(&left) == false {
        traverse_from_sink_deep(left,data,points_passed);
    }
    // go up
    let up = Point{
        i:loc.i-1,
        j:loc.j
    };
    if points_passed.borrow().contains(&up) == false {
        traverse_from_sink_deep(up,data,points_passed);
    }
    // go down
    let down = Point{
        i:loc.i+1,
        j:loc.j
    };
    if points_passed.borrow().contains(&down) == false {
        traverse_from_sink_deep(down,data,points_passed);
    }
}

fn get_basin_size(i: usize, j: usize, data: &Vec<Vec<u32>>)-> u32{
    let points_vec: RefCell<Vec<Point>> = RefCell::from(Vec::new());
    traverse_from_sink_deep(Point{i:i as i32,j:j as i32},data,&points_vec);
    points_vec.into_inner().into_iter().collect::<HashSet<Point>>()
        .len() as u32
}

fn main() {
    let data = BufReader::new(File::open("input.txt").unwrap())
        .lines().into_iter().map(|l| l.unwrap().chars().into_iter()
                 .map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut basins :Vec<u32> = Vec::new();

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
                    basins.push(get_basin_size(i,j,&data));
                    continue;
                }
            }

            // no up
            if left !=-1 && right != -1 && up == -1 && down != -1 {
                if *current_item < left as u32 && *current_item < down as u32 && *current_item < right as u32 {
                    basins.push(get_basin_size(i,j,&data));
                    continue;
                }
            }

            // no down
            if left !=-1 && right != -1 && up != -1 && down == -1 {
                if *current_item < left as u32 && *current_item < up as u32 && *current_item < right as u32 {
                    basins.push(get_basin_size(i,j,&data));
                    continue;
                }
            }

            // no left
            if left ==-1 && right != -1 && up != -1 && down != -1 {
                if *current_item < up as u32 && *current_item < down as u32 && *current_item < right as u32 {
                    basins.push(get_basin_size(i,j,&data));
                    continue;
                }
            }

            // no right
            if left !=-1 && right == -1 && up != -1 && down != -1 {
                if *current_item < left as u32 && *current_item < up as u32 && *current_item < down as u32 {
                    basins.push(get_basin_size(i,j,&data));
                    continue;
                }
            }

            // no left and up
            if left ==-1 && right != -1 && up == -1 && down != -1 {
                if *current_item < down as u32 && *current_item < right as u32 {
                    basins.push(get_basin_size(i,j,&data));
                    continue;
                }
            }

            // no left and down
            if left ==-1 && right != -1 && up != -1 && down == -1 {
                if *current_item < up as u32 && *current_item < right as u32 {
                    basins.push(get_basin_size(i,j,&data));
                    continue;
                }
            }

            //no right up
            if left !=-1 && right == -1 && up == -1 && down != -1 {
                if *current_item < left as u32 && *current_item < down as u32 {
                    basins.push(get_basin_size(i,j,&data));
                    continue;
                }
            }

            //no right  down
            if left !=-1 && right == -1 && up != -1 && down == -1 {
                if *current_item < left as u32 && *current_item < up as u32{
                    basins.push(get_basin_size(i,j,&data));
                    continue;
                }
            }
        }
    }

    basins.sort();
    let mut total_mult_size = 1;
    for i in basins.len()-3..basins.len(){
        total_mult_size*=basins.get(i).unwrap();
    }
    println!("RESULT: {}",total_mult_size);
}
