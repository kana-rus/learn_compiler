use std::collections::vec_deque::{VecDeque, IntoIter};

pub type Int = u32;
pub type Link<T> = Option<Box<T>>;

pub fn link(node: Node) -> Link<Node> {
    Some(Box::new(node))
}
pub fn assert(condition: bool, message: &str) {
    if !condition {
        println!("{}", message);
        panic!();
    }
}
pub fn build_from(degits: &VecDeque<Int>) -> Int {
    degits.iter().fold(0, |a, b| 10*a + b)
}


pub struct Parser;
pub trait Process {
fn tokenize(input: String) -> IntoIter<Token> {
    let mut input = input.chars();
    let mut digits = VecDeque::<Int>::new();
    let mut ret = VecDeque::<Token>::new();

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
                            '('                   => ret.push_back(Token::PrimOpen),
                            ')'                   => ret.push_back(Token::PrimClose),
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
    ret.into_iter()
}

    fn parse(tokens: IntoIter<Token>) -> Tree;

    fn evaluate(tree: Tree) -> Int {
        let (op, lh, rh) = (
            tree.root.elem,
            tree.root.left,
            tree.root.right
        );
        calc(op, lh, rh)
    }
}
fn calc(op: Elem, lh: Link<Node>, rh: Link<Node>) -> Int {
    match op {
        Elem::Num(number) => number,
        Elem::Ope(operator) => {
            let (lh, rh) = (
                lh.expect("left Node not exists"),
                rh.expect("right Node not exists")
            );
            let left_num = match lh.elem {
                Elem::Num(number) => number,
                Elem::Ope(lop) => calc(
                    Elem::Ope(lop), lh.left, lh.right
                ),
            };
            let right_num = match rh.elem {
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


#[derive(Debug)]
pub struct Tree {
    pub root: Node,
} impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Node {
                elem: Elem::Num(0),
                left: None,
                right: None,
            }
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub elem: Elem,
    pub left: Link<Node>,
    pub right: Link<Node>,
} impl Node {

    pub fn insert_left(&mut self, num: Int) {
        self.left = link(Node {
            elem: Elem::Num(num),
            left: None,
            right: None,
        })
    }
    pub fn insert_right(&mut self, num: Int) {
        self.right = link(Node {
            elem: Elem::Num(num),
            left: None,
            right: None,
        })
    }
}

#[derive(Debug)]
pub enum Elem {
    Num(Int),
    Ope(char),
} impl Elem {
    pub fn expect_num(self) -> Int {
        match self {
            Elem::Num(number) => number,
            Elem::Ope(char) => {println!("{} is not Num", char); panic!();},
        }
    }
    pub fn unwrap_ope(self) -> char {
        match self {
            Elem::Num(num) => {println!("{} is not Ope", num); panic!();},
            Elem::Ope(operator) => operator,
        }
    }
}
#[derive(Debug)]
pub enum Token {
    Num(Int),
    Ope(char),
    PrimOpen,
    PrimClose,
} impl Token {
    pub fn expect_num(&self) -> Int {
        match self {
            Token::Num(number) => *number,
            Token::Ope(ope) => { println!("\"{}\" is not Num", ope); panic!(); },
            Token::PrimOpen => { println!("\"(\" is not Num"); panic!(); },
            Token::PrimClose => { println!("\")\" is not Num"); panic!(); },
        }
    }
    pub fn expect_ope(&self) -> char {
        match self {
            Token::Ope(operator) => *operator,
            Token::Num(num) => { println!("{} is not Ope", num); panic!(); },
            Token::PrimOpen => { println!("\"(\" is not Ope"); panic!(); },
            Token::PrimClose => { println!("\")\" is not Ope"); panic!(); },
        }
    }
}
