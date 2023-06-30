pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statements(&self);
}

pub trait Expression: Node {
    fn expression(self);
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        return "".to_string();
    }
}
