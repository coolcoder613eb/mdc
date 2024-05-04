use mini_markdown::lex;
use mini_markdown::lexer::Token;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let markdown_file = args[1].clone();
    let markdown_input = std::fs::read_to_string(markdown_file).expect("Failed to read file.");
    let tokens = lex(markdown_input.as_str(), &[]);
    //println!("{:#?}", tokens);
    for token in tokens {
        match token {
            Token::Header(level, text, _) => {
                println!("{}{}", level, text)
            }
            Token::Newline => {
                println!("N")
            }
            Token::Plaintext(text) => {
                for line in text.lines() {
                    println!("T{}", line)
                }
            }
            Token::Bold(text) => {
                println!("B{}", text)
            }
            Token::Italic(text) => {
                println!("I{}", text)
            }
            Token::Link(_, title, _) => match title {
                Some(text) => {
                    println!("T{}", text)
                }
                None => {}
            },
            Token::Code(text) => {
                println!("M{}", text)
            }
            Token::CodeBlock(text, _) => {
                for line in text.lines() {
                    println!("C{}", line)
                }
            }
            _ => {}
        }
    }
}
