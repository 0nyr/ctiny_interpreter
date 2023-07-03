

use pest::Parser;

#[derive(Parser)]
#[grammar = "ctiny.pest"]
pub struct CTinyParser;

#[test]
fn my_test() {
    // to see test output: $ cargo test -- --nocapture
    let parse_result = CTinyParser::parse(Rule::addition, "1773 + 1362").unwrap();
    
    // check that the first pair is an the correct addition
    let pair = parse_result.clone().next().unwrap();
    assert_eq!(pair.as_rule(), Rule::addition);
    assert_eq!(pair.as_str(), "1773 + 1362");
    
    // print the tokens
    let tokens = parse_result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

pub fn parse(file_content: String) -> Result<(), pest::error::Error<Rule>> {
    log::debug!("Content of file: {}", file_content);

    let pairs = CTinyParser::parse(Rule::translation_unit, &file_content)?;

    // if the parsing succeeds, return Ok
    Ok(())
}