use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::vec;

static mut C: usize = 1;

fn main() {
    let args: Vec<String> = args().collect();
    let s = read_vectors_from_file(args[1].as_str());
    let mut r: Vec<usize> = vec![];
    let mut p = false;

    unsafe {
        for i in s.clone() {
            if r.is_empty() {
                r.push(i);
                C = 1;
            } else if i > r[r.len() - 1] {
                r.push(i);
                C = 1;
            } else if i == r[r.len() - 1] {
                r.push(i);
                C = 1;
            } else if i < r[r.len() - 1] {
                p = true;
                while p {
                    if i < r[r.len() - C] {
                        if i < r[0] {
                            r.insert(0, i);
                            p = false;
                            C = 1;
                        } else if i == r[C] {
                            r.insert(C + 1, i);
                            p = false;
                            C = 1;
                        } else if i > r[r.len() - C] {
                            r.insert(r.len() - C, i);
                        } else if i < r[0] || i < r[1] {
                            r.insert(1, i);
                            p = false;
                            C = 1;
                        } else {
                            C += 1;
                        }
                    } else if i == r[r.len() - C] {
                        r.insert(r.len() - C, i);
                        p = false;
                        C = 1;
                    } else if i > r[r.len() - C] {
                        r.insert((r.len() - C) + 1, i);
                        C = 1;
                        p = false;
                    }
                }
            }
        }
    }
    println!("{r:?}");
}

fn read_vectors_from_file(filename: &str) -> Vec<usize> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vectors = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let vector = usize::from_str(&line).unwrap();

        vectors.push(vector);
    }

    vectors
}
