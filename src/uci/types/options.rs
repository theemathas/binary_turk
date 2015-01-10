// TODO enumerate options.

pub type NameAndVal = (Name, Val);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub enum Name {
    Dummy,
}

#[derive(PartialEq, Eq, Clone, Show)]
pub enum Val {
    Check(bool),
    Spin(i64),
    Combo(String),
    Button,
    String(String),
}

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub enum Type {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

#[derive(PartialEq, Eq, Clone, Show)]
pub enum Info {
    Check,
    Spin(i64, i64),
    Combo(Vec<String>),
    Button,
    String,
}
