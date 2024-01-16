use pest::Parser;

#[derive(Parser)]
#[grammar = "core/zest.pest"]
struct ZestParser;

use pest::error::Error;

#[path = "./ast.rs"]
pub mod ast;

pub fn parse_zest_file(file: &str) -> Result<ast::Expression, Error<Rule>> {
    let zest = ZestParser::parse(Rule::zest, file)?.next().unwrap();

    use pest::iterators::Pair;
    fn recursive_parser(p: Pair<Rule>) -> ast::Expression {
        match p.as_rule() {
            Rule::number => ast::Expression::Number(p.as_str().parse().unwrap()),
            Rule::string => ast::Expression::String(p.as_str().parse().unwrap()), // with bug
            Rule::expr => {
                let mut iter = p.into_inner();

                if let Some(val) = iter.next() {
                    let mut name = String::new();
                    let mut expr: Vec<ast::Expression> = Vec::new();

                    match val.as_rule() {
                        Rule::name => name = val.as_str().to_string(),
                        _ => expr.push(recursive_parser(val))
                    }
                    loop {
                        match iter.next() {
                            Some(x) => expr.push(recursive_parser(x)),
                            None => break
                        }
                    }
                    ast::Expression::Expression { operator: name, operands: expr }

                } else {
                    // handle empty expression
                    ast::Expression::Expression { operator: String::from(""), operands: vec![] }
                }
            },
            Rule::zest => {
                recursive_parser(p.into_inner().next().unwrap())
            },
            _ => unreachable!()
        }
    }

    // println!("{:#?}\n", zest);
    Ok(recursive_parser(zest))
}
