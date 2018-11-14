use std::io;
use std::fs::File;
use std::io::Read;
use std::env;
use std::char;
use std::error::Error;
const SIZE: usize = 30000;

struct Env {
    ptr: usize,
    data: [u8; SIZE]
}

fn main() {
    print!("This is running");
    // Initialize the Brainf*ck Stack
    let mut stack: Env = Env {
        ptr: 0,
        data: [0; SIZE]
    };
    // Read in Arguments and Open File
    let args: Vec<String> = env::args().collect();
    print!("This is running");
    let mut f  = match File::open(&args[1]) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", &args[1],
                                                   why.description()),
        Ok(file) => file,
    };
    print!("This is running");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer);
    // Create an iterator from the file
    let char_vec: Vec<char> = buffer.chars().collect();
    let mut read_pos: usize = 0;
    let mut block_start = 0;
    let mut scanning = false;
    print!("This is running");
    while read_pos < char_vec.len() {
        match char_vec[read_pos] {
            '>' => stack.ptr += 1,
            '<' => stack.ptr -= 1,
            '+' => stack.data[stack.ptr] += 1,
            '-' => stack.data[stack.ptr] -= 1,
            '.' => print!("{:?}", char::from_digit(stack.data[stack.ptr].into(), 10)),
            ',' => {
                let byte:u8 = match io::stdin().bytes().next() {
                    Some(v) => {
                        match v {
                            Err(why) => panic!("what's happening with {}", why.description()),
                            Ok(v) => v
                        }
                    }
                    None => 0
                };
                stack.data[stack.ptr] = byte;
            },
            '[' => {
                if stack.data[stack.ptr] != 0 {
                    block_start = read_pos;
                } else {
                    scanning = true;
                }
            },
            ']' => {
                if scanning {
                    scanning = false;
                } else if stack.data[stack.ptr] != 0 {
                    read_pos = block_start;
                }
            },
            _ => ()
        }
        read_pos += 1;
    }
}
