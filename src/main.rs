use std::io::{self, BufWriter, Stdout, Write};
use std::{env, fs, process};

fn main() {
    let mut buf = BufWriter::new(io::stdout());
    run(program(), &mut buf);
    buf.write_all(&[b'\n']).unwrap(); // newline for formatting
    buf.flush().unwrap();
}

fn program() -> Vec<char> {
    let commands = vec!['>', '<', '+', '-', '.', ',', '[', ']'];
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("USAGE:    bfi <INPUT>");
        println!("  `bfi -h` or `bfi --help` for help");
        process::exit(0);
    }
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!("bfi - a Brainfuck Interpreter");
        println!("  author: benharmonics");
        println!("  https://github.com/benharmonics/bfi");
        println!("USAGE:");
        println!("          bfi <FILE> to read from a file");
        println!("          bfi '<PROG>' to input a program directly");
        println!("EXAMPLES:");
        println!("        > bfi '+++++++[>+++++++<-]>+++.--.--.'");
        println!("  output: 420");
        println!("        > echo '++++++++[>+++++++<-]>++.-------.' > file.b");
        println!("        > bfi file.b");
        println!("  output: :3");
        process::exit(0);
    }
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
                if data_pointer < data.len() - 1 {
                    data_pointer += 1
                } else {
                    data_pointer = 0
                }
            }
            '<' => {
                if data_pointer > 0 {
                    data_pointer -= 1
                } else {
                    data_pointer = data.len() - 1
                }
            }
            '+' => {
                if data[data_pointer] < 255 {
                    data[data_pointer] += 1
                } else {
                    data[data_pointer] = 0
                }
            }
            '-' => {
                if data[data_pointer] > 0 {
                    data[data_pointer] -= 1
                } else {
                    data[data_pointer] = 255
                }
            }
            '.' => {
                buf.write_all(&[data[data_pointer]]).unwrap();
                // tests require that the buffer be flushed all at once
                // is this a good way to implement this?
                if !cfg!(test) {
                    buf.flush().unwrap()
                }
            }
            ',' => {
                buf.write_all(b"Input a char: ").unwrap();
                buf.flush().unwrap();
                let mut line = String::new();
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line.");
                let byte = line
                    .chars()
                    .next()
                    .unwrap_or_else(|| char::from(data[data_pointer]));
                data[data_pointer] = byte as u8
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
                prog_pointer -= 1
            }
            _ => {}
        }
        // increment the program pointer every loop
        prog_pointer += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use io::Read;

    #[test]
    fn it_prints_a_dollar_sign() {
        // dollar sign is ASCII 36
        let program = "++++++++++++++++++++++++++++++++++++.".chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        assert_eq!('$', *buf.buffer().first().unwrap() as char);
    }

    #[test]
    fn it_prints_hello_world() {
        // https://tnu.me/brainfuck/generator
        let s1 = "++++++++++[>+++++++>++++++++++>+++>+";
        let s2 = "<<<<-]>++.>+.+++++++..+++.>++.<<++++";
        let s3 = "+++++++++++.>.+++.------.--------.>+.>.";
        let program = format!("{s1}{s2}{s3}").chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        let (data, _) = buf.buffer().split_at(12);
        let received: String = data.iter().map(|byte| char::from(*byte)).collect();
        assert_eq!(String::from("Hello World!"), received);
    }

    #[test]
    fn it_reads_nested_loops() {
        // prints 'hello world'... don't ask me to explain it
        let s = "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.";
        let program = String::from(s).chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        let (data, _) = buf.buffer().split_at(11);
        let received: String = data.iter().map(|byte| char::from(*byte)).collect();
        assert_eq!(String::from("hello world"), received);
    }

    #[test]
    #[ignore] // has a super long output
    fn it_prints_0_to_99() {
        let file = fs::read_to_string("tests/test1.b").unwrap();
        let program = file.chars().collect();
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
        let program = file.chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
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
        let mut s = String::new();
        buf.buffer().read_to_string(&mut s).unwrap();
        assert_eq!(expected, s);
    }

    #[test]
    fn it_will_overflow() {
        // the file increments cell 0 256 times and prints the value,
        // which should be 0 because 255 + 1 should overflow
        let file = fs::read_to_string("tests/test3.b").unwrap();
        let program = file.chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        assert_eq!(0, *buf.buffer().first().unwrap());
    }

    #[test]
    fn it_will_underflow() {
        // subtracting 1 from 0 should underflow yielding 255
        let program = "-.".chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        assert_eq!(255, *buf.buffer().first().unwrap());
    }

    #[test]
    fn it_wraps_around() {
        // moving left from cell 0 should put you in cell 29999
        // moving right from cell 29999 should put you in cell 0
        let file = fs::read_to_string("tests/test4.b").unwrap();
        let program = file.chars().collect();
        let mut buf = BufWriter::new(io::stdout());
        run(program, &mut buf);
        let (data, _) = buf.buffer().split_at(3);
        let received: String = data.iter().map(|byte| char::from(*byte)).collect();
        assert_eq!(String::from("!!!"), received);
    }
}
