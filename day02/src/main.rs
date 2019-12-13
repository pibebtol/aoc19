use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let mut input: Vec<i32> = Vec::new();
    for s in content.split(',') {
        input.push(s.parse::<i32>().unwrap());
    }
    for i in 0..99 {
        for j in 0..99 {
            if compute(input.clone(), i, j) == 19_690_720 {
                println!("The result is {}", 100 * i + j);
            }
        }
    }
    println!("The 0 index is {}", compute(input, 12, 2));
    Ok(())
}

fn compute(mut code: Vec<i32>, i: i32, j: i32) -> i32 {
    code[1] = i;
    code[2] = j;
     for i in 0..code.len() {
        if i % 4 == 0 {
            if code[i] == 99 {
                break;
            }
            let i1 = code[i+1];
            let i2 = code[i+2];
            let i3 = code[i+3];
            if code[i] == 1 {
                code[i3 as usize] = code[i1 as usize] + code[i2 as usize];
            }
            if code[i] == 2 {
                code[i3 as usize] = code[i1 as usize] * code[i2 as usize];
            }
        }
    }
    code[0]
}
