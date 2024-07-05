use ast::BinaryAST;

mod ast;
mod token;

fn main() {
    let input = "(1 + 2) * (3 - 4)";
    let ast = BinaryAST::new(input);

    println!("{:#?}", ast);
}