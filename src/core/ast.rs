pub mod ast {
    pub enum Literal {
        Number(i64),
        String(String),
        Symbol(String)
    }

    pub struct Expression {
        pub operator: String,
        pub operands: Vec<Expression>
    }
}
