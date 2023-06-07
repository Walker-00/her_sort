use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::vec;

fn main() {
    let args: Vec<String> = args().collect();
    let s = read_vectors_from_file(&args[1]);
    let mut r: Vec<u32> = vec![];
    let mut c = 1;

    for i in s {
        if r.is_empty() || i >= r[r.len() - 1] {
            r.push(i);
        } else if i < r[r.len() - 1] {
            loop {
                if i < r[r.len() - c] {
                    if i < r[0] {
                        r.insert(0, i);
                        c = 1;
                        break;
                    } else if i == r[c] {
                        r.insert(c + 1, i);
                        c = 1;
                        break;
                    } else if i > r[r.len() - c] {
                        r.insert(r.len() - c, i);
                        break;
                    } else if i < r[0] || i < r[1] {
                        r.insert(1, i);
                        c = 1;
                        break;
                    } else {
                        c += 1;
                    }
                } else if i == r[r.len() - c] {
                    r.insert(r.len() - c, i);
                    c = 1;
                    break;
                } else if i > r[r.len() - c] {
                    r.insert((r.len() - c) + 1, i);
                    c = 1;
                    break;
                }
            }
        }
    }
    println!("{:?}", r);
}

fn read_vectors_from_file(filename: &str) -> Vec<u32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vectors = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let vector = u32::from_str(&line).unwrap();

        vectors.push(vector);
    }

    vectors
}
