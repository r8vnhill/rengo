pub(crate) enum Expression {
    Number(i64),

    Increment(Box<Expression>),

    Decrement(Box<Expression>),
}
