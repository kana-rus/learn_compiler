use crate::utils::exit_with_report;
use super::{Tree, Node, Elem};
use std::fmt::Display;


impl Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Elem::Init => exit_with_report("found remaining Init elem"),
            Elem::Num(n) => format!("Num({})", n),
            Elem::Ope(o) => format!("Ope({})", o),
        })
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_node(&self.root, 0))
    }
}

fn format_node(node: &Node, indent: u16) -> String {
    let mut output = String::new();
    
    // for _ in 0..indent { output += " "; }
    output += "Node {\n";
    for _ in 0..indent { output += " "; }
    output += &format!("    elem : {}, \n", node.elem);

    for _ in 0..indent { output += " "; }
    output += &format!("    left : {}, \n",
        if node.left.is_none() {
            String::from("None")
        } else {
//            String::from("\n") +
            /*&*/format_node(node.left.as_ref().unwrap().as_ref(), indent+4)
        }
    );

    for _ in 0..indent { output += " "; }
    output += &format!("    right: {},\n",
        if node.right.is_none() {
            String::from("None")
        } else {
            //String::from("\n") +
            /*&*/format_node(node.right.as_ref().unwrap().as_ref(), indent+4)
        }
    );

    for _ in 0..indent { output += " "; }
    output += "}";

    output
}
