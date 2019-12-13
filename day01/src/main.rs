use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    // read in file
    let file = File::open("input")
        .expect("File could not be read");
    let buf_reader = BufReader::new(file);

    // calculate sums
    let mut sum_normal = 0;
    let mut sum_enhanced = 0;
    for line in buf_reader.lines() {
        let mass = line.expect("error getting line")
            .parse().expect("couldn't parse line");
        sum_normal += mass / 3 - 2;
        sum_enhanced += calculate_enhanced(mass);
    }
    println!("The normal fuel requirement is {}!", &sum_normal);
    println!("The enhanced fuel requirement is {}!", &sum_enhanced);
}

// recursive calculation of the mass (including fuel mass)
fn calculate_enhanced(mass: i32) -> i32 {
    let result = mass / 3 - 2;
    if result > 0 {
        calculate_enhanced(result) + result
    } else {
        0
    }
}
