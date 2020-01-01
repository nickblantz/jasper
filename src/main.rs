mod interpreter;

use std::fs;
use interpreter::parser::input_stream::InputStream;
use interpreter::parser::token_stream::TokenStream;

fn main() {
    let file_name: &str = "data/my_program.lang";
    let mut lang_file = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut is: InputStream = InputStream::from_string(&mut lang_file);
    let mut ts: TokenStream = TokenStream::create(&mut is);
    while let Some(tok) = ts.read_next() {
        println!("{:?}", tok);
    }


    //let kw: Keyword = Keyword::from_string(String::from("end"));
    //let kw_s: str = kw.to_str();
}