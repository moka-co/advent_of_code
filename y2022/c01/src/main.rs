use std::fs;

fn max_calories_elf(contents: &String) -> i32 {
    let mut sum : i32 = 0;
    let mut max : i32 = sum;

    for line in contents.lines(){

        if line.is_empty(){

            if sum > max {
                max = sum;
            }
            sum = 0;
        }else{

            sum += line.parse::<i32>().unwrap(); //line to input
        }
    }

    max
}


fn max_three_calories_elf(contents: &String) -> i32 {
    let mut tuple: (i32, i32, i32) = (0, 0, 0);
    let mut sum : i32 = 0;

    for line in contents.lines(){

        if line.is_empty(){

            tuple = if  sum > tuple.0{
                ( sum, tuple.0, tuple.1)
            }else if sum > tuple.1 {
                (tuple.0, sum, tuple.1)
            }else if  sum > tuple.2{
                ( tuple.0, tuple.1, sum)
            }else{
                tuple
            };
            
            sum = 0;
        }else{

            sum += line.parse::<i32>().unwrap(); //line to input
        }
    }

    tuple.0+tuple.1+tuple.2
}

fn main() {
    let filepath = String::from("input");

    let contents = fs::read_to_string(filepath)
        .expect("Cannot read file");
    
    println!("Max calories: {}", max_calories_elf(&contents));
    let res = max_three_calories_elf(&contents); 
    println!("Sum of three max calories: {}",res);
    
}
