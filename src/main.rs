//TODO: comentar codigo, otimizar logica
use clap::Parser;
use std::{cmp, usize};

static LAUGHTER: &[char] = &['A', 'E', 'H', 'I', 'U'];
static BITZERO: char = '0';
static BITMAX: usize = 8;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Decodificador de risada", long_about = None)]
struct Args {
    #[arg(short, long)]
    decode: Option<String>,

    #[arg(short, long)]
    encode: Option<String>,

    #[arg(short, long)]
    file: Option<String>,
}

fn laughter_encode(string_to_decode: &str) {
    let mut binary = String::new();

    for ch in string_to_decode.chars() {
        // changed to u32 because the unicode for u32 doesnt accept é or ,
        let mut bin = format!("{:b}", ch as u32);

        // add 0 to the string, because all bins need to have 8 characters
        while bin.len() < BITMAX {
            bin.insert(0, BITZERO);
        }

        // push them to the string, so you will have 00000000 00000000 00000000
        binary.push_str(&bin);
        binary.push(' ');
    }

    let splited_binary = binary.split(" ");

    let mut array_base5 = String::new();

    for n in splited_binary {
        array_base5.push_str(&set_base5(n));
    }

    let mut output = String::new();

    for (_i, c) in array_base5.char_indices() {
        let index = c.to_digit(10).unwrap() as usize;
        output.push(LAUGHTER[index]);
    }

    println!("{}", output);
}

fn set_base5(s: &str) -> String {
    let mut n = 0;
    let mut result = String::new();

    // Since this take every character and sent the n result to get the LAUGH character
    for (i, c) in s.chars().rev().enumerate() {
        n += match c {
            '0' => 0,
            // Calculates the pow, for example 2 ^ 3 if i == 3
            '1' => 2i32.pow(i as u32),
            _ => unreachable!(),
        };
    }

    while n > 0 {
        // I believe this part is for make the result value be 5 or less
        result.push(char::from((n % 5) as u8 + '0' as u8));
        n /= 5;
    }

    result.chars().rev().collect()
}

fn validate_laughter(encoded_laugh: &str) -> String {
    let mut final_message = String::new();

    if encoded_laugh.len() % 3 == 0 {
        for letter in encoded_laugh.chars() {
            if LAUGHTER.contains(&letter) {
                for (pos, e) in LAUGHTER.iter().enumerate() {
                    if e == &letter {
                        final_message.push_str(&format!("{}", pos));
                    }
                }
            } else {
                panic!("Invalid Char '{}' found in the laugh", &letter)
            }
        }

        return final_message;
    } else {
        panic!("INVALID LAUGH!!");
    }
}

fn laughter_decode(mut encoded_laugh: &str) {
    let mut v = vec![];

    let mut final_string = String::new();

    while !encoded_laugh.is_empty() {
        //cut the laught in 3 per time
        let (chunk, rest) = encoded_laugh.split_at(cmp::min(3, encoded_laugh.len()));
        v.push(chunk);

        // set the laugh encoded to the rest of the split
        encoded_laugh = rest;

        let mut binary_vec = vec![];

        let _string = String::new();

        for letters in &v {
            let binary = cast_to_binary(letters);

            binary_vec.push(binary);
        }

        for i in binary_vec {
            let y = isize::from_str_radix(&i, 2).unwrap();

            let ascii = y as u8;
            let character = ascii as char;

            if encoded_laugh.is_empty() {
                final_string.push(character);
            }
        }
    }
    println!("{}", final_string);
}

fn cast_to_binary(s: &str) -> String {
    let mut n = 0;
    let mut result = String::new();

    for (i, c) in s.chars().rev().enumerate() {
        n += match c {
            '0' => 0,
            '1' => 1 * 5i32.pow(i as u32),
            '2' => 2 * 5i32.pow(i as u32),
            '3' => 3 * 5i32.pow(i as u32),
            '4' => 4 * 5i32.pow(i as u32),
            _ => unreachable!(),
        };
    }

    while n > 0 {
        result.push(char::from((n % 2) as u8 + '0' as u8));
        n /= 2;
    }

    result.chars().rev().collect()
}

fn main() {
    let args = Args::parse();

    if let Some(ref decode_value) = args.decode {
        println!("Decodificando: {}", decode_value);
        let risada_base5 = validate_laughter(&decode_value);
        laughter_decode(&risada_base5);
    }

    if let Some(ref encode_value) = args.encode {
        println!("Codificando: {}", encode_value);
        laughter_encode(&encode_value)
    }
}
