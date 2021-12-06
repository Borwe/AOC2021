use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cell::RefCell;

#[derive(Debug)]
struct Fish {
    cycle: u8,
    produces: bool
}

impl Fish {
    fn set_produces_true(&mut self){
        self.produces=true;
    }
}

#[derive(Debug)]
struct School {
    fishes: Vec<RefCell<Fish>>
}

impl School {
    fn from(file: File)->Self{
        
        School{fishes: BufReader::new(file).lines().into_iter().flat_map(|x|{
            let mut z: Vec<String> = Vec::new();
            x.unwrap().split(",").for_each(|s|{
                z.push(String::from(s));
            });
            z
        }).map(|x| x.parse::<u8>())
            .map(|x| RefCell::new(Fish{cycle: x.unwrap(), produces: true})).collect()}
    }

    fn simulate(&mut self, days: u16){
        for _ in 0..days{
            let mut new_fishes: Vec<Fish> = Vec::new();
            for j in 0..self.fishes.len(){
                let mut fish = self.fishes.get(j).unwrap().borrow_mut();

                if fish.cycle==0{
                    fish.cycle = 6;
                }else{
                    fish.cycle = fish.cycle -1;
                }

                if fish.cycle==6 && fish.produces == true {
                    new_fishes.push(Fish{cycle: 8, produces: false});
                }else if fish.cycle==6 && fish.produces == false {
                    fish.set_produces_true();
                }
            }

            for f in new_fishes {
                self.fishes.push(RefCell::new(f));
            }
        }
    }
}

fn main() {
    let mut school = School::from(File::open("input.txt").unwrap());

    school.simulate(80);
    println!("There are now {} fish",school.fishes.len());
    //println!("Last day: {:?}",school.fishes.into_iter().map(|r|{
    //    r.into_inner()
    //}).map(|f| f.cycle).collect::<Vec<u32>>());
}
