use std::io;
use std::fs::File;
use std::io::Read;
use std::env;
use std::char;
use std::error::Error;
use std::io::{stdin,stdout,Write};

const SIZE: usize = 30000;

struct Env {
    ptr: usize,
    data: [u8; SIZE]
}

fn main() {
    // Initialize the Brainf*ck Stack
    let mut stack: Env = Env {
        ptr: 0,
        data: [0; SIZE]
    };
    // Read in Arguments and Open File
    let args: Vec<String> = env::args().collect();
    let mut f  = match File::open(&args[1]) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", &args[1],
                                                   why.description()),
        Ok(file) => file,
    };
    let mut buffer = String::new();
    f.read_to_string(&mut buffer);
    // Create an iterator from the file
    let char_vec: Vec<char> = buffer.chars().collect();
    let mut block_start = Vec::new();
    let mut read_pos: usize = 0;
    while read_pos < char_vec.len() {
        match char_vec[read_pos] {
            '>' => stack.ptr += 1,
            '<' => stack.ptr -= 1,
            '+' => stack.data[stack.ptr] += 1,
            '-' => stack.data[stack.ptr] -= 1,
            '.' => print!("{}", stack.data[stack.ptr] as char),
            ',' => {
                let mut input=String::new();
                let _=stdout().flush();
                stdin().read_line(&mut input).expect("Did not enter a correct string");
                if let Some('\n')=input.chars().next_back() {
                    input.pop();
                }
                if let Some('\r')=input.chars().next_back() {
                    input.pop();
                }
                if input.len() != 0 {
                    let ch = input.chars().next().unwrap();
                    stack.data[stack.ptr] = ch as u8;
                } else {
                    stack.data[stack.ptr] = 0;
                }
            },
            '[' => {
                if stack.data[stack.ptr] != 0 {
                    block_start.push(read_pos);
                } else {
                    let mut block_count = 0;
                    read_pos += 1;
                    while (block_count != 0 || char_vec[read_pos] != ']') && read_pos < char_vec.len()  {
                        if char_vec[read_pos] == '[' {
                            block_count += 1;
                        } else if char_vec[read_pos] == ']' {
                            block_count -= 1;
                        }
                        read_pos += 1;
                    }
                }
            },
            ']' => {
                let new_pos = match block_start.pop() {
                    Some(v) => v,
                    None => read_pos
                };
                if stack.data[stack.ptr] != 0 {
                    read_pos = new_pos;
                    block_start.push(read_pos);
                }
            },
            _ => ()
        }
        read_pos += 1;
    }
}
