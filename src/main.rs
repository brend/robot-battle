mod tokenizer;

fn main() {
    let script = r#"
rotate treads 90
move forward 10
scan
fire
if scan > 0 {
    fire
}
"#;

    let tokens = tokenizer::tokenize_script(script);

    println!("Tokens: {:#?}", tokens);
}
