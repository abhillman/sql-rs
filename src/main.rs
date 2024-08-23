mod algebra;
mod token;

use crate::grammar::StatementParser;
use lalrpop_util::{lalrpop_mod, ParseError};

lalrpop_mod!(grammar);

use crate::algebra::Algebra;
use crate::token::{Lexer, LexicalError};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn eval(
    line: &String,
) -> std::result::Result<Algebra, ParseError<usize, token::Token, LexicalError>> {
    let lexer = Lexer::new(line);
    let parser = StatementParser::new();
    parser.parse(lexer)
}

fn main() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match eval(&line) {
                    Ok(algebra) => {
                        println!("{algebra:#?}")
                    }
                    Err(e) => {
                        eprintln!("{e:#?}")
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::grammar::StatementParser;
    use crate::token::Lexer;

    #[test]
    fn t0() {
        let lexer = Lexer::new("SELECT *");
        let parser = StatementParser::new();
        let alg = parser.parse(lexer);

        dbg!(&alg);
        assert!(alg.is_ok());
    }
}
