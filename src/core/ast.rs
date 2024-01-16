#[derive(Debug)]
pub enum Expression {
    Number(i64),
    String(String),
    // Symbol(String),
    Expression {
        operator: String,
        operands: Vec<Expression>
    }
}
