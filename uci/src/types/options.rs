// TODO enumerate options.

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Value {
    Dummy(i64),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Name {
    Dummy,
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
