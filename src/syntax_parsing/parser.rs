

use pest::Parser;

#[derive(Parser)]
#[grammar = "ctiny.pest"]
pub struct CTinyParser;

pub fn parse(file_content: String) -> Result<(), pest::error::Error<Rule>> {
    // log::debug!("Content of file: {}", file_content);

    // let pairs = CTinyParser::parse(Rule::translation_unit, &file_content)?;



    //let test_string = "bool test_function() { }";
    let test_string = "int variable_name[10], variable_name2;";
    let pairs = CTinyParser::parse(Rule::multi_declaration, test_string)
        ?.next().unwrap()
        .into_inner();
    
    // let pairs = CTinyParser::parse(Rule::function_definition, test_string)
    //     ?.next().unwrap()
    //     .into_inner();

    // let pair = pairs.clone().next().unwrap();
    //assert_eq!(pair.as_rule(), Rule::function_definition);
    //assert_eq!(pair.as_str(), test_string);

    let tokens = pairs.tokens();

    for token in tokens {
        println!("{:?}", token);
    }


    // if the parsing succeeds, return Ok
    Ok(())
}