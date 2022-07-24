mod utils; use utils::*;

/* STEP.2
 * expr ::= num ('+' num)*
 */

impl Process for Parser {
    fn parse(mut tokens: std::collections::vec_deque::IntoIter<Token>) -> Tree {
        let mut tree = Tree::new();

        let first = tokens.next().expect("input is required")
                                 .unwrap_num()
                                 .expect("firat token has to be Num");
        tree.root.element = Element::Num(first);

        while let Some(token) = tokens.next() {
            let op = token.unwrap_ope().expect(&format!("unexpected token: {:?}", token));
            tree.root = Node {
                element: Element::Ope(op),
                left: link(tree.root),
                right: None,
            };
            let right = tokens.next().expect("right Num not exists")
                            .unwrap_num().expect("not Num");
            tree.root.right = link(Node {
                element: Element::Num(right),
                left: None,
                right: None
            })
        }

        tree
    }

    fn evaluate(tree: Tree) -> Int {
        let (op, lh, rh) = (
            tree.root.element,
            tree.root.left,
            tree.root.right
        );
        calc(op, lh, rh)
    }
}

fn calc(op: Element, lh: Link<Node>, rh: Link<Node>) -> Int {
    match op {
        Element::Num(number) => number,
        Element::Ope(operator) => {
            let (lh, rh) = (
                lh.expect("left Node not exists"),
                rh.expect("right Node not exists")
            );
            let left_num = match lh.element {
                Element::Num(number) => number,
                Element::Ope(lop) => calc(
                    Element::Ope(lop), lh.left, lh.right
                ),
            };
            let right_num = match rh.element {
                Element::Num(number) => number,
                Element::Ope(rop) => calc(
                    Element::Ope(rop), rh.left, rh.right
                ),
            };

            match operator {
                '+' => left_num + right_num,
                '-' => left_num - right_num,
                '*' => left_num * right_num,
                '/' => left_num / right_num,
                 _  => panic!(),
            }
        },
    }
}




#[test]
fn plus_15_27_41_66() {
    assert_eq!(
        Parser::evaluate(
            Parser::parse(
                Parser::tokenize(String::from(" 15  +27+41 +   66"))
            )
        ),
        15 + 27 + 41 + 66
    );
}
#[test]
fn _44_minus_28_plus_5555_minus_63() {
    assert_eq!(
        Parser::evaluate(
            Parser::parse(
                Parser::tokenize(String::from("44  - 28+ 5555 -  63   "))
            )
        ),
        44 - 28 + 5555 - 63
    );
}

fn main() { /* printing debug */
    let tokens = Parser::tokenize(String::from("1 + 2 + 3"));
    println!("{:?}", tokens);
    let tree = Parser::parse(tokens);
    println!("{:?}", tree);
    let value = Parser::evaluate(tree);
    println!("{}", value);
}
