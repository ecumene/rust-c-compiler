use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Int,
    Return,
    Identifier(String),
    IntLiteral(i32),
}

impl Token {
    fn from_string(input: String) -> Option<Token> {
        match input.as_ref() {
            "" | " " | "\n" => None,
            "(" => Some(Token::OpenParenthesis),
            ")" => Some(Token::CloseParenthesis),
            "{" => Some(Token::OpenBrace),
            "}" => Some(Token::CloseBrace),
            ";" => Some(Token::Semicolon),
            "int" => Some(Token::Int),
            "return" => Some(Token::Return),
            _ => {
                if let Ok(number) = input.parse::<i32>() {
                    return Some(Token::IntLiteral(number));
                }
                Some(Token::Identifier(input))
            }
        }
    }
}

trait IsDelimiter {
    fn is_delimiter(&self) -> bool;
}

impl IsDelimiter for char {
    fn is_delimiter(&self) -> bool {
        match self {
            ' ' | '(' | ')' | '}' | '{' | ';' => true,
            _ => false,
        }
    }
}

trait Tokenize {
    fn tokenize(self) -> Vec<Token>;
}

impl Tokenize for &str {
    fn tokenize(self) -> Vec<Token> {
        let mut chars = self.chars().peekable();
        let mut out = vec![];
        loop {
            if !chars.peek().is_some() {
                return out
                    .iter()
                    .filter_map(|token: &String| Token::from_string(token.clone()))
                    .collect();
            }
            let token = chars
                .take_while_ref(|character| !character.is_delimiter())
                .collect::<String>();
            out.push(token);
            out.push(chars.next().unwrap().to_string());
        }
    }
}

fn main() {
    println!("Hello, world!");
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
        let output = vec![
            Token::Int,
            Token::Identifier("main".to_owned()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Return,
            Token::IntLiteral(2),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        assert_eq!(input.tokenize(), output);
    }
}
