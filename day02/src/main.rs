use std::fs::File;
use std::io::Read;

fn main() {
    // read in input from file
    let mut file = File::open("input")
        .expect("couldn't read in file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("couldn't parse file properly");

    // convert input to vector
    let mut input = Vec::new();
    for s in content.split(',') {
        input.push(s.parse()
            .expect("couldn't parse input array"));
    }
    
    // calculate result
    println!("The 0 index for code 1202 is {}", compute(input.clone(), 12, 2));
    for i in 0..=99 {
        for j in 0..=99 {
            if compute(input.clone(), i, j) == 19_690_720 {
                println!("The result is {}", 100 * i + j);
            }
        }
    }
}

fn compute(mut code: Vec<usize>, second: usize, third: usize) -> usize {
    code[1] = second;
    code[2] = third;

    // execut OP-code
    let mut pos = 0;
    while code[pos] != 99 {
        let pos3 = code[pos + 3];
        match code[pos] {
            1 => {
                code[pos3] = code[code[pos + 1]] + code[code[pos + 2]];
                pos += 4;
            },
            2 => {
                code[pos3] = code[code[pos + 1]] * code[code[pos + 2]];
                pos += 4;
            },
            _ => println!("Somethings itchy here!"),
        }
    }
    code[0]
}
