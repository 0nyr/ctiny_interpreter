use pest::error::Error;

use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "ctiny.pest"]
pub struct CTinyParser;


pub fn parse(file_content: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    let pairs = CTinyParser::parse(Rule::translation_unit, file_content)?;
    Ok(pairs)
}


    // let tokens = pairs.tokens();

    // for token in tokens {
    //     println!("{:?}", token);
    // }


    // if the parsing succeeds, return Ok