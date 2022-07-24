mod utils; use utils::*;


/* STEP.1
 * expr ::= num '+' num
 * with NO SPACE
 * */

impl Process for Parser {
    fn parse(mut tokens: std::collections::vec_deque::IntoIter<Token>) -> Tree {
        let mut tree = Tree::new();
        let mut current = tokens.next();
        assert(current.is_some(), "input is required");

        let first_token_val = current.unwrap().unwrap_num();
        assert(first_token_val.is_some(), "first token has to be number");
        tree.root.left = link(Node {
            element: Element::Num(first_token_val.unwrap()),
            left: None,
            right: None,
        });

        current = tokens.next();
        assert(current.is_some(), "operator needed");
        if let Some(ope) = current.unwrap().unwrap_ope() {
            tree.root.element = Element::Ope(ope);
        } else {
            println!("unexpected operator"); panic!();
        }

        current = tokens.next();
        assert(current.is_some(), "right num needed");
        if let Some(num) = current.unwrap().unwrap_num() {
            tree.root.right = link(Node {
                element: Element::Num(num),
                left: None,
                right: None
            })
        }

        assert(tokens.next().is_none(), "right number has to be the end of expr");
        tree
    }
    fn evaluate(tree: Tree) -> Int {
        let (left, right) = (
            tree.root.left.unwrap().element.unwrap_num(),
            tree.root.right.unwrap().element.unwrap_num()
        );
        match tree.root.element.unwrap_ope() {
            '+' => left + right,
            '-' => left - right,
            '*' => left * right,
            '/' => left / right,
             _  => panic!(),
        }
    }
}




#[test]
fn plus_314_2345() {
    let tokens = Parser::tokenize(String::from(" 313 +  2345  "));
    let parsed_tree = Parser::parse(tokens);
    assert_eq!(
        Parser::evaluate(parsed_tree),
        2658
    );
}

fn main() { /* for printing debug */
    let tokens = Parser::tokenize(String::from(" 313 +  2345  "));
    println!("{:?}", tokens);
    let parsed_tree = Parser::parse(tokens);
    println!("{:?}", parsed_tree);
    println!("{}", Parser::evaluate(parsed_tree));
}
