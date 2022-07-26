use crate::{arith::*, tree::*, utils::*};
use std::collections::vec_deque::IntoIter;


impl Parse for Arith { /* default */ }

pub struct Equa {
    left: Arith,
    right: Arith,
} impl Equa {

    pub fn parse(&mut tokens: IntoIter<Token>) -> Self {
        let left = Arith::parse(tokens);

    }

    pub fn evaluate() -> bool {

    }
}

fn equation(arith: Arith, tokens: IntoIter<Token>) -> Node {
    let left = expr(&mut arith, &mut tokens);
    match arith.current {
        Token::Ope("=") => arith.current = tokens.next().expect("\"=\": no righthand found"),
        _ => exit_with_report("no '='"),
    }
    let right = expr(&mut arith, &mut tokens);
    
    assert(tokens.is_empty(), format!("remaining: {} ~", tokens.next().unwrap()));

    Node {
        elem: Elem::Ope("="),
        left: link(left),
        right: link(left),
    }
}
