use lalrpop_util::{ErrorRecovery, ParseError};

use crate::lexer::Lexer;
use crate::token::{LexError, Token};
use crate::{TraFileParser, TraFragment};


pub fn parse_trafile<'a>(
    errors: &mut Vec<ErrorRecovery<usize, Token, LexError>>,
    content: &'a str,
) -> Result<Vec<TraFragment>, ParseError<usize, Token, LexError>> {
    let mut gather_errors = Vec::new();
    let lexer = Lexer::new(content);
    let result = TraFileParser::new().parse(&mut gather_errors, lexer);
    errors.extend(gather_errors);
    result
}
