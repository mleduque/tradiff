
mod lexer;
mod parsers;
mod token;
mod tra_structs;
#[cfg(test)]
mod test;

pub use tra_structs::*;
pub use parsers::parse_trafile;
pub use token::{Token, LexError};

use lalrpop_util::lalrpop_mod;

lalrpop_util::lalrpop_mod!(tra);

pub use tra::*;
