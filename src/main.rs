//TODO: comentar codigo, otimizar logica
use clap::Parser;
use std::cmp;

static RISADA: &[char] = &['A', 'E', 'H', 'I', 'U'];

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Decodificador de risada", long_about = None)]
struct Args {
    #[arg(short, long)]
    decode: Option<String>,

    #[arg(short, long)]
    encode: Option<String>,
}

fn valida_risada(risada_encodada: &str) -> String {
    let mut mensagem_final = String::new();

    if risada_encodada.len() % 3 == 0 {
        //println!("TAMANHO DE RISADA VALIDO");
        for letra in risada_encodada.chars() {
            if RISADA.contains(&letra) {
                //println!("CARACTERE VALIDO");

                for (pos, e) in RISADA.iter().enumerate() {
                    if e == &letra {
                        mensagem_final.push_str(&format!("{}", pos));
                    }
                }
            } else {
                panic!("Caracter '{}' invalido encontado na RISADA!!", &letra)
            }
        }

        return mensagem_final;
    } else {
        panic!("RISADA INVALIDA!!");
    }
}

fn encoda_risada(risada_decodada: &str) {
    let mut binary = String::new();

    for ch in risada_decodada.chars() {
        let mut bin = format!("{:b}", ch as u8);
        while bin.len() < 8 {
            bin.insert(0, '0');
        }
        binary.push_str(&bin);
        binary.push(' ');
    }

    let binario_separado = binary.split(" ");

    let mut array_base5 = String::new();

    for n in binario_separado {
        array_base5.push_str(&para_base5(n));
    }

    let mut output = String::new();

    for (_i, c) in array_base5.char_indices() {
        let index = c.to_digit(10).unwrap() as usize;
        output.push(RISADA[index]);
    }
    println!("{}", output);
}

fn decoda_risada(mut risada_encodada: &str) {
    let mut v = vec![];

    let mut string_final = String::new();

    while !risada_encodada.is_empty() {
        //corta string da risada em 3
        let (chunk, rest) = risada_encodada.split_at(cmp::min(3, risada_encodada.len()));
        v.push(chunk);
        risada_encodada = rest;

        let mut vetor_binario = vec![];

        let _string = String::new();

        for letras in &v {
            let binario = para_binario(letras);

            vetor_binario.push(binario);
        }

        for i in vetor_binario {
            let y = isize::from_str_radix(&i, 2).unwrap();

            let ascii = y as u8;
            let character = ascii as char;

            if risada_encodada.is_empty() {
                string_final.push(character);
            }
        }
    }
    println!("{}", string_final);
}

fn para_binario(s: &str) -> String {
    let mut n = 0;
    let mut resultado = String::new();

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
        resultado.push(char::from((n % 2) as u8 + '0' as u8));
        n /= 2;
    }

    resultado.chars().rev().collect()
}

fn para_base5(s: &str) -> String {
    let mut n = 0;
    let mut resultado = String::new();

    for (i, c) in s.chars().rev().enumerate() {
        n += match c {
            '0' => 0,
            '1' => 1 * 2i32.pow(i as u32),
            _ => unreachable!(),
        };
    }

    while n > 0 {
        resultado.push(char::from((n % 5) as u8 + '0' as u8));
        n /= 5;
    }

    resultado.chars().rev().collect()
}

fn main() {
    let args = Args::parse();

    if let Some(ref decode_value) = args.decode {
        println!("Decodificando: {}", decode_value);
        let risada_base5 = valida_risada(&decode_value);
        decoda_risada(&risada_base5);
    }

    if let Some(ref encode_value) = args.encode {
        println!("Codificando: {}", encode_value);
        encoda_risada(&encode_value)
    }
}
