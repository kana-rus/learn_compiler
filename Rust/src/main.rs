// compile +, - expression

use std::{env, str::Chars};
mod utils;

use utils::{
    report_unexpected_token,
    strtol, // StrtolError,

};

fn main() {
    let mut input: Chars;
        let buff; input = {
        let mut args = env::args();
        buff = args.nth(1).expect("input is in need!");
        if args.next().is_some() { println!("only 1 argument is in need!"); panic!(); }
        buff.chars()
    };

    let (mut num, mut next_char): (u32, Option<char>);

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

    let mut output = format!("
.intel_syntax noprefix
.global main
main:
    mov rax, {}\n", num);

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

    println!("{}", output);
}
