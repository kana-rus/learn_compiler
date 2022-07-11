use std::{str::Chars, collections::VecDeque};

// use crate::parser::strtol;

use super::utils::{
    Int,
};

pub enum TokenKind {
    SymbolToken,
    NumberToken,
    // EOFToken,
} impl TokenKind {
    pub fn _is_symbol(&self) -> bool {
        match self {
            Self::SymbolToken => true,
            _others => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            Self::NumberToken => true,
            _others => false,
        }
    }
    /*
    fn _is_eof(&self) -> bool {
        match self {
            Self::EOFToken => true,
            _others => false,
        }
    }
    */
    fn _clone(&self) -> Self {
        match self {
            Self::SymbolToken => Self::SymbolToken,
            Self::NumberToken => Self::NumberToken,
            // Self::EOFToken    => Self::EOFToken,
        }
    }
}

pub struct Token {
    pub kind: TokenKind,
    // next: Option<Box<Token>>,
    pub value: Option<Int>, // kind が NumberToken のとき使う値
    pub string: String,
} impl Token {
    /*
    fn new(&mut self, kind: TokenKind, string: String, value: Option<Int>) -> Self {
        let new_token = Token {
            kind: kind._clone(),
            string: string.clone(),
            value,
            next: None,
        };
        self.next = Some(Box::new(
            Token {
                kind,
                string,
                value,
                next: None,
            }
        ));
        new_token
    }
    */

    /*
    #[allow(unused_assignments)]
    fn consume(mut token: &Self, expected: char) -> Result<(), ()> {
        if !token.kind._is_symbol() || !token.string.starts_with(expected) {
            Err(())
        } else {
            token = token.next.as_ref().unwrap().as_ref(); //#
            Ok(())
        }
    }
    #[allow(unused_assignments)]
    fn expect(mut token: &Self , expected: char) -> Result<(), String> {
        if !token .kind._is_symbol() || !token.string.starts_with(expected) {
            Err(format!("not {}", expected))
        } else {
            token = token.next.as_ref().unwrap().as_ref(); //# 
            Ok(())
        }
    }
    #[allow(unused_assignments)]
    fn expect_number(mut token: &Self) -> Result<Int, String> {
        if !token.kind._is_number() {
            Err(format!("{} is not a number", token.string))
        } else {
            let val = token.value.expect("no value was found");
            token = token.next.as_ref().unwrap().as_ref(); //#
            Ok(val)
        }
    }
    */
}

pub fn tokenize(mut input: Chars) /*-> Token*/ -> /*Vec<Token>*/VecDeque<Token> {
    let mut token_vec = VecDeque::new();
    // let mut head = Token {
    //     kind: TokenKind::SymbolToken,
    //     next: None,
    //     value: None,
    //     string: String::new(),
    // };
    // let mut current = head;

    let mut next_step = input.next();

    while next_step.is_some() {
        let next_char = next_step.unwrap();

        if next_char.is_whitespace() {
            // do nothing
            next_step = input.next();

        } else if next_char.is_ascii_digit() {
            // let (rem_digits, next) = strtol(&mut input);
            let mut digits_vec = vec![next_char.to_digit(10).unwrap()];
            let mut next = input.next();
            while next.is_some() {
                let next_char = next.unwrap(); if !next_char.is_ascii_digit() { break; }
                digits_vec.push(next_char.to_digit(10).unwrap());
                next = input.next();
            }
            let digits_as_num = digits_vec.iter().fold(0, |a, b| 10*a + b);
            /*
            let value_str = match rem_digits {
                None => next_char.to_string(),
                Some(rem_digits_as_number) => next_char.to_string() + &rem_digits_as_number.to_string(),
            };
            */
            // current = current.new(TokenKind::NumberToken, value_str.clone(), Some(value_str.parse::<Int>().unwrap()));
            token_vec.push_back(Token{
                kind: TokenKind::NumberToken,
                string: digits_as_num.to_string(),
                value: Some(digits_as_num),
            });
            next_step = next;

        } else if next_char == '+' || next_char == '-' {
            // current = current.new(TokenKind::SymbolToken, next_char.to_string(), None);
            token_vec.push_back(Token{
                kind: TokenKind::SymbolToken,
                string: next_char.to_string(),
                value: None,
            });
            next_step = input.next();

        } else {
            println!("unexpected charactor: \'{}\'", next_char); panic!();
        }
    }

    // _ = current.new(TokenKind::EOFToken, String::new(), None);
    token_vec
}
