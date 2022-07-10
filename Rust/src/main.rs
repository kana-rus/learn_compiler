use std::env;

fn main() {
    let input = {
        let args = env::args().collect::<Vec<String>>();
        if args.len() != 2 {
            println!("only 1 arg is need!");
            panic!();
        }
        args[1].parse::<usize>().expect("failed to convert arg into usize")
    };

    println!("
.intel_syntax noprefix
.global main
main:
  mov rax, {}
  ret
", input);
}
