#![feature(exact_size_is_empty)]
mod utils;
mod tree;
mod arith;
mod equa; use equa::*;

/* STEP.5
 * equation ::= expr '==' expr
 * expr ::= mul ('+' mul | '-' mul)*
 * mul ::= prim ('*' prim | '/' prim)
 * prim ::= num | '(' expr ')'
 */




fn main() { /* printing debug */

}
