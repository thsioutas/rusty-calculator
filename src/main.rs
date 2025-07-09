use crate::parser::Parser;
use clap::Parser as ClapParser;
use tracing::{info, Level};
use tracing_subscriber::fmt;

mod parser;
mod token;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The log verbosity level
    #[clap(short, long)]
    pub verbosity: Level,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // Setup logger
    let subscriber = fmt().with_max_level(args.verbosity).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        let input = buffer.trim();
        match calculate(input) {
            Ok(result) => println!("{} = {}", input, result),
            Err(err) => println!("An error occurred while calculating: {}: {}", input, err),
        }
    }
}

fn calculate(input: &str) -> anyhow::Result<i64> {
    info!("{input}");
    let mut parser = Parser::new(input)?;
    // Parse the input and return the expression
    let expr = parser.parse_expr()?;
    info!("{expr:?}");
    // Evaluate the expression
    let res = expr.eval()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::calculate;
    use std::i64;

    #[test]
    fn test_calculate_simple() {
        let input = "-1+5*(2+1)-3";
        let result = calculate(input).unwrap();
        assert_eq!(11, result);
    }

    #[test]
    fn test_calculate_double_parenthesis() {
        let input = "-2+5*((10+5)*3)+8-14/2";
        let result = calculate(input).unwrap();
        assert_eq!(224, result);
    }

    #[test]
    fn test_calculate_dib_by_zero() {
        let input = "-2+10/(5-5)";
        let err = calculate(input).unwrap_err();
        assert_eq!("Division by zero", format!("{}", err));
    }

    #[test]
    fn test_calculate_negative_result() {
        let input = "5*(3-5)+1";
        let result = calculate(input).unwrap();
        assert_eq!(-9, result);
    }

    #[test]
    fn test_calculate_spaces() {
        let input = "5   *(3-  5) +1";
        let result = calculate(input).unwrap();
        assert_eq!(-9, result);
    }

    #[test]
    fn test_calculate_wrong_input() {
        let input = "5**(3-  5) +1";
        let err = calculate(input).unwrap_err();
        assert_eq!("Unexpected token in factor: Asterisk", format!("{}", err));

        let input = "5*)(3-  5) +1";
        let err = calculate(input).unwrap_err();
        assert_eq!(
            "Unexpected token in factor: RightParenthesis",
            format!("{}", err)
        );
    }

    #[test]
    fn test_calculate_unmatched_parenthesis() {
        let input = "1+((2*3)+2";
        let _err = calculate(input).unwrap_err();
    }

    #[test]
    fn test_calculate_overflow() {
        let input = format!("{}+{}", i64::MAX, i64::MAX);
        let err = calculate(&input).unwrap_err();
        assert_eq!("Overflow on addition", format!("{}", err));

        let input = format!("{}+1", i64::MIN);
        let err = calculate(&input).unwrap_err();
        assert_eq!("number too large to fit in target type", format!("{}", err));
    }
}
