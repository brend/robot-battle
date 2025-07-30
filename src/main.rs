mod ast;
mod parser;
mod tokenizer;

fn main() {
    let script = r#"
loop {
    scan
    move forward 1
    fire
}
"#;

    let tokens = tokenizer::tokenize_script(script);

    let ast = parser::parse_tokens(&tokens);

    println!("AST: {:#?}", ast);
}
