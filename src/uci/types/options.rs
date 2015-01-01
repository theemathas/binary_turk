// TODO enumerate options.

pub type NameAndVal = (Name, Val);

#[deriving(PartialEq,Eq,Copy,Clone)]
pub enum Name {
    Dummy,
}

#[deriving(PartialEq, Eq, Clone)]
pub enum Val {
    Check(bool),
    Spin(i64),
    Combo(String),
    Button,
    String(String),
}

#[deriving(PartialEq,Eq,Copy,Clone)]
pub enum Type {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

#[deriving(PartialEq, Eq, Clone)]
pub enum Info {
    Check,
    Spin(i64, i64),
    Combo(Vec<String>),
    Button,
    String,
}
