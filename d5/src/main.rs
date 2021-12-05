use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet,HashMap};

#[derive(Debug,PartialEq, Eq,Hash)]
struct Points{
    x: u32,
    y: u32
}

impl Points{
    fn get_points_between_hor_or_vec(&self, point: &Points)->HashSet<Points>{
        let mut points: HashSet<Points> = HashSet::new();
        points.insert(Points{x:self.x,y:self.y});
        points.insert(Points{x:point.x,y:point.y});
        if self.x == point.x {
            //straight along y
            if self.y <= point.y{
                for y in self.y..=point.y{
                    points.insert(Points{x:self.x,y});
                }
            }else{
                for y in point.y..=self.y{
                    points.insert(Points{x:self.x,y});
                }
            }
        }
        if self.y == point.y{
            //straight along x
            if self.x <= point.x {
                for x in self.x..=point.x{
                    points.insert(Points{x,y:self.y});
                }
            }else {
                for x in point.x..=self.x{
                    points.insert(Points{x,y:self.y});
                }
            }
        }
        points
    }
}

#[derive(Debug)]
struct Pipe{
    points: Vec<Points>,
}

impl Pipe{
    fn new(string: String)->Self{
        let mut beg_end: Vec<&str> = string.split(" -> ").collect();
        let mut points: Vec<Points> = Vec::new();
        for i in 0..2{
            let points_vec: Vec<&str> = beg_end.get_mut(i)
                .unwrap().split(",").collect();
            points.push(Points{
                x: points_vec.get(0).unwrap().parse::<u32>().unwrap(),
                y: points_vec.get(1).unwrap().parse::<u32>().unwrap()});
        }
        Pipe{points}
    }

    fn is_vertical_or_horizontal(&self)->bool{
        let x1 = self.points.get(0).unwrap().x;
        let x2 = self.points.get(1).unwrap().x;
        let y1 = self.points.get(0).unwrap().y;
        let y2 = self.points.get(1).unwrap().y;

        if x1 == x2 || y1 == y2 {
            true
        }else{
            false
        }
    }
}

fn get_pipes()->Vec<Pipe>{
    BufReader::new(File::open("input.txt").unwrap()).lines()
        .map(|x|{
            Pipe::new(x.unwrap())
        }).collect()
}

fn get_points_that_score_2_or_more(pipes: Vec<Pipe>)->usize{
    let mut points: HashMap<Points,u32> = HashMap::new();
    for pipe in pipes{
        let point_a = pipe.points.get(0).unwrap();
        let point_b = pipe.points.get(1).unwrap();
        point_a.get_points_between_hor_or_vec(point_b).into_iter().for_each(|ps|{
            let opt_val =  points.get(&ps);
            if opt_val == None{
                points.insert(ps,1);
            }else{
                let val: u32 = *opt_val.unwrap();
                points.insert(ps,val+1);
            }
        });
    }
    let mut count = 0;
    for (_,v) in points.into_iter(){
        if v>=2 {
            count=count+1;
        }
    }
    count
}

fn main() {
    let pipes: Vec<Pipe> = get_pipes().into_iter()
        .filter(|x| x.is_vertical_or_horizontal()).collect();

    println!("Ovelapping points: {}",get_points_that_score_2_or_more(pipes));
}
