#[derive(Debug, Clone, Copy)]
pub enum BasicOp {
    Add, // a+b
    Sub, // a-b
    Mul, // a*b
    Div, // a/b
}

#[derive(Debug, Clone, Copy)]
pub enum Expo {
    Exp, // a^b
    Log, // a,b
}

#[derive(Debug, Clone, Copy)]
pub enum Tri {
    Sin, // a, Undefined
    Con, // a, Undefined
    Tan, // a, Undefined
    Arcsin, // a, Undefined
    Arccos, // a, Undefined
    Arctan, // a, Undefined
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Basic(BasicOp),
    Expo(Expo),
    Tri(Tri)
}