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

        let mut current = input.next();
        loop {
            match current {
                Some(char) => {
                    match char.to_digit(10) {
                        Some(d) => {
                            digits.push_back(d);
                        },
                        None => {
                            if !digits.is_empty() {
                                ret.push_back(Token::Num(build_from(&digits)));
                                digits.clear();
                            }
                            match char {
                                '+' | '-' | '*' | '/' => ret.push_back(Token::Ope(char)),
                                '('                   => { ret.push_back(Token::PrimOpen);
                                    prim_state += 1;
                                },
                                ')'                   => { ret.push_back(Token::PrimClose);
                                    prim_state -= 1;
                                    assert(prim_state >= 0, "unexpected token: \")\"");
                                },
                                ' '                   => (),
                                 _                    => { println!("unexpected: {}", char); panic!(); }
                            }
                        },
                    }
                    current = input.next();
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

    fn parse(tokens: IntoIter<Token>) -> Tree;

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
                '+' => left_num + right_num,
                '-' => left_num - right_num,
                '*' => left_num * right_num,
                '/' => left_num / right_num,
                 _  => { println!("{} is not Ope", operator); panic!(); },
            }
        },
    }
}

