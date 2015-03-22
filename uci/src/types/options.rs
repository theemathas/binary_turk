// TODO enumerate options.

use std::str::{FromStr, ParseBoolError};
use std::num::ParseIntError;
use std::error::FromError;

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
    ($name:ident, Check) =>  { type $name = bool; };
    ($name:ident, Spin)  =>  { type $name = i64; };
    ($name:ident, Combo) =>  { type $name = u32; };
    ($name:ident, Button) => { type $name = (); };
    ($name:ident, String) => { type $name = String; };
}

macro_rules! info_impl {
    (Check()) => { Info::Check };
    (Spin($min:expr, $max:expr)) => { Info::Spin($min, $max) };
    (Combo($($val:expr),+)) => { Info::Combo(&[$($val,)+]) };
    (Button()) => { Info::Button };
    (String()) => { Info::String };
}

macro_rules! parse_value {
    ($name:expr, Combo, $value_string:expr) => {{
        let temp = $name as usize; // work around an ICE
        let combo_list: &[&'static str] = match INFO[temp] {
            Info::Combo(x) => &x,
            _ => unreachable!(),
        };
        combo_list.position_elem(&&*$value_string).map(|x| x as u32).ok_or(ParseValueError(()))
    }};
    ($name:expr, Button, $value_string:expr) => { Ok::<(), ParseValueError>(()) };
    ($name:expr, Spin, $value_string:expr) => {{
        let val: i64 = try!($value_string.parse());
        let temp = $name as usize; // work around an ICE
        let (min_val, max_val): (i64, i64) = match INFO[temp] {
            Info::Spin(x, y) => (x, y),
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
         $name:ident ($repr:expr) : $kind:ident ($($info:tt),*),
      )+) => {

        $(declare_type!($name, $kind);)+

        #[derive(PartialEq, Eq, Clone, Debug)]
        pub enum Value {
            $($name($name),)+
        }

        #[derive(PartialEq, Eq, Copy, Clone, Debug)]
        pub enum Name {
            $($name,)+
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
                        $repr => Ok(Name::$name),
                     )+
                    _ => Err(ParseNameError(())),
                }
            }
        }

        const INFO: [Info; $num_opt] = [$(
            info_impl!($kind($($info),*)),
        )+];
    }
}

options_impl!{
    (1) options
    Hash("hash"): Spin (1, 1024),
    //TestCheck("testcheck"): Check (),
    //TestCombo("testcombo"): Combo ("foo", "bar", "baz"),
    //TestButton("testbutton"): Button (),
    //TestString("teststring"): String (),
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
    Combo(&'static[&'static str]),
    Button,
    String,
}
