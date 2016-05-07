use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Token{
    //字面量
    STR(String),
    INT(String),
    //符号
    EQ,
    COMMA,
    LPAR,
    RPAR,
    LSQB,
    RSQB,
    LBRACE,
    RBRACE,
    IDEN(String),
    LF,
    //关键字
    VAR,
    IF,
    WHILE,
    //FOR,
    //错误
    ERR(String),
}

impl Token {
    pub fn print(self){
        match self {
            Token::STR(s) => { print!("  str:{}",s); },
            Token::INT(s) => { print!("  int:{}",s); },
            Token::EQ => { print!("  ="); },
            Token::COMMA => { print!("  ,"); },
            Token::LPAR => { print!("  ("); },
            Token::RPAR => { print!("  )"); },
            Token::LSQB => { print!("  ["); },
            Token::RSQB => { print!("  ]"); },
            Token::LBRACE => { print!("  {{"); },
            Token::RBRACE => { print!("  }}"); },
            Token::IDEN(s) => { print!("  iden:{}",s); },
            Token::LF => { print!("  "); },
            Token::VAR => { print!("  var"); },
            Token::IF => { print!("  if"); },
            Token::WHILE => { print!("  while"); },
            Token::ERR(s) => { print!("  error:{}",s); },
            
        }
    }
}

struct ReadToken{
    i:usize,
    char_vec:Vec<char>,
}

fn is_iden_start(c:char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '_' => true,
        _ => false,
    }
}

fn is_iden(c:char) -> bool {
    match c {
        '0'...'9' | 'a'...'z' | 'A'...'Z' | '_' => true,
        _ => false,
    }
}

fn is_num(c:char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false,
    }
}
    
impl ReadToken {

    fn now_char(&self) -> Option<char> {
        if self.i<self.char_vec.len() {Some(self.char_vec[self.i])}
        else { None }
    }
    
    fn goto_useful(&mut self) {
        loop {
            if let Some(now) = self.now_char() {
                if now == '\r'
                || now == ' '
                || now == '\t' {
                    self.i += 1;
                }else{
                    break;
                }
            }
            else{
                break;
            }
        }
    }
    
    fn next(&mut self) -> Option<Token>{
        self.goto_useful();
        let mut tmp_str = String::new();
        match self.now_char() {
            Some(c) => {
                if is_iden_start(c) {
                    self.i += 1;
                    tmp_str.push(c);
                    loop {
                        match self.now_char() {
                            Some(c2) => {
                                if is_iden(c2) {
                                    self.i += 1;
                                    tmp_str.push(c2);
                                }
                                else { break; }
                            },
                            _ => {break;},
                        }
                    }
                    let keywords = get_keywords();
                    match keywords.get(&*tmp_str) {
                        Some(k) => {
                            return Some(k.clone());
                        },
                        None => {
                            let t = Token::IDEN(tmp_str);
                            return Some(t);
                        },
                    }
                }
                else {
                    match c {
                        '=' => { self.i += 1; return Some(Token::EQ) }
                        '(' => { self.i += 1; return Some(Token::LPAR); },
                        ')' => { self.i += 1; return Some(Token::RPAR); },
                        '[' => { self.i += 1; return Some(Token::LSQB); },
                        ']' => { self.i += 1; return Some(Token::RSQB); },
                        '{' => { self.i += 1; return Some(Token::LBRACE); },
                        '}' => { self.i += 1; return Some(Token::RBRACE); },
                        ',' => { self.i += 1; return Some(Token::COMMA); },
                        '\n' => { self.i += 1;return Some(Token::LF); },
                         _  => { self.i += 1;return Some(Token::ERR("".to_string())); },
                    }
                }
            },
            None => { return None; },
        }
    }
    
}

pub fn get_keywords() -> HashMap<&'static str,Token> {
    let mut keywords = HashMap::new();
    keywords.insert("var",Token::VAR);
    keywords.insert("if",Token::IF);
    keywords.insert("while",Token::WHILE);
    keywords
}

pub fn get_char_vec(path:&Path) -> Vec<char> {
    let mut f = match File::open(path){
        Ok(r) => r,
        _ => panic!("can't open file !"),
    }; 
    let mut buffer = String::new();
    match f.read_to_string(&mut buffer){
        Ok(r) => r,
        _ => panic!("can't convent file to string !"),
    };
    let str_vec: Vec<char> = buffer.chars().collect();
    str_vec
}

pub fn get_tokens_from(path:&Path) -> Vec<Token>{
    let char_vec = get_char_vec(path);
    let mut tokens: Vec<Token> = Vec::new();
    let mut read_token = ReadToken{i:0,char_vec:char_vec};
    loop {
        let t = read_token.next();
        match t {
            Some(t) => {
                tokens.push(t);
            },
            None => {break;},
        }
    }
    tokens
}



