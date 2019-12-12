use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let mut sum_normal: i32 = 0;
    let mut sum_enhanced: i32 = 0;
    for line in buf_reader.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        sum_normal += mass/3-2;
        sum_enhanced += calculate_enhanced(mass);
    }
    println!("The normal fuel requirement is {}!", &sum_normal);
    println!("The enhanced fuel requirement is {}!", &sum_enhanced);
    Ok(())
}

fn calculate_enhanced(mass: i32) -> i32 {
    let result = mass/3 - 2;
    if result > 0 {
        calculate_enhanced(result) + result
    } else {
        0
    }
}
