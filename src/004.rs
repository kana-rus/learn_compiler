mod utils;
mod tree;
mod arith; use arith::*;

/* STEP.4
 * expr ::= mul (('+'|'-') mul)*
 * mul ::= prim (('*'|'/') prim)*
 * prim ::= num | '(' expr ')'
 */

impl Parse for Arith { /* default */ }


#[test]
#[allow(non_snake_case)]
fn input_1pl2_ml_3() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("(1+2) * 3")
                )
            )
        ),
        (1 + 2) * 3
    );
}
#[test]
#[allow(non_snake_case)]
fn input1pl__2pl3_ml4_ml5() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("1 + ((2+3)*4) * 5")
                )
            )
        ),
        1 + ((2 + 3) * 4) * 5
    );
}
#[test]
#[allow(non_snake_case)]
fn input_1pl2pl3_ml_4pl5pl6() {
    assert_eq!(
        Arith::evaluate(
            Arith::parse(
                Arith::tokenize(
                    String::from("(1+2+3) * (4+5+6)")
                )
            )
        ),
        (1+2+3) * (4+5+6)
    );
}

fn main() { /* printing debug */
    println!("\n(1+2)*3:\n{}",
        Arith::parse(
            Arith::tokenize(
                String::from("(1+2) * 3")
            )
        )
    );
    println!("\n(1+2+3)*(4+5+6):\n{}",
        Arith::parse(
            Arith::tokenize(
                String::from("(1+2+3) * (4+5+6)")
            )
        )
    );
}
