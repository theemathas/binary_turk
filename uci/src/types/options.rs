// TODO enumerate options.

pub type NameAndVal = (Name, Val);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Name {
    Dummy,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Val {
    Check(bool),
    Spin(i64),
    Combo(String),
    Button,
    String(String),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Type {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Info {
    Check,
    Spin(i64, i64),
    Combo(Vec<String>),
    Button,
    String,
}
