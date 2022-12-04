use lazy_static::lazy_static;
use std::fs;

use regex::Regex;

fn extract_limits(text: &str) -> Vec<u8> {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"[0-9]?[0-9]"
            ).unwrap();
    }

    REGEX.find_iter(text).map(
        |mat| mat.as_str().parse::<u8>().unwrap()
    ).collect::<Vec<u8>>()
}

fn main() {
    let fp = String::from("input");
    let input = fs::read_to_string(fp).expect("Cannot read");

    let mut counter : i32 = 0;
    let mut overlaps : i32 = 0;
    for line in input.split("\n"){
        let limit = extract_limits(line);//.iter().map(|x| x.parse::<u8>().unwrap()).collect();

        // a1 = limit[0], b1 = limit[1]
        // a2 = limit[2], b2 = limit[3]

        //fully contained
        //    a1---b1 
        // a2---------b2
        if (limit[2] >= limit[0] && limit[3] <= limit[1] )   || ( limit[0] >= limit[2] && limit[1] <= limit[3] ) {
            counter+=1;
        }

        //overlaps
        // a1---b1
        // a2-------b2
        if  limit[0] <= limit[3] && limit[2] <= limit[1]  {
            overlaps+=1;
        }
    }

    println!("Fully contained:{counter}\nOverlaps: {overlaps}");

}
