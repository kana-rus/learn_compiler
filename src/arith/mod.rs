use crate::{utils::*, tree::*};
use std::collections::vec_deque::{VecDeque, IntoIter};


pub struct Arith {
    pub current: Token,
}
pub trait Parse {

    fn tokenize(input: String) -> IntoIter<Token> {
        let mut input = input.chars();
        let mut digits = VecDeque::<Int>::new();
        let mut ret = VecDeque::<Token>::new();
        let mut prim_state = 0;

        let mut current = input.next()/*.expect("no input given")*/;
        // let mut next = input.next();
        loop {
            match current {
                Some(char) => {
                    match char.to_digit(10) {
                        Some(d) => {
                            digits.push_back(d);
                            current = input.next();//
                        },
                        None => {
                            if !digits.is_empty() {
                                ret.push_back(Token::Num(build_from(&digits)));
                                digits.clear();
                                // current = input.next();//
                            }
                            match char {
                                '+' => {
                                    current = input.next();
                                    while current.is_some() && current.unwrap() == ' ' {
                                        current = input.next();
                                    }

                                    match current.expect("\"+\": no righthand given") {
                                        '=' => {
                                            ret.push_back(Token::Ope("+="));
                                            current = input.next();//
                                        },
                                         _  => ret.push_back(Token::Ope("+")),
                                    }
                                },
                                '-' => {
                                    current = input.next();
                                    while current.is_some() && current.unwrap() == ' ' {
                                        current = input.next();
                                    }

                                    match current.expect("\"-\": no righthand given") {
                                        '=' => {
                                            ret.push_back(Token::Ope("-="));
                                            current = input.next();//
                                        },
                                         _  => ret.push_back(Token::Ope("-")),
                                    }
                                },
                                '*' => {
                                    current = input.next();
                                    while current.is_some() && current.unwrap() == ' ' {
                                        current = input.next();
                                    }

                                    match current.expect("\"*\": no righthand given") {
                                        '=' => {
                                            ret.push_back(Token::Ope("*="));
                                            current = input.next();//
                                        },
                                         _  => ret.push_back(Token::Ope("*")),
                                    }
                                },
                                '/' => {
                                    current = input.next();
                                    while current.is_some() && current.unwrap() == ' ' {
                                        current = input.next();
                                    }

                                    match current.expect("\"/\": no righthand given") {
                                        '=' => {
                                            ret.push_back(Token::Ope("/="));
                                            current = input.next();//
                                        },
                                         _  => ret.push_back(Token::Ope("/")),
                                    }
                                },
                                '=' => {
                                    current = input.next();
                                    while current.is_some() && current.unwrap() == ' ' {
                                        current = input.next();
                                    }

                                    match current {
                                        None => ret.push_back(Token::Ope("=")),
                                        Some('=') => {
                                            ret.push_back(Token::Ope("=="));
                                            current = input.next();
                                        },
                                        _ => current = input.next(),
                                    }
                                },
                                '(' => {
                                    ret.push_back(Token::PrimOpen);
                                    prim_state += 1;
                                    current = input.next();//
                                },
                                ')' => {
                                    ret.push_back(Token::PrimClose);
                                    prim_state -= 1;
                                    assert(prim_state >= 0, "unexpected token: \")\"");
                                    current = input.next();//
                                },
                                ' '                   => current = input.next(),//
                                 _                    => { println!("unexpected: {}", char); panic!(); }
                            }
                        },
                    }
                },
                None => {
                    if !digits.is_empty() { ret.push_back(Token::Num(build_from(&digits))); }
                    break;
                },
            }
        }
        assert(prim_state == 0, "\"(\" and \")\" not match");
        ret.into_iter()
    }

    fn parse(mut tokens: IntoIter<Token>) -> Tree {
        let mut arith = Arith { current: /*Token::Init*/tokens.next().expect("no input given") };
        let /*mut*/ root = expr(&mut arith, &mut tokens);
        Tree { root }
    }

    fn evaluate(tree: Tree) -> Int {
        let root_node = tree.root;
        let (op, lh, rh) = (
            root_node.elem,
            root_node.left,
            root_node.right
        );
        calc(op, lh, rh)
    }
}

#[allow(unused)]
fn calc(op: Elem, lh: Link<Node>, rh: Link<Node>) -> Int {
    match op {
        Elem::Init => exit_with_report("found Init elem"),
        Elem::Num(number) => number,
        Elem::Ope(operator) => {
            let (lh, rh) = (
                lh.expect("left Node not exists"),
                rh.expect("right Node not exists")
            );
            let left_num = match lh.elem {
                Elem::Init => exit_with_report("found Init elem"),
                Elem::Num(number) => number,
                Elem::Ope(lop) => calc(
                    Elem::Ope(lop), lh.left, lh.right
                ),
            };
            let right_num = match rh.elem {
                Elem::Init =>exit_with_report("found Init elem"),
                Elem::Num(number) => number,
                Elem::Ope(rop) => calc(
                    Elem::Ope(rop), rh.left, rh.right
                ),
            };

            match operator {
                "+" => left_num + right_num,
                "-" => left_num - right_num,
                "*" => left_num * right_num,
                "/" => left_num / right_num,
                 _  => { println!("{} is not Ope", operator); panic!(); },
            }
        },
    }
}
pub fn expr(arith: &mut Arith, tokens: &mut IntoIter<Token>) -> Node {
    let mut node = mul(arith, tokens);
    loop {
        match arith.current {
            Token::Ope("+") => {
                arith.current = tokens.next().expect("\"+\": no righthand given");
                node = Node {
                    elem: Elem::Ope("+"),
                    left: link(node),
                    right: link(mul(arith, tokens)),
                }
            },
            Token::Ope("-") => {
                arith.current = tokens.next().expect("\"-\": no righthand given");
                node = Node {
                    elem: Elem::Ope("-"),
                    left: link(node),
                    right: link(mul(arith, tokens)),
                }
            },
            _ => return node,
        }
    }
}

fn mul(arith: &mut Arith, tokens: &mut IntoIter<Token>) -> Node {
    let mut node = prim(arith, tokens);
    loop {
        match arith.current {
            Token::Ope("*") => {
                arith.current = tokens.next().expect("\"*\": no righthand given");
                node = Node {
                    elem: Elem::Ope("*"),
                    left: link(node),
                    right: link(prim(arith, tokens)),
                }
            },
            Token::Ope("/") => {
                arith.current = tokens.next().expect("\"/\": no righthand given");
                node = Node {
                    elem: Elem::Ope("/"),
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
            
            if !arith.current.is_prim_close() {
                exit_with_report("no \")\"");
            }
            if let Some(next) = tokens.next() {
                arith.current = next;
            }
            return node;
        },

        Token::Ope(o) => exit_with_report(format!("unexpected token: {}", o)),
        Token::PrimClose => exit_with_report("unexpected token: )"),
        // Token::Init => exit_with_report("exexpectedly found Init token"),
    }
}
