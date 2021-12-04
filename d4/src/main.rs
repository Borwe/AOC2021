use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct BingoTable{
    rows: Vec<u32>
}

struct HasScore {
    has: bool
}

impl BingoTable{
    fn get_rows(&self)-> Vec<&[u32]>{
        let mut rows: Vec<&[u32]> = Vec::new();
        for i in 0..5{
            let starting = i*5;
            let ending = starting+5;
            rows.push(&self.rows[starting..ending]);
        }
        rows
    }

    fn get_collums(&self)-> Vec<Vec<u32>>{
        let mut colls: Vec<Vec<u32>> = Vec::new();
        for i in 0..5{
            let mut vec: Vec<u32> = Vec::new();
            vec.push(*self.rows.get(0+i).unwrap());
            vec.push(*self.rows.get(5+i).unwrap());
            vec.push(*self.rows.get(10+i).unwrap());
            vec.push(*self.rows.get(15+i).unwrap());
            vec.push(*self.rows.get(20+i).unwrap());
            colls.push(vec)
        }
        colls
    }

    fn does_coll_fill_with_scores(&self, scores:&[u32])-> HasScore{
        let mut count = 0;
        for v in self.get_collums(){
            for x in v{
                for s in scores{
                    if x == *s {
                       count = count+1;
                    }
                }
            }
            if count>=5 {
                return HasScore{  has:true }
            }else{
                count=0;
            }
        }
        return HasScore{  has:false }
    }

    fn does_row_fill_with_scores(&self, scores:&[u32])-> HasScore{
        let mut count = 0;
        for v in self.get_rows(){
            for x in v{
                for s in scores{
                    if x == s {
                       count = count+1;
                    }
                }
            }
            if count >= 5 {
                return HasScore{  has:true }
            }else{
                count =0
            }
        }
        HasScore{  has:false }
    }
}

fn get_scores()-> Vec<u32>{
    BufReader::new(File::open("input.txt").unwrap()).lines().next()
        .map(|x| {
        let y: Vec<u32> = String::from(x.unwrap()).split(",").into_iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        y
    }).unwrap()
}

fn parse_bingo_tables()->Vec<BingoTable>{
    let mut bingo_tables: Vec<BingoTable>= Vec::new();
    let mut lines = BufReader::new(File::open("input.txt").unwrap())
        .lines().into_iter();
    lines.next();
    lines.next();

    let mut bingo_table = BingoTable{rows: Vec::new()};
    for l in lines {
        let string = String::from(l.unwrap());
        if string.len()>=13 {
            for x in string.split(" "){
                if x.len()>0{
                    bingo_table.rows.push(x.parse::<u32>().unwrap());
                }
            }
        }else{
            bingo_tables.push(bingo_table);
            bingo_table = BingoTable{rows: Vec::new()};
        }
    }
    bingo_tables.push(bingo_table);
    bingo_tables
}

fn sum_of_unmarked_bingo_tabl(table: &BingoTable, scores: &[u32])->u32{
    let mut to_sum_vec: Vec<u32> = Vec::new();
    table.get_rows().into_iter().for_each(|x|{
        for y in x {
            if scores.contains(y)==false{
                to_sum_vec.push(*y);
            }
        }
    });
    let sum: u32 = to_sum_vec.into_iter().sum();
    println!("Scores len: {}",scores.len());
    let score_final:u32 = *scores.get(scores.len()-1).unwrap();
    sum * score_final
}

fn get_final_score(scores: Vec<u32>,tables: Vec<BingoTable>)->u32{
    let scores_len = scores.len();
    for i in 0..scores_len{
        let scores_used = &scores[0..=i];
        for bt in (&tables).into_iter(){
            let result_cols = bt.does_coll_fill_with_scores(scores_used);
            let result_rows = bt.does_row_fill_with_scores(scores_used);
            if result_cols.has==true {
                return sum_of_unmarked_bingo_tabl(bt,scores_used);
            }else if result_rows.has==true {
                return sum_of_unmarked_bingo_tabl(bt,scores_used);
            }
        }
    }
    0
}

fn main() {
    let scores = get_scores();
    let tables = parse_bingo_tables();
    let score = get_final_score(scores,tables);
    println!("Score is: {}",score);
}
