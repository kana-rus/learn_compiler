// compile +, - expression

use std::{env, str::Chars};


mod utils; /*use utils::{
    Int,
};
*/
mod parser; use parser::{
    // strtol,
    report_unexpected_token,
};
mod tokenizer; use tokenizer::{
    TokenKind,
    tokenize,
};

fn main() {
    let input: Chars;
        let buff; input = {
        let mut args = env::args();
        buff = args.nth(1).expect("input is in need!");
        if args.next().is_some() { println!("only 1 argument is in need!"); panic!(); }
        buff.chars()
    };

    /*
    let (mut num, mut next_char): (Int, Option<char>);

    match strtol(&mut input) {
        (Some(ret_num), Some(ret_op)) => {
            num = ret_num;
            next_char  = Some(ret_op);
        },
        (Some(ret_num), None) => {
            num = ret_num;
            next_char  = None;
        },
        (None, _) => { println!("here expression has to start with a number"); panic!(); }
    }
    */
    let mut token_queue = tokenize(input);

    let first_token = token_queue.pop_front().expect("more than 1 token is need");
    if !first_token.kind.is_number() {
        println!("here expression has to start with a number");
        panic!();
    }

    let mut output = format!("
.intel_syntax noprefix
.global main
main:
    mov rax, {}\n", first_token.value.unwrap());

    /*
    while next_char.is_some() {
        let op = next_char.unwrap();
        match strtol(&mut input) {
            (None, _) => report_unexpected_token(op),
            (Some(ret_num), ret_op) => {
                num = ret_num;
                match op {
                    '+' => output += &format!("    add rax, {}\n", num),
                    '-' => output += &format!("    sub rax, {}\n", num),
                    other => report_unexpected_token(other),
                }
                next_char = ret_op;
            },
        }
    }
    */
    let mut parse_state = TokenKind::NumberToken;
    let mut next_operation = String::new();
    while !token_queue.is_empty() {
        let token = token_queue.pop_front().unwrap();
        match token.kind {
            TokenKind::SymbolToken => {
                if parse_state._is_symbol() { report_unexpected_token(&token.string); }
                parse_state = TokenKind::SymbolToken;

                match token.string.as_str() {
                    "+" => next_operation += "    add rax, ",
                    "-" => next_operation += "    sub rax, ",
                    _other => report_unexpected_token(token.string),
                }
            },
            TokenKind::NumberToken => {
                if parse_state.is_number() { report_unexpected_token(&token.string); }
                parse_state = TokenKind::NumberToken;

                next_operation += &token.string;
                next_operation += "\n";

                output += &next_operation;
                next_operation.clear();
            },
        }
    }

    println!("{}", output);
}

