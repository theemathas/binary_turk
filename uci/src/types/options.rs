// TODO enumerate options.

use std::str::{FromStr, ParseBoolError};
use std::num::ParseIntError;
use std::error::FromError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct ParseValueError(());

#[derive(Debug)]
pub struct ParseNameError(());

impl FromError<ParseNameError> for ParseValueError {
    fn from_error(_: ParseNameError) -> Self { ParseValueError(()) }
}
impl FromError<ParseIntError> for ParseValueError {
    fn from_error(_: ParseIntError) -> Self { ParseValueError(()) }
}
impl FromError<ParseBoolError> for ParseValueError {
    fn from_error(_: ParseBoolError) -> Self { ParseValueError(()) }
}
// Required since String implements FromStr with Error = ()
impl FromError<()> for ParseValueError {
    fn from_error(_: ()) -> Self { ParseValueError(()) }
}

macro_rules! declare_type {
    ($name:ident, Check) =>  { pub type $name = bool; };
    ($name:ident, Spin)  =>  { pub type $name = i64; };
    ($name:ident, Combo) =>  { pub type $name = u32; };
    ($name:ident, Button) => { pub type $name = (); };
    ($name:ident, String) => { pub type $name = String; };
}

macro_rules! to_owned {
    ($value:expr, String) => { $value.to_string() };
    ($value:expr, $kind:ident) => { $value };
}

macro_rules! info_impl {
    ($name:ident : Check() = $default:expr) => { Info::Check(Name::$name, $default) };
    ($name:ident : Spin($min:expr, $max:expr) = $default:expr) => { Info::Spin(Name::$name, $default, $min, $max) };
    ($name:ident : Combo($($val:expr),+) = $default:expr) => { Info::Combo(Name::$name, $default, &[$($val,)+]) };
    ($name:ident : Button() = $default:expr) => { Info::Button(Name::$name, $default) };
    ($name:ident : String() = $default:expr) => { Info::String(Name::$name, $default) };
}

macro_rules! parse_value {
    ($name:expr, Combo, $value_string:expr) => {{
        let temp = $name as usize; // work around an ICE
        let combo_list: &[&'static str] = match INFO[temp] {
            Info::Combo(_, _, x) => &x,
            _ => unreachable!(),
        };
        combo_list.position_elem(&&*$value_string).map_or(Err(ParseValueError(())), |x| Ok(x as u32))
    }};
    ($name:expr, Button, $value_string:expr) => { Ok::<(), ParseValueError>(()) };
    ($name:expr, Spin, $value_string:expr) => {{
        let val: i64 = try!($value_string.parse());
        let temp = $name as usize; // work around an ICE
        let (min_val, max_val): (i64, i64) = match INFO[temp] {
            Info::Spin(_, _, x, y) => (x, y),
            _ => unreachable!(),
        };
        if val >= min_val && val <= max_val {
            Ok(val)
        } else {
            Err(ParseValueError(()))
        }
    }};
    ($name:expr, $kind:ident, $value_string:expr) => {
        $value_string.parse()
    };
}

macro_rules! options_impl {
    (($num_opt:expr) options
     $(
         $name:ident ($field_name:ident, $str:expr) : $kind:ident ($($info:tt),*) = $default:expr,
      )+) => {

        mod type_of {
            $(declare_type!($name, $kind);)+
        }

        #[derive(PartialEq, Eq, Clone, Debug)]
        pub enum Value {
            $($name(type_of::$name),)+
        }

        #[derive(PartialEq, Eq, Copy, Clone, Debug)]
        pub enum Name {
            $($name,)+
        }

        pub const INFO: [Info; $num_opt] = [$(
            info_impl!($name: $kind($($info),*) = $default),
        )+];

        #[derive(Clone, Debug)]
        pub struct Data {
            $(
                pub $field_name: type_of::$name,
             )+
        }
        impl Data {
            pub fn new() -> Data {
                Data {
                    $(
                        $field_name: to_owned!($default, $kind),
                     )+
                }
            }
            pub fn set_value(&mut self, val: Value) {
                match val {
                    $(
                        Value::$name(x) => self.$field_name = x,
                     )+
                }
            }
        }

        impl FromStr for Value {
            type Err = ParseValueError;
            fn from_str(s: &str) -> Result<Self, ParseValueError> {
                // consume everything up to and including "name"
                let mut words = s.split(' ').skip_while(|&s| s != "name");
                words.next();

                let mut name_vec = Vec::<&str>::new();
                let mut value_vec = Vec::<&str>::new();

                let mut found_value = false;
                for curr_word in words {
                    if found_value {
                        value_vec.push(curr_word);
                    } else {
                        if curr_word == "value" {
                            found_value = true;
                        } else {
                            name_vec.push(curr_word);
                        }
                    }
                }

                let name_string  =  name_vec.connect(" ").trim().to_string();
                let value_string = value_vec.connect(" ").trim().to_string();

                let name: Name = try!(name_string.parse());
                Ok(match name {
                    $(
                        Name::$name => Value::$name(try!(
                                parse_value!(Name::$name, $kind, value_string))),
                    )+
                })
            }
        }

        impl FromStr for Name {
            type Err = ParseNameError;
            fn from_str(s: &str) -> Result<Self, ParseNameError> {
                match s {
                    $(
                        $str => Ok(Name::$name),
                     )+
                    _ => Err(ParseNameError(())),
                }
            }
        }

        impl Display for Name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match *self {
                    $(
                        Name::$name => write!(f, $str),
                     )+
                }
            }
        }
    }
}

options_impl!{
    (5) options
    Hash(hash_size, "hash"): Spin (1, 1024) = 1,
    TestCheck(test_check, "testcheck"): Check () = false,
    TestCombo(test_combo, "testcombo"): Combo ("foo", "bar", "baz") = 0,
    TestButton(test_button, "testbutton"): Button () = (),
    TestString(test_string, "teststring"): String () = "something",
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Type {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

// The second fields of each variant are the default values
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Info {
    Check(Name, bool),
    Spin(Name, i64, i64, i64),
    Combo(Name, u32, &'static[&'static str]),
    Button(Name, ()),
    String(Name, &'static str),
}

impl Display for Info {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Info::Check(name, default) =>
                write!(f, "name {} type check default {}", name, default),
            Info::Spin(name, default, min, max) =>
                write!(f, "name {} type spin default {} min {} max {}", name, default, min, max),
            Info::Combo(name, default, choices) => {
                try!(write!(f, "name {} type combo default {}", name, choices[default as usize]));
                for s in choices {
                    try!(write!(f, " var {}", s));
                }
                Ok(())
            },
            Info::Button(name, _) =>
                write!(f, "name {} type button", name),
            Info::String(name, default) =>
                write!(f, "name {} type string default {}", name, default),
        }
    }
}
