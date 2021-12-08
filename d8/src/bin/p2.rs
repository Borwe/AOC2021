use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Data{
    inputs: Vec<String>,
    outputs: Vec<String>,
    input_nums: HashMap<u8, String>
}

impl Data {
    fn get_numbers_map(&mut self){
        // get 1,4,7,8 seg
        self.inputs.sort();
        self.inputs.sort_by(|x, y|{
            x.len().cmp(&y.len())
        });
        self.input_nums.insert(1,self.inputs.get(0).unwrap().clone());
        self.input_nums.insert(7,self.inputs.get(1).unwrap().clone());
        self.input_nums.insert(4,self.inputs.get(2).unwrap().clone());
        self.input_nums.insert(8,self.inputs.get(self.inputs.len()-1).unwrap().clone());

        
        //for 6,0,9
        for i in 6..9{
            let one = self.input_nums.get(&1).unwrap();
            let four = self.input_nums.get(&4).unwrap();
            let mut chars_in_one = 0;
            let mut chars_in_four = 0;
            let string: String = self.inputs.get(i).unwrap().clone();

            one.clone().chars().for_each(|c|{
                if string.contains(&String::from(c)) == true {
                    chars_in_one+=1;
                }
            });
            four.clone().chars().for_each(|c|{
                if string.contains(&String::from(c)) == true {
                    chars_in_four+=1;
                }
            });

            if chars_in_one == 1 {
                self.input_nums.insert(6,string);
            } else if chars_in_one != 1 && chars_in_four == 3 {
                self.input_nums.insert(0,string);
            } else {
                self.input_nums.insert(9,string);
            }
        }

        //set 3
        for i in 3..6{
            let one = self.input_nums.get(&1).unwrap();
            let nine = self.input_nums.get(&9).unwrap();
            let mut chars_in_one = 0;
            let mut extra_car_than_nine = 0;
            let string: String = self.inputs.get(i).unwrap().clone();
            one.clone().chars().for_each(|c|{
                if string.contains(&String::from(c)) == true {
                    chars_in_one+=1;
                }
            });
            string.clone().chars().for_each(|c|{
                if nine.contains(&String::from(c)) == false {
                    extra_car_than_nine+=1;
                }
            });

            if chars_in_one == 2 {
                self.input_nums.insert(3,string);
            }else if extra_car_than_nine == 1 {
                self.input_nums.insert(2,string);
            }else {
                self.input_nums.insert(5,string);
            }
        }
    }


    fn get_output_value(&mut self)-> u64{
        self.get_numbers_map();
        let mut string_value = String::new();
        for i in self.outputs.clone().into_iter(){
            for (k,v) in self.input_nums.clone(){
                if v.len()==i.len(){
                    let count =v.clone().chars().into_iter().filter(|c| {
                        i.contains(*c)
                    }).count();
                    if count == i.len(){
                        string_value+=&k.to_string();
                    }
                }
            }
        }
        string_value.parse::<u64>().unwrap()
    }

}

fn main() {
    let datas = BufReader::new(File::open("input.txt").unwrap())
        .lines().into_iter()
        .map(|x| {
            let split = x.unwrap().split("|").map(|x| String::from(x)).collect::<Vec<String>>();
            let inputs = 
                    split.get(0).unwrap().split(" ").map(|x| String::from(x)).filter(|x| x.len()>0).collect::<Vec<String>>();
            let outputs = 
                    split.get(1).unwrap().split(" ").map(|x| String::from(x)).filter(|x| x.len()>0).collect::<Vec<String>>();
            Data{
                inputs,
                outputs,
                input_nums: HashMap::new()
            }
        })
        .collect::<Vec<Data>>();
    println!("SUM: {}",datas.into_iter().map(|mut d| d.get_output_value()).sum::<u64>());
}
