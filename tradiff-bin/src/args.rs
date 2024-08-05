
use clap_derive::Parser;


#[derive(Parser, Debug)]
#[command(name = "tradiff")]
#[command(author, version)]
#[command(about = "Shows differences in entries between two weidu TRA files", long_about = None)]
pub struct Cli {

    /// Charset to be used when reading both files<br>
    /// For the accepted values see https://encoding.spec.whatwg.org/#concept-encoding-get
    #[arg(long, short)]
    pub charset: Option<String>,
    /// Charset to be used when reading the first file<br>
    /// For the accepted values see https://encoding.spec.whatwg.org/#concept-encoding-get
    #[arg(long, requires = "charset2", conflicts_with="charset")]
    pub charset1: Option<String>,
    /// Charset to be used when reading the second file<br>
    /// For the accepted values see https://encoding.spec.whatwg.org/#concept-encoding-get
    #[arg(long, requires = "charset1", conflicts_with="charset")]
    pub charset2: Option<String>,

    /// The first file to be compared
    pub file1: String,
    /// The second file to be compared
    pub file2: String,
}
