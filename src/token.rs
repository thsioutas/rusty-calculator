use anyhow::{bail, Result};

/// Tokens representing the units of a methematical expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParenthesis,
    RightParenthesis,
    Eof,
}

/// Converts a string input into a sequence of `Token`s
pub struct TokenTranslator<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> TokenTranslator<'a> {
    /// Constructs a `TokenTranslator` from the input string.
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    /// Returns the next token from the input string.
    ///
    /// # Errors
    /// Returns an `anyhow::Error` in the following cases:
    /// - Int64 parsing failure
    /// - Unexpected character/symbol in the input string
    pub fn next_token(&mut self) -> Result<Token> {
        match self.chars.next() {
            Some(' ') => self.next_token(),
            Some('+') => Ok(Token::Plus),
            Some('-') => Ok(Token::Minus),
            Some('*') => Ok(Token::Asterisk),
            Some('/') => Ok(Token::Slash),
            Some('(') => Ok(Token::LeftParenthesis),
            Some(')') => Ok(Token::RightParenthesis),
            Some(ch) if ch.is_ascii_digit() => {
                let mut digits = ch.to_string();
                while let Some(&d @ '0'..='9') = self.chars.peek() {
                    digits.push(d);
                    self.chars.next();
                }
                let num = digits.parse()?;
                Ok(Token::Int(num))
            }
            None => Ok(Token::Eof),
            ch => bail!("Unexpected character: '{:?}'", ch),
        }
    }
}
