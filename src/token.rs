use logos::{Logos, SpannedIter};
use std::num::ParseIntError;
#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", skip r"#.*\n?", error = LexicalError)]
pub enum Token {
    #[token("SELECT")]
    SELECT,
    #[token("FROM")]
    FROM,
    #[token("WHERE")]
    WHERE,
    #[token("*")]
    STAR,

    #[regex("[1-9][0-9]*", |lex| lex.slice().parse())]
    NUMBER(i64),

    #[regex(r"[a-zA-Z]+[a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    IDENTIFIER(String),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(_: ParseIntError) -> Self {
        LexicalError::InvalidToken
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token>,
}
impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}
