use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: ncut <filePath> <columnIndex>");
        return;
    }


    let args: Vec<String> = env::args().collect();
    let columnIndex = &args[1].parse::<usize>().unwrap();
    println!("{}", columnIndex);

    let filePath = &args[2];
    println!("{}", filePath);

    streamColumn(&filePath, *columnIndex);
}

fn streamColumn(filePath : &str, i: usize) {
    let mut data = String::new();
    let mut f = File::open(filePath).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    for line in data.lines() {
        let mut words: Vec<&str> = line.split(',').collect();
        println!("{}", words[i as usize]);
    }
}
