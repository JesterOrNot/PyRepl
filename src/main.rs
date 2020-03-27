use std::process::exit;
use synterm::{gen_lexer, gen_parse, Color, CommandLineTool};

struct MyTool;

impl CommandLineTool for MyTool {
    fn evaluator_function(line: &String) -> String {
        match line.as_str() {
            "exit" => {
                exit(0);
            }
            _ => {
                subprocess::Exec::shell(format!("python3 -c '{}'", line))
                    .stdout(subprocess::Redirection::Merge)
                    .join()
                    .unwrap();
                "".to_string()
            }
        }
    }
    fn syntax_highlight(string: &str) {
        gen_lexer!(TheLexer, (KeyWord, "import|print|lambda|if|for|async|await|assert|while|with|raise|return|try|pass|def|del|from|global|nonlocal|not"), (Integer, "[0-9]+"), (Operator, r"==|\*\*|\+|-|/|%"));
        gen_parse!(TheLexer, parser, (KeyWord, Color::Red), (Integer, Color::Green), (Operator, Color::Yellow));
        parser(TheLexer::lexer(string));
    }
}

fn main() {
    MyTool.start();
}
