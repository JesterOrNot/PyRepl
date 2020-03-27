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
        gen_lexer!(TheLexer, (Foo, "foo"), (Bar, "bar"));
        gen_parse!(TheLexer, parser, (Foo, Color::Red), (Bar, Color::Green));
        parser(TheLexer::lexer(string));
    }
}

fn main() {
    MyTool.start();
}
