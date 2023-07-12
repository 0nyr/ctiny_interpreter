use pest::error::Error;

use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "ctiny.pest"]
pub struct CTinyParser;

pub fn parse(rule: Rule, file_content: &str) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    let pairs = CTinyParser::parse(rule, file_content);
    pairs
}