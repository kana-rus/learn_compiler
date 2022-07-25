use std::{collections::vec_deque::{VecDeque, IntoIter}, fmt::Display};

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
pub fn exit_with_report<D: Display>(msg: D) -> ! {
    println!("{}", msg);
    panic!()
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


#[derive(Debug)]
pub struct Tree {
    pub root: Node,
} impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Node {
                elem: Elem::Num(0),
                left: None,
                right: None
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

    pub fn init() -> Node {
        Node {
            elem: Elem::Init,
            left: None,
            right: None,
        }
    }

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
    Init,
    Num(Int),
    Ope(char),
} impl Elem {
    pub fn is_init(&self) -> bool {
        match self {
            Elem::Init => true,
            _ => false,
        }
    }
    pub fn is_num(&self) -> bool {
        match self {
            Elem::Num(_) => true,
            _ => false,
        }
    }
    pub fn is_ope(&self) -> bool {
        match self {
            Elem::Ope(_) => true,
            _ => false,
        }
    }

    pub fn expect_num(self) -> Int {
        match self {
            Elem::Num(number) => number,
            Elem::Ope(char) => {println!("{} is not Num", char); panic!();},
            _ => exit_with_report("this is Init elem"),
        }
    }
    pub fn unwrap_ope(self) -> char {
        match self {
            Elem::Num(num) => {println!("{} is not Ope", num); panic!();},
            Elem::Ope(operator) => operator,
            Elem::Init => exit_with_report("thi is Init elem"),
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
    pub fn is_num(&self) -> bool {
        match self {
            Token::Num(_) => true,
            _ => false,
        }
    }
    pub fn is_ope(&self) -> bool {
        match self {
            Token::Ope(_) => true,
            _ => false,
        }
    }
    pub fn is_prim_open(&self) -> bool {
        match self {
            Token::PrimOpen => true,
            _ => false,
        }
    }
    pub fn is_prim_close(&self) -> bool {
        match self {
            Token::PrimClose => true,
            _ => false,
        }
    }

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
