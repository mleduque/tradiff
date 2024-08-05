
mod tra_structs;
#[cfg(test)]
mod test;

pub use tra_structs::*;

use lalrpop_util::lalrpop_mod;

lalrpop_util::lalrpop_mod!(tra);

pub use tra::*;
