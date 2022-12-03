use std::fs;


fn get_priority(ch: char) -> i32 {
    let ascii : i32 = ch as i32;
    let a : i32 = 'a' as i32;
    let ma : i32= 'A' as i32;
    if ascii > 96 {
        ascii - a + 1
    }else if ascii < 91 {
        ascii - ma + 27
    }else{
        0
    }
}

fn main() {
    
    let fp = String::from("input");

    let contents = fs::read_to_string(fp).expect("Cannot read file");
    let input = contents.split("\n").collect::<Vec<&str>>();

    let mut total_priority : i32 = 0;

    //Solution one
    for line in &input {
        let size = line.len();
        let hf = size/2;
        let fc = line.get(0..hf).unwrap();
        let sc = line.get(hf..size).unwrap();

        for ch in fc.chars(){
            if sc.contains(ch) == true {
                total_priority += get_priority(ch);
                break;
            }
        }
    }
    print!("{} ",total_priority);
    total_priority=0;

    //Solution two
    for line in input.chunks(3){
        for ch in line[0].chars(){
            if line[1].contains(ch) && line[2].contains(ch){
                total_priority += get_priority(ch);
                break;
            }
        }
    }

    println!(" {}",total_priority);

    
    
}
