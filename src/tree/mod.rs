use crate::{exit_with_report, Int};
mod tree_utils;

pub type Link<T> = Option<Box<T>>;
pub fn link(node: Node) -> Link<Node> {
    Some(Box::new(node))
}


#[derive(Debug)]
pub struct Tree {
    pub root: Node,
} impl Tree {
    #[allow(unused)]
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
    #[allow(unused)]
    pub fn init() -> Node {
        Node {
            elem: Elem::Init,
            left: None,
            right: None,
        }
    }

    #[allow(unused)]
    pub fn insert_left(&mut self, num: Int) {
        self.left = link(Node {
            elem: Elem::Num(num),
            left: None,
            right: None,
        })
    }
    #[allow(unused)]
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
    #[allow(unused)]
    pub fn is_init(&self) -> bool {
        match self {
            Elem::Init => true,
            _ => false,
        }
    }
    #[allow(unused)]
    pub fn is_num(&self) -> bool {
        match self {
            Elem::Num(_) => true,
            _ => false,
        }
    }
    #[allow(unused)]
    pub fn is_ope(&self) -> bool {
        match self {
            Elem::Ope(_) => true,
            _ => false,
        }
    }

    #[allow(unused)]
    pub fn expect_num(self) -> Int {
        match self {
            Elem::Num(number) => number,
            Elem::Ope(char) => {println!("{} is not Num", char); panic!();},
            _ => exit_with_report("this is Init elem"),
        }
    }
    #[allow(unused)]
    pub fn unwrap_ope(self) -> char {
        match self {
            Elem::Num(num) => {println!("{} is not Ope", num); panic!();},
            Elem::Ope(operator) => operator,
            Elem::Init => exit_with_report("thi is Init elem"),
        }
    }
}
// #[derive(Debug)]
pub enum Token {
    // Init,
    Num(Int),
    Ope(char),
    PrimOpen,
    PrimClose,
} impl Token {
    #[allow(unused)]
    pub fn is_num(&self) -> bool {
        match self {
            Token::Num(_) => true,
            _ => false,
        }
    }
    #[allow(unused)]
    pub fn is_ope(&self) -> bool {
        match self {
            Token::Ope(_) => true,
            _ => false,
        }
    }
    #[allow(unused)]
    pub fn is_prim_open(&self) -> bool {
        match self {
            Token::PrimOpen => true,
            _ => false,
        }
    }
    #[allow(unused)]
    pub fn is_prim_close(&self) -> bool {
        match self {
            Token::PrimClose => true,
            _ => false,
        }
    }

    #[allow(unused)]
    pub fn expect_num(&self) -> Int {
        match self {
            Token::Num(number) => *number,
            Token::Ope(ope) => { println!("\"{}\" is not Num", ope); panic!(); },
            Token::PrimOpen => { println!("\"(\" is not Num"); panic!(); },
            Token::PrimClose => { println!("\")\" is not Num"); panic!(); },
            // Token::Init => exit_with_report("Init is not Num"),
        }
    }
    #[allow(unused)]
    pub fn expect_ope(&self) -> char {
        match self {
            Token::Ope(operator) => *operator,
            Token::Num(num) => { println!("{} is not Ope", num); panic!(); },
            Token::PrimOpen => { println!("\"(\" is not Ope"); panic!(); },
            Token::PrimClose => { println!("\")\" is not Ope"); panic!(); },
            // Token::Init => exit_with_report("Init is not Num"),
        }
    }
}
