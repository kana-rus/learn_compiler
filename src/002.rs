mod utils; use utils::*;

/* STEP.2
 * expr ::= num ('+' num)*
 */

impl Process for Parser {
    fn parse(mut tokens: std::collections::vec_deque::IntoIter<Token>) -> Tree {
        let mut tree = Tree::new();

        let first = tokens.next().expect("input is required").expect_num();
        tree.root.elem = Elem::Num(first);

        while let Some(token) = tokens.next() {
            let op = token.expect_ope();
            tree.root = Node {
                elem: Elem::Ope(op),
                left: link(tree.root),
                right: None,
            };
            let right = tokens.next().expect("right Num not exists").expect_num();
            tree.root.insert_right(right);
        }

        tree
    }
}





#[test]
fn _15_plus_27_plus_41_plus_66() {
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

fn main() { /* printing, panicing debug */
    let tokens = Parser::tokenize(String::from("1 + 2 + 3"));
    println!("{:?}", tokens);
    let tree = Parser::parse(tokens);
    println!("{:?}", tree);
    let value = Parser::evaluate(tree);
    println!("{}", value);

    Parser::parse(Parser::tokenize(String::from(" + 102")));//shold panic
}
