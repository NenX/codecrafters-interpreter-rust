use super::Stmt;

#[derive(Clone, Debug)]

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}
impl From<Vec<Stmt>> for BlockStmt {
    fn from(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}
impl BlockStmt {
    pub fn from(arr: impl IntoIterator<Item = Stmt>) -> Self {
        Self {
            statements: arr.into_iter().collect(),
        }
    }
    pub fn push(&mut self, value: Stmt) {
        self.statements.push(value);
    }
}

#[test]
fn aa() {
    let a = [1, 2, 3];
}
