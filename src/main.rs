use std::io::{self, BufWriter, Read, Stdout, Write};
use std::{env, fs};

fn main() {
    let mut buf = BufWriter::new(io::stdout());
    run(program(), &mut buf);
    buf.write_all(&[b'\n']).unwrap();
    buf.flush().unwrap();
}

fn program() -> Vec<char> {
    let commands = vec!['>', '<', '+', '-', '.', ',', '[', ']'];
    let args: Vec<_> = env::args().collect();
    fs::read_to_string(&args[1])
        .unwrap_or_else(|_| String::from(&args[1]))
        .chars()
        .filter(|c| commands.contains(c))
        .collect()
}

fn run(program: Vec<char>, buf: &mut BufWriter<Stdout>) {
    let mut data: Vec<u8> = vec![0; 30000];
    let mut prog_pointer = 0;
    let mut data_pointer = 0;
    while prog_pointer < program.len() {
        match program[prog_pointer] {
            '>' => {
                if data_pointer < data.len() {
                    data_pointer += 1
                }
            }
            '<' => {
                if data_pointer > 0 {
                    data_pointer -= 1
                }
            }
            '+' => {
                if data[data_pointer] < 255 {
                    data[data_pointer] += 1
                }
            }
            '-' => {
                if data[data_pointer] > 0 {
                    data[data_pointer] -= 1
                }
            }
            '.' => {
                buf.write_all(&[data[data_pointer]]).unwrap();
                if !cfg!(test) {
                    buf.flush().unwrap()
                };
            }
            ',' => {
                println!("Input a char: ");
                let input = io::stdin()
                    .bytes()
                    .next()
                    .and_then(|res| res.ok())
                    .unwrap();
                data[data_pointer] = input;
            }
            '[' => {
                if data[data_pointer] == 0 {
                    let mut open_braces = 1;
                    while open_braces > 0 {
                        prog_pointer += 1;
                        if program[prog_pointer] == '[' {
                            open_braces += 1
                        }
                        if program[prog_pointer] == ']' {
                            open_braces -= 1
                        }
                    }
                }
            }
            ']' => {
                // we don't need to check data[data_pointer] because the matching '['
                // will skip the instruction if data[data_pointer] is zero anyway
                let mut open_braces = 1;
                while open_braces > 0 {
                    prog_pointer -= 1;
                    if program[prog_pointer] == ']' {
                        open_braces += 1
                    }
                    if program[prog_pointer] == '[' {
                        open_braces -= 1
                    }
                }
                prog_pointer -= 1;
            }
            _ => {}
        }
        // increment the program pointer every loop
        prog_pointer += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_a_dollar_sign() {
        let program: Vec<char> = "++++++++++++++++++++++++++++++++++++.".chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        assert_eq!('$', *buf.buffer().first().unwrap() as char);
    }

    #[test]
    fn it_prints_hello_world() {
        let s1 = "++++++++++[>+++++++>++++++++++>+++>+";
        let s2 = "<<<<-]>++.>+.+++++++..+++.>++.<<++++";
        let s3 = "+++++++++++.>.+++.------.--------.>+.>.";
        let program: Vec<char> = format!("{s1}{s2}{s3}").chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        let (data, _) = buf.buffer().split_at(12);
        let data: String = data.iter().map(|byte| *byte as char).collect();
        assert_eq!(String::from("Hello World!"), data);
    }

    #[test]
    fn it_prints_0_to_99() {
        let file = fs::read_to_string("tests/test1.b").unwrap();
        let program: Vec<char> = file.chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        let mut s = String::new();
        buf.buffer().read_to_string(&mut s).unwrap();
        let mut expected = String::from("00\n01\n02\n03\n04\n05\n06\n07\n08\n09\n");
        for i in 10..=99 {
            let next_num = format!("{i}\n");
            expected.push_str(&next_num);
        }
        assert_eq!(expected, s);
    }

    #[test]
    #[ignore] // has a super long output
    fn it_prints_0_to_999() {
        let file = fs::read_to_string("tests/test2.b").unwrap();
        let program: Vec<char> = file.chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        let mut s = String::new();
        buf.buffer().read_to_string(&mut s).unwrap();
        let mut expected = String::new();
        for i in 0..=9 {
            let next_num = format!("00{i}\n");
            expected.push_str(&next_num);
        }
        for i in 10..=99 {
            let next_num = format!("0{i}\n");
            expected.push_str(&next_num);
        }
        for i in 100..=999 {
            let next_num = format!("{i}\n");
            expected.push_str(&next_num);
        }
        assert_eq!(expected, s);
    }

    #[test]
    fn it_wont_overflow() {
        let file = fs::read_to_string("tests/test4.b").unwrap();
        let program: Vec<char> = file.chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        assert_eq!(255, *buf.buffer().first().unwrap());
    }
}
