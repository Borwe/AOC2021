use std::fs::File;
use std::io::{BufReader, BufRead};

type VecVecs = Vec<Vec<u32>>;

fn turn_string_to_vec(string:String)-> Vec<u32>{
    let mut vec: Vec<u32> = Vec::new();
    for x in string.chars(){
        vec.push(x.to_digit(10).unwrap());
    }
    vec
}

fn turn_to_dec(collection:& Vec<u32>)-> u32{
    let len= collection.len()-1 ;
    let mut return_val: u32 = 0;
    let base: u32 = 2;
    for (pos, val) in collection.into_iter().enumerate(){
        return_val = return_val + (val * (base.pow((len as u32)-(pos as u32))));
    }
    return_val
}

fn get_column_with_1_and_0(vec: &Vec<&Vec<u32>>, column: usize)-> (Vec<usize>,Vec<usize>){
    let mut vec_of_ones:Vec<usize> = Vec::new();
    let mut vec_of_zeroes:Vec<usize> = Vec::new(); 
    for (pos,value) in vec.into_iter().enumerate(){
        if *(value.get(column).unwrap())==1{
            vec_of_ones.push(pos);
        }else{
            vec_of_zeroes.push(pos);
        }
    }
    (vec_of_ones,vec_of_zeroes)
}

fn get_slice_of_vec_vecs(vec: Vec<&Vec<u32>>, mut slice: Vec<usize>)-> Vec<&Vec<u32>>{
    let mut return_val:Vec<&Vec<u32>> = Vec::new();
    for (pos,val) in vec.into_iter().enumerate(){
        if slice.is_empty() {
            break;
        }

        if slice.contains(&pos) {
            for (p,v) in slice.clone().into_iter().enumerate(){
                if v==pos {
                    slice.remove(p);
                    break;
                }
            }
            return_val.push(val);
        }
    }
    return_val
}

fn get_oxygen_rating(input_vec:& VecVecs)->u32{
    let length = (&input_vec[0]).len()-1;
    let mut vals_to_use: Vec<& Vec<u32>> = Vec::new();
    for val in input_vec{
        vals_to_use.push(val);
    }

    for i in 0..=length{
        let (ones,zeroes)=get_column_with_1_and_0(&vals_to_use,i);
        let ones_size = ones.len();
        let zeroes_size = zeroes.len();
        if ones_size==1 && zeroes_size==1{
            //return the amount of oxygen rating
            println!("O2 ones:{}, zeroes{}",ones_size,zeroes_size);
            return turn_to_dec(get_slice_of_vec_vecs(vals_to_use,ones).get(0).unwrap());
        }else if ones_size==0 {
            println!("O2 ones:{}, zeroes{}",ones_size,zeroes_size);
            return turn_to_dec(get_slice_of_vec_vecs(vals_to_use,zeroes).get(0).unwrap());
        }else if zeroes_size==0 {
            println!("O2 ones:{}, zeroes{}",ones_size,zeroes_size);
            return turn_to_dec(get_slice_of_vec_vecs(vals_to_use,ones).get(0).unwrap());
        }else if zeroes_size > ones_size {
            //determine zeroes size is more
            vals_to_use = get_slice_of_vec_vecs(vals_to_use,zeroes);
        }else if ones_size>=zeroes_size{
            //determine one size has more
            vals_to_use = get_slice_of_vec_vecs(vals_to_use,ones);
        }
    }
    0
}


fn get_co2_rating(input_vec:& VecVecs)->u32{
    let length = (&input_vec[0]).len()-1;
    let mut vals_to_use: Vec<& Vec<u32>> = Vec::new();
    for val in input_vec{
        vals_to_use.push(val);
    }

    for i in 0..=length{
        let (ones,zeroes)=get_column_with_1_and_0(&vals_to_use,i);
        let ones_size = ones.len();
        let zeroes_size = zeroes.len();
        if zeroes_size==1 && ones_size==1 {
            //return the amount of co2 rating
            println!("CO2 ones:{}, zeroes{}",ones_size,zeroes_size);
            return turn_to_dec(get_slice_of_vec_vecs(vals_to_use,zeroes).get(0).unwrap());
        }else if ones_size==0 {
            println!("CO2 ones:{}, zeroes{}",ones_size,zeroes_size);
            return turn_to_dec(get_slice_of_vec_vecs(vals_to_use,zeroes).get(0).unwrap());
        }else if zeroes_size==0 {
            println!("CO2 ones:{}, zeroes{}",ones_size,zeroes_size);
            return turn_to_dec(get_slice_of_vec_vecs(vals_to_use,ones).get(0).unwrap());
        }else if zeroes_size <= ones_size {
            //determine zeroes size is less
            vals_to_use = get_slice_of_vec_vecs(vals_to_use,zeroes);
        }else if ones_size<zeroes_size{
            //determine one size has less
            vals_to_use = get_slice_of_vec_vecs(vals_to_use,ones);
        }
    }
    0
}

fn main(){
    let input: VecVecs = BufReader::new(File::open("input.txt").unwrap())
        .lines().into_iter().map(|x| String::from(x.unwrap())).map(|x| turn_string_to_vec(x))
        .collect();
    let oxygen_rating = get_oxygen_rating(&input);
    let co2_rating = get_co2_rating(&input);
    println!("Oxygen Rate: {}, Co2 Rate: {}",oxygen_rating,co2_rating);
    println!("Life support rating: {}", (co2_rating*oxygen_rating));
}
