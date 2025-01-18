use std::{fs::OpenOptions, io::Read, path::Path};

use tracing_subscriber::FmtSubscriber;

#[derive(Debug)]
enum VarType {
    Int(i64),
    Uint(u64),
    Bool(bool),
    String(String),
}

enum ParserState {
    Let,
    Var,
    Equal,
    Assignment,
}

#[derive(Debug)]
struct Parser {
    variables: Vec<(String, VarType)>,
}

impl Parser {
    fn push_variable(&mut self, var: (String, VarType)) {
        self.variables.push(var);
    }

    pub fn parse(&mut self, source: &mut impl Read) {
        let mut buff = String::default();
        source
            .read_to_string(&mut buff)
            .expect("failed to read from source");
        let mut state = ParserState::Let;
        let mut state_buff: Vec<char> = vec![];
        for (i, ch) in buff.chars().enumerate() {
            match state {
                ParserState::Let => {
                    match ch {
                        'l' | 'e' | 't' => {
                            state_buff.push(ch);
                        }
                        _ => {}
                    }
                    if state_buff.iter().collect::<String>() == "let" {
                        state_buff.clear();
                        state = ParserState::Var;
                    }
                }
                ParserState::Var => {
                    match ch {
                        ' ' => {
                            if !state_buff.is_empty() {
                                state = ParserState::Equal;
                            }
                        }
                        _ => {
                            state_buff.push(ch);
                        }
                    }
                    println!("in var state");
                }
                ParserState::Equal => {
                    match ch {
                        '=' => {
                            state = ParserState::Assignment;
                        }
                        ' ' => {}
                        _ => {
                            panic!("found {ch}");
                        }
                    }
                    println!("in equal state");
                }
                ParserState::Assignment => {
                    if state_buff.is_empty() {
                        continue;
                    }

                    let mut num_buff: Vec<char> = vec![];
                    for ch in buff[i..].chars() {
                        match ch {
                            ' ' | '\n' => {}
                            '0'..='9' => {
                                num_buff.push(ch);
                            }
                            ';' => break,
                            _ => todo!("supporting anything besides `let <var> = <u64>`"),
                        }
                    }

                    if num_buff.is_empty() {
                        continue;
                    }

                    let var = num_buff.iter().collect::<String>().parse().unwrap();
                    self.push_variable((state_buff.iter().collect::<String>(), VarType::Uint(var)));
                    state = ParserState::Let;
                    state_buff.clear();
                }
            }
        }
    }
}

fn main() {
    setup_logger("parser_log");
    let mut source = std::fs::OpenOptions::new()
        .read(true)
        .open("./examples/example_1.txt")
        .unwrap();
    let mut parser = Parser { variables: vec![] };
    parser.parse(&mut source);
    println!("{parser:?}");
}

fn setup_logger<P: AsRef<Path>>(file: P) {
    _ = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file.as_ref())
        .expect("truncating log file failed");

    let appender = tracing_appender::rolling::never(".", file);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(appender)
        .with_ansi(false)
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);
}
