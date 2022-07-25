mod utils; use utils::*;
use std::collections::vec_deque::IntoIter;

/* STEP.4
 * expr ::= mul (('+'|'-') mul)*
 * mul ::= prim (('*'|'/') prim)*
 * prim ::= num | '(' expr ')'
 */

impl Parse for Arith {
    fn parse(mut tokens: std::collections::vec_deque::IntoIter<Token>) -> Tree {
        let mut arith = Arith { current: /*Token::Init*/tokens.next().expect("no input given") };
        let /*mut*/ root = expr(&mut arith, &mut tokens);
        /*
        while root.elem.is_init() && root.right.is_none() {
            root = *root.left.unwrap();
        }
        */
        Tree { root }
    }
}

fn expr(arith: &mut Arith, tokens: &mut IntoIter<Token>) -> Node {
//    println!("[arith] expr1 {:?}", arith.current);//
    let mut node = mul(arith, tokens);
//    println!("[node] expr1 {:?}", &node);
    //let next = tokens.next(); if next.is_none() { return node; }
    //arith.current = next.unwrap();
//    println!("[arith] expr2 {:?}", arith.current);//
    loop {
        match arith.current {
            Token::Ope('+') => {
                arith.current = tokens.next().expect("\"+\": no righthand given");
                node = Node {
                    elem: Elem::Ope('+'),
                    left: link(node),
                    right: link(mul(arith, tokens)),
                }
            },
            Token::Ope('-') => {
                arith.current = tokens.next().expect("\"-\": no righthand given");
                node = Node {
                    elem: Elem::Ope('-'),
                    left: link(node),
                    right: link(mul(arith, tokens)),
                }
            },
            _ => return node,
        }
    }
}

fn mul(arith: &mut Arith, tokens: &mut IntoIter<Token>) -> Node {
//    println!("[arith] mul1 {:?}", arith.current);//
    let mut node = prim(arith, tokens);
//    println!("[node] mul1 {:?}", &node);
//    let next = tokens.next(); if next.is_none() { return node; }
//    arith.current = next.unwrap();
//    println!("[arith] mul2 {:?}", arith.current);//
    loop {
        match arith.current {
            Token::Ope('*') => {
                arith.current = tokens.next().expect("\"*\": no righthand given");
                node = Node {
                    elem: Elem::Ope('*'),
                    left: link(node),
                    right: link(prim(arith, tokens)),
                }
            },
            Token::Ope('/') => {
                arith.current = tokens.next().expect("\"/\": no righthand given");
                node = Node {
                    elem: Elem::Ope('/'),
                    left: link(node),
                    right: link(prim(arith, tokens)),
                }
            },
            _ => return node,
        }
    }
}

fn prim(arith: &mut Arith, tokens: &mut IntoIter<Token>) -> Node {
//    println!("[arith] prim1 {:?}", arith.current);//
    match arith.current {
        Token::Num(n) => {
            let next = tokens.next();
            if next.is_some() { arith.current = next.unwrap(); }
            Node {
                elem: Elem::Num(n),
                left: None,
                right: None,
            }
        },
        Token::PrimOpen => {
            arith.current = tokens.next().expect("no \")\"");
            let node = expr(arith, tokens);
//    println!("[node] prim1 {:?}", &node);
//    println!("[arith] prim2 {:?}", arith.current);//
            
            if !arith.current.is_prim_close() {
                exit_with_report("no \")\"");
            }
            if let Some(next) = tokens.next() {
                arith.current = next;
            }
            // if tokens.next().expect("no close").expect_ope() != ')' { exit_with_report("no \")\" found"); }
            return node;
        },

        Token::Ope(o) => exit_with_report(format!("unexpected token: {}", o)),
        Token::PrimClose => exit_with_report("unexpected token: )"),
        Token::Init => exit_with_report("exexpectedly found Init token"),
    }
}


#[test]
fn input_1pl2_ml_3() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("(1+2) * 3")
                )
            )
        ),
        (1 + 2) * 3
    );
}
#[test]
fn input1pl__2pl3_ml4_ml5() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("1 + ((2+3)*4) * 5")
                )
            )
        ),
        1 + ((2 + 3) * 4) * 5
    );
}
#[test]
fn input_1pl2pl3_ml_4pl5pl6() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("(1+2+3) * (4+5+6)")
                )
            )
        ),
        (1+2+3) * (4+5+6)
    );
}

fn main() { /* printing debug */
    println!("\n(1+2)*3:\n{:?}",
        Arith::parse(
            Arith::tokenize(
                String::from("(1+2) * 3")
            )
        )
    );
    println!("\n(1+2+3)*(4+5+6):\n{:?}",
        Arith::parse(
            Arith::tokenize(
                String::from("(1+2+3) * (4+5+6)")
            )
        )
    );
}
