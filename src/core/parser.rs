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
            Rule::name => ast::Expression::Name(p.as_str().to_string()),
            Rule::expr => {
                let expr: Vec<ast::Expression> = p.into_inner().map(recursive_parser).collect();
                if expr.len() == 0 {
                    ast::Expression::Unit
                } else { 
                    ast::Expression::Expression(expr)
                }
            }
            Rule::zest => {
                recursive_parser(p.into_inner().next().unwrap())
            },
            _ => unreachable!()
        }
    }

    // println!("{:#?}\n", zest);
    Ok(recursive_parser(zest))
}
