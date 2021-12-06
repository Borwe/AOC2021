use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap,HashSet};
use std::cell::RefCell;


#[derive(Debug)]
struct School {
    total_fishes_produced: u128,
    starting_fishes: HashSet<u8>,
    overall_days: i32,
    repeats: HashMap<u8,u32>
}

impl School {
    fn from(file: File,overall_days: i32)->Self{

        let fishes: Vec<u8> = BufReader::new(file).lines().into_iter().flat_map(|x|{
            let mut z: Vec<String> = Vec::new();
            x.unwrap().split(",").for_each(|s|{
                z.push(String::from(s));
            });
            z
        }).map(|x| x.parse::<u8>().unwrap()).collect();
        let mut repeats: HashMap<u8,u32> = HashMap::new();
        for f in fishes.clone(){
            if repeats.contains_key(&f) {
                repeats.insert(f, repeats.get(&f).unwrap()+1);
            }else {
                repeats.insert(f,1);
            }
        }

        let fishes = fishes.into_iter().collect::<HashSet<u8>>();
        School{
            total_fishes_produced: 0,
            starting_fishes: fishes,
            overall_days,
            repeats
        }
    }
    
    fn simulate(&mut self){
        for f in self.starting_fishes.clone(){
            let total: RefCell<u128> = self.prod_from_read_fish(f);
            match self.repeats.get(&f){
                Some(x)=>{
                    self.total_fishes_produced=self.total_fishes_produced+ (total.into_inner()*((*x) as u128));
                },
                _ => {continue;}
            }
        }
    }

    fn prod_from_read_fish(&mut self, day: u8)-> RefCell<u128>{
        let mut overall_days: i32 = self.overall_days - (day+1) as i32;
        let total: RefCell<u128> = RefCell::new(1);
        self.prod_fresh(overall_days,&total);
        while (overall_days-7) >= 0 {
            overall_days -= 7;
            self.prod_fresh(overall_days,&total);
        }
        total
    }

    fn prod_fresh(&mut self, mut overall_days: i32,total:& RefCell<u128>){
        {
            let mut totoal_b = total.borrow_mut();
            *totoal_b+=1;
        }
        if (overall_days-9) >= 0{
            overall_days -= 9;
            self.prod_fresh(overall_days,total);
            while (overall_days-7) >= 0 {
                overall_days -= 7;
                self.prod_fresh(overall_days,total);
            }
        }
    }
}

fn main() {
    let mut school = School::from(File::open("input.txt").unwrap(),256);

    school.simulate();
    println!("There are now {} fish",school.total_fishes_produced);
    //println!("Last day: {:?}",school.fishes.into_iter().map(|r|{
    //    r.into_inner()
    //}).map(|f| f.cycle).collect::<Vec<u32>>());
}
