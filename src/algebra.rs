#[derive(Debug)]
pub enum Algebra {
    Query(Box<Algebra>, Box<Algebra>, Option<Box<Algebra>>),
    SELECT(Args),
    FROM(Identifier),
    WHERE(Args),
}

pub type Args = Vec<Arg>;

#[derive(Debug)]
pub enum Literal {
    Int(i64),
}

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum Arg {
    Identifier(Identifier),
    Literal(Literal),
    Star,
}
