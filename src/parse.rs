use crate::lex::*;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Const(i32),
    UnOp(Operator, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Return(pub Expression);

#[derive(Debug, PartialEq)]
pub struct Fun(pub String, pub Return);

#[derive(Debug, PartialEq)]
pub struct Prog(pub Fun);

#[derive(Debug)]
pub struct ParseError<'a>(&'a Token);

impl<'a> Expression {
    fn new<I>(tokens: &mut I) -> Result<Expression, ParseError<'a>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let int_token = tokens.next().expect("No integer keyword.");
        match int_token {
            Token::IntLiteral(number) => Ok(Expression::Const(*number)),
            Token::Op(operator) => Ok(Expression::UnOp(
                *operator,
                Box::new(Expression::new(tokens)?),
            )),
            _ => Err(ParseError(int_token)),
        }
    }
}

impl<'a> Return {
    fn new<I>(tokens: &mut I) -> Result<Return, ParseError<'a>>
    where
        I: Iterator<Item = &'a Token>,
    {
        tokens
            .next()
            .expect_to_be(Token::Return, "No return keyword.")?;
        let constant_def = Ok(Return(Expression::new(tokens)?));
        tokens
            .next()
            .expect_to_be(Token::Semicolon, "No semicolon declared.")?;
        constant_def
    }
}

impl<'a> Fun {
    fn new<I>(tokens: &mut I) -> Result<Fun, ParseError<'a>>
    where
        I: Iterator<Item = &'a Token>,
    {
        tokens
            .next()
            .expect_to_be(Token::Int, "No function return type declared.")?;
        let identifier = tokens.next().expect("No function declared.");
        tokens
            .next()
            .expect_to_be(Token::OpenParenthesis, "No function declared.")?;
        tokens
            .next()
            .expect_to_be(Token::CloseParenthesis, "No close parenthesis declared.")?;
        tokens
            .next()
            .expect_to_be(Token::OpenBrace, "No open brace declared.")?;
        let fun_def = match identifier {
            Token::Identifier(id) => Ok(Fun(id.to_owned(), Return::new(tokens)?)),
            _ => Err(ParseError(identifier)),
        };
        tokens
            .next()
            .expect_to_be(Token::CloseBrace, "No closing brace declared.")?;
        fun_def
    }
}

trait ExpectToBe<'a, T> {
    fn expect_to_be(self, other: Token, error_message: &str) -> Result<T, ParseError<'a>>;
}

impl<'a> ExpectToBe<'a, Token> for Option<&'a Token> {
    fn expect_to_be(self, other: Token, error_message: &str) -> Result<Token, ParseError<'a>> {
        let inside = self.expect(error_message);
        if inside == &other {
            Ok(other)
        } else {
            Err(ParseError(inside))
        }
    }
}

impl FromStr for Prog {
    type Err = ParseError<'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Prog(
            Fun::new(&mut s.tokenize().iter()).expect("Invalid program."),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_lexes_sample_input() {
        let input = "\
int main() {
    return 2;
}";
        let output = Prog(Fun("main".to_string(), Return(Expression::Const(2))));
        assert_eq!(input.parse::<Prog>().unwrap(), output);
    }
}
