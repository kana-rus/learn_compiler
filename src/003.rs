mod utils; use utils::*;

/* STEP.3
 * expr ::= prim (('+'|'-') prim)*
 * prim ::= num | '(' expr ')'
 */

impl Parse for Arith {
    fn parse(mut tokens: std::collections::vec_deque::IntoIter<Token>) -> Tree {
        let mut root = parse_prim(&mut tokens);
        while root.elem.is_init() && root.right.is_none() {
            root = *root.left.unwrap();
        }

        Tree { root }
    }
}
fn parse_prim(tokens: &mut std::collections::vec_deque::IntoIter<Token>) -> Node {
    let mut root = Node::init();

    while let Some(token) = tokens.next() {
        match token {
            Token::Num(n) => {
                if root.elem.is_init() {
                    root.elem = Elem::Num(n);
                } else if root.right.is_none() {
                    root.insert_right(n);
                } else {
                    exit_with_report(format!("unexpected token: {}", n));
                }
            },
            Token::Ope(o) => {
                if root.elem.is_init() && root.left.is_some() {
                    root.elem = Elem::Ope(o);
                } else if root.elem.is_num() ||
                (root.elem.is_ope() && root.left.is_some() && root.right.is_some()) {
                    root = Node {
                        elem: Elem::Ope(o),
                        left: link(root),
                        right: None,
                    }
                } else {
                    exit_with_report(format!("unexpected token: {}", o));
                }
            },
            Token::PrimOpen => {
                if root.left.is_none() {
                    root.left = link(parse_prim(tokens));
                } else if root.right.is_none() {
                    root.right = link(parse_prim(tokens));
                } else {
                    exit_with_report("unexpected token: \"(\"");
                }
            },
            Token::PrimClose => return root,
        }
    }
    root
}


#[test]
fn input1pl_2pl3_() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("1 + (2 + 3)")
                )
            )
        ),
        1 + (2 + 3)
    );
}
#[test]
fn input1pl_2pl_3pl4__() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("1 + (2 + (3 + 4))")
                )
            )
        ),
        1 + (2 + (3 + 4))
    );
}
#[test]
fn input_2_() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("(2)")
                )
            )
        ),
        2
    );
}
#[test]
fn input__2__() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("((2))")
                )
            )
        ),
        2
    );
}
#[test]
fn input_1pl2_() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("(1 + 2)")
                )
            )
        ),
        1 + 2
    );
}
#[test]
fn input1pl__2pl3_pl4pl5_() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("1 + ((2 + 3) + 4 + 5)")
                )
            )
        ),
        1 + ((2 + 3) + 4 + 5)
    );
}

fn main() { /* printing debug */
    println!("\n1 + (2 + 3)...\n{:?}",
        Arith::parse(
            Arith::tokenize(
                String::from("1 + (2 + 3)")
            )
        )
    );
    println!("\n1 + ( 2 + (3 + 4))...\n{:?}",
        Arith::parse(
            Arith::tokenize(
                String::from("1 + ( 2 + (3 + 4))")
            )
        )
    );
    println!("\n(2)...\n{:?}",
        Arith::parse(
            Arith::tokenize(
                String::from("(2)")
            )
        )
    );
    println!("\n((2))...\n{:?}",
        Arith::parse(
            Arith::tokenize(
                String::from("(( 2))")
            )
        )
    );
    println!("\n(1 + 2)...\n{:?}",
        Arith::parse(
            Arith::tokenize(
                String::from("(1 + 2)")
            )
        )
    );
    println!("\n1 + ((2 + 3) + 4 + 5)...\n{:?}",
        Arith::parse(
            Arith::tokenize(
                String::from("1 + ((2 + 3) + 4 + 5)")
            )
        )
    );
}
