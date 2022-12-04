use lazy_static::lazy_static;
use std::fs;

use regex::Regex;
use std::collections::HashSet;

fn extract_limits(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"[0-9]?[0-9]"
            ).unwrap();
    }
    REGEX.find_iter(text).map(|mat| mat.as_str()).collect::<Vec<&str>>()
}

fn main() {
    let fp = String::from("input");
    let input = fs::read_to_string(fp).expect("Cannot read");

    let mut counter : i32 = 0;
    let mut overlaps : i32 = 0;
    for line in input.split("\n"){
        let limit : Vec<u8> = extract_limits(line).iter().map(|x| x.parse::<u8>().unwrap()).collect();

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

    /*
    let target = "1-11,1-44";
    //let res = extract_couples(target);
    let t = extract_couples2(target);
    let integers : Vec<i32> = t.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    for x in integers.iter() { println!("{}",x); }
    println!{"{:?}",t};
    
    //let r1 : HashSet<i32> = [0..4].into_iter().collect::<HashSet<i32>>();
    let first: Vec<i32> = (integers[0]..integers[1]).collect();
    let mut a: HashSet<i32> = (integers[0]..integers[1]).collect();

    let second: Vec<i32> = (integers[2]..integers[3]).collect();
    //let mut b: HashSet<i32> = second.into_iter().collect();
    let mut b : HashSet<i32> = (integers[2]..integers[3]).collect();
    println!("{:?}",&a);
    println!("{:?}",&b);

    if  a.is_subset(&b) || b.is_subset(&a) {
        println!("GOT EM!\n");
    }

    */

}
