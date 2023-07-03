

use pest::Parser;

#[derive(Parser)]
#[grammar = "ctiny.pest"]
pub struct CTinyParser;

pub fn parse(file_content: String) -> Result<(), pest::error::Error<Rule>> {
    //log::debug!("Content of file: {}", file_content);

    let pairs = CTinyParser::parse(Rule::translation_unit, &file_content)?;

    // let tokens = pairs.tokens();

    // for token in tokens {
    //     println!("{:?}", token);
    // }


    // if the parsing succeeds, return Ok
    Ok(())
}