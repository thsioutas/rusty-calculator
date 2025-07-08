use crate::token::*;
use anyhow::{anyhow, bail, Result};
use tracing::{debug, trace};

/// AST formation which supports overflow-safe operations using `checked_*` methods
#[derive(Clone, Debug)]
pub enum Expr {
    Int(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Recursively evaluates the expression returning an `i64` or an error.
    ///
    /// # Errors
    /// Returns an `anyhow::Error` in the following cases:
    /// - Overflow on any operation
    /// - Division by zero
    pub fn eval(&self) -> Result<i64> {
        debug!("Eval: {:?}", self);
        match self {
            Expr::Int(n) => Ok(*n),
            Expr::Neg(e) => e
                .eval()?
                .checked_neg()
                .ok_or(anyhow!("Overflow on negation")),
            Expr::Add(a, b) => a
                .eval()?
                .checked_add(b.eval()?)
                .ok_or(anyhow!("Overflow on addition")),
            Expr::Sub(a, b) => a
                .eval()?
                .checked_sub(b.eval()?)
                .ok_or(anyhow!("Overflow on substraction")),
            Expr::Mul(a, b) => a
                .eval()?
                .checked_mul(b.eval()?)
                .ok_or(anyhow!("Overflow on multiplication")),
            Expr::Div(a, b) => {
                let b = b.eval()?;
                if b == 0 {
                    bail!("Division by zero")
                } else {
                    a.eval()?
                        .checked_div(b)
                        .ok_or(anyhow!("Overflow on division"))
                }
            }
        }
    }
}

/// Recursive-descent parser for arithmetic expressions.
/// It holds:
/// * A `TokenTranslator`, which converts raw characters into tokens
/// * The current token
pub struct Parser<'a> {
    translator: TokenTranslator<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut translator = TokenTranslator::new(input);
        let current_token = translator.next_token()?;
        debug!("First token = {:?}", current_token);
        Ok(Parser {
            translator,
            current_token,
        })
    }

    /// Parse an additive expression
    /// An expression is a combination of numbers, and operations (addition, subtraction, multiplication, division)
    /// expression ::= term (("+" | "-") term)*
    /// For example: "2*3+5-6"
    pub fn parse_expr(&mut self) -> Result<Expr> {
        debug!("Start parsing expression from {:?}", self.current_token);
        let mut node = self.parse_term()?;
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let operation = self.current_token.clone();
            debug!("Operate: {:?}", operation);
            self.advance()?;
            let rhs = self.parse_term()?;
            node = match operation {
                Token::Plus => Expr::Add(Box::new(node.clone()), Box::new(rhs)),
                Token::Minus => Expr::Sub(Box::new(node.clone()), Box::new(rhs)),
                _ => unreachable!("Unreachable"),
            };
            debug!("New expression {:?}", node);
        }
        Ok(node)
    }

    /// Parse a term
    /// A term is multiplicative expression:
    /// term ::= factor (("*" | "/") factor)*
    /// For example: "2 * 3 / 4"
    fn parse_term(&mut self) -> Result<Expr> {
        debug!("Start parsing term from {:?}", self.current_token);
        let mut node = self.parse_factor()?;
        while matches!(self.current_token, Token::Asterisk | Token::Slash) {
            let operation = self.current_token.clone();
            debug!("Operate: {:?}", operation);
            self.advance()?;
            let rhs = self.parse_factor()?;
            node = match operation {
                Token::Asterisk => Expr::Mul(Box::new(node.clone()), Box::new(rhs)),
                Token::Slash => Expr::Div(Box::new(node.clone()), Box::new(rhs)),
                _ => unreachable!("Unreachable"),
            };
            debug!("New term {:?}", node);
        }
        Ok(node)
    }

    /// Parse a primary expression:
    /// factor ::= INT | "-" factor | "(" expression ")"
    ///
    /// Handles:
    /// - Integer literals (i.e. 42)
    /// - Negative numbers (i.e. -7)
    /// - Parenthesized sub-expressions (i.e. (1+2))
    fn parse_factor(&mut self) -> Result<Expr> {
        debug!("Parse {:?} as factor", self.current_token);
        match self.current_token {
            Token::Minus => {
                self.advance()?;
                let factor = self.parse_factor()?;
                let factor = Expr::Neg(Box::new(factor));
                debug!("New factor {:?}", factor);
                Ok(factor)
            }
            Token::Int(n) => {
                self.advance()?;
                let factor = Expr::Int(n);
                debug!("New factor {:?}", factor);
                Ok(factor)
            }
            Token::LeftParenthesis => {
                self.advance()?;
                let expr = self.parse_expr()?;
                self.advance()?;
                debug!("New expression (via factor) {:?}", expr);
                Ok(expr)
            }
            _ => bail!("Unexpected token in factor: {:?}", self.current_token),
        }
    }

    fn advance(&mut self) -> Result<()> {
        trace!("Advance");
        self.current_token = self.translator.next_token()?;
        Ok(())
    }
}
