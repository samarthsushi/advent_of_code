use std::fs;

#[derive(Debug)]
enum Token {
    Mul(i32, i32),
    Do,
    Dont,
}

struct Parsec<'a> {
    evaluate: bool,
    input: &'a str,
    cursor: usize
}

impl<'a> Parsec<'a> {
    fn new(input: &'a str) -> Self {
        Self { evaluate: true, input, cursor: 0 }
    }

    fn skip_meaningless(&mut self) {
        while let Some(c) = self.peek_char() {
            if c != 'm' && c != 'd' {
                self.cursor += c.len_utf8();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        let c = self.input[self.cursor..].chars().next();
        return c;
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_meaningless();
        
        if let Some(c) = self.peek_char() {
            match c {
                'm' => {
                    self.cursor += c.len_utf8();
                    if self.peek_char() == Some('u') {
                        self.cursor += 'u'.len_utf8();
                        if self.peek_char() == Some('l') {
                            self.cursor += 'l'.len_utf8();
                            if self.peek_char() == Some('(') {
                                self.cursor += '('.len_utf8();
                                let mut num1 = String::new();
                                while let Some(c) = self.peek_char() {
                                    if c.is_ascii_digit() {
                                        num1.push(c);
                                        self.cursor += c.len_utf8();
                                    } else {
                                        break;
                                    }
                                }
                                let mut num2 = String::new();
                                if self.peek_char() == Some(',') {
                                    self.cursor += ','.len_utf8();
                                    while let Some(c) = self.peek_char() {
                                        if c.is_ascii_digit() {
                                            num2.push(c);
                                            self.cursor += c.len_utf8();
                                        } else {
                                            break;
                                        }
                                    }
                                }

                                if self.peek_char() == Some(')') {
                                    self.cursor += ')'.len_utf8();
                                    if !self.evaluate {
                                        return None;
                                    }
                                    if num1.len() < 1 || num1.len() > 3 || num2.len() < 1 || num2.len() > 3 {
                                        return None;
                                    }
                                    if let (Ok(x), Ok(y)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
                                        return Some(Token::Mul(x, y));
                                    }
                                }
                            }
                        }
                    }
                    None
                }
                'd' => {
                    self.cursor += c.len_utf8();
                    if self.peek_char() == Some('o') {
                        self.cursor += 'o'.len_utf8();
                        if self.input[self.cursor..].starts_with("n't") {
                            self.cursor += "n't".len();
                            self.evaluate = false;
                            return Some(Token::Dont);
                        } else {
                            self.evaluate = true;
                            return Some(Token::Do);
                        }
                    }
                    None
                }
                _ => { 
                    eprintln!("should have encountered an m or d"); 
                    None 
                }
            }
        } else {
            None
        }
    }
}

fn main() {
    let file_path = "data/data.txt";
    let s = fs::read_to_string(file_path).unwrap();
    let len = s.len();
    let mut parsec = Parsec::new(&s);
    let mut sum = 0;

    while parsec.cursor < len {
        if let Some(token) = parsec.next_token() {
            match token {
                Token::Mul(x,y) => {
                    sum += x * y;
                },
                _ => {
                    continue;
                }
            }
        }
    }

    println!("{sum}");
}