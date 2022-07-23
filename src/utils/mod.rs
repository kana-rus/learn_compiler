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

pub fn tokenize(input: String) -> IntoIter<Token> {
    let mut input = input.chars();
    let mut digits = VecDeque::<Int>::new();
    let mut ret = VecDeque::<Token>::new();

    while let Some(char) = input.next() {
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
                    ' '                   => continue,
                     _                    => { println!("unexpected: {}", char); panic!(); }
                }
            },
        }
    }
    ret.into_iter()//.iter()
}
    // pub fn parse(tokens: Iter<Token>) -> Tree;
    // pub fn evaluate(tree: Tree) -> Int;

#[derive(Debug)]
pub struct Tree {
    pub root: Node,
} impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Node {
                element: Element::Num(0),
                left: None,
                right: None,
            }
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub element: Element,
    pub left: Link<Node>,
    pub right: Link<Node>,
}

#[derive(Debug)]
pub enum Element {
    Num(Int),
    Ope(char),
} impl Element {
    pub fn unwrap_num(self) -> Int {
        match self {
            Element::Num(number) => number,
            Element::Ope(_) => panic!(),
        }
    }
    pub fn unwrap_ope(self) -> char {
        match self {
            Element::Num(_) => panic!(),
            Element::Ope(operator) => operator,
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
    pub fn unwrap_num(&self) -> Option<Int> {
        match self {
            Token::Num(number) => Some(*number),
            _ => None,
        }
    }
    pub fn unwrap_ope(&self) -> Option<char> {
        match self {
            Token::Ope(operator) => Some(*operator),
            _ => None,
        }
    }
}
