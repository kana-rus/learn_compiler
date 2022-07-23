type Int = u32;
fn assert(condition: bool, message: &str) {
    if !condition {
        println!("{}", message);
        panic!();
    }
}

/* STEP.1
 * expr ::= num '+' num
 * with NO SPACE
 * */
#[derive(Debug)]
struct Tree {
    root: Node,
}
#[derive(Debug)]
struct Node {
    value: Value,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
#[derive(Debug)]
enum Value {
    Num(Int),
    Ope(char),
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Node {
                value: Value::Ope('+'),
                left: None,
                right: None,
        } }
    }
}
impl Value {
    pub fn unwrap_num(self) -> Int {
        match self {
            Value::Num(number) => number,
            Value::Ope(_) => panic!(),
        }
    }
    pub fn is(self, target: char) -> bool {
        match self {
            Value::Ope(operator) => operator == target,
            Value::Num(_) => panic!(),
        }
    }
}

fn parse(input: String) -> Tree {
    assert(!input.is_empty(), "input is requireed");

    let mut tree = Tree::new();
    let mut input = input.chars();
    let (mut left_degits, mut right_degits) = (Vec::<Int>::new(), Vec::<Int>::new());
    /*
    let mut current_point = input.next();

    let mut current_value = current_point.expect("input is requireed");
    while current_value.is_digit(10) {
        num_degits.push(current_value.to_digit(10).unwrap());

    }
    */
    let mut next_point = input.next();
    while let Some(next_value) = next_point {
        if let Some(d) = next_value.to_digit(10) {
            left_degits.push(d);
        } else {
            break;
        }
        next_point = input.next();
    }
    assert(!left_degits.is_empty(), "left number is requireed");

    let op = next_point.expect("operator is requireed");
    assert(op == '+', &format!("unexpected operator: {}", op));

    next_point = input.next();
    while let Some(next_value) = next_point {
        if let Some(d) = next_value.to_digit(10) {
            right_degits.push(d);
        } else {
            break;
        }
        next_point = input.next();
    }
    assert(!right_degits.is_empty(), "right number is requireed");

    assert(next_point.is_none(), "right number has to be end of expr");

    tree.root.left = Some(Box::new(
        Node {
            value: Value::Num(left_degits.iter().fold(0, |a,b| 10*a + b)),
            left: None,
            right: None,
        }
    ));
    tree.root.right = Some(Box::new(
        Node {
            value: Value::Num(right_degits.iter().fold(0, |a,b| 10*a + b)),
            left: None,
            right: None
        }
    ));

    tree
}
fn eval(tree: Tree) -> Int {
    let root = tree.root;

    let operator = root.value;
    let left_val = root.left.unwrap().value.unwrap_num();
    let right_val = root.right.unwrap().value.unwrap_num();

    if operator.is('+') {
        left_val + right_val
    } else {
        println!("unexpected operator");
        panic!();
    }
}




#[test]
fn plus_1_2() {
    let parsed_tree = parse(String::from("1+2"));
    assert_eq!(
        eval(parsed_tree),
        3
    );
}
#[test]
fn plus_31_41() {
    let parsed_tree = parse(String::from("31+41"));
    assert_eq!(
        eval(parsed_tree),
        72
    );
}
#[test]
fn plus_89_72() {
    let parsed_tree = parse(String::from("89+72"));
    assert_eq!(
        eval(parsed_tree),
        161
    );
}
#[test]
fn plus_314_2345() {
    let parsed_tree = parse(String::from("314+2345"));
    assert_eq!(
        eval(parsed_tree),
        2659
    );
}

fn main() { /* for printing debug */
    let test3_tree = parse(String::from("89+72"));
    println!("{:?}", test3_tree);
}
