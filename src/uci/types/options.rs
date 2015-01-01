// TODO enumerate options.

pub type NameAndVal = (Name, Val);

pub enum Name {
    Dummy,
}

pub enum Val {
    Check(bool),
    Spin(i64),
    Combo(String),
    Button,
    String(String),
}

pub enum Type {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

pub enum Info {
    Check,
    Spin(i64, i64),
    Combo(Vec<String>),
    Button,
    String,
}
