use crate::filehandling::get_string;
use std::fs::File;
use std::io::Read;

///
/// 2019 (C) Christopher Höllriegl
///
///

mod filehandling;

///
/// Return the value from config as String
///
pub fn return_as_string(_value: &str) -> String
{
    return get_string(_value);
}

///
/// Return the value from config as int32
///
pub fn return_as_int32(_value: &str) -> i32
{
    // TODO implement return_as_int32
    return 2;
}

///
/// Return the value from config as bool
///
pub fn return_as_bool(_value: &str) -> bool
{
    // TODO implement return_as_bool
    // Maybe just compare string length, faster than full string compare?
    return true;
}

///
/// Return the value from config as u16 (unsigned short)
///
pub fn return_as_u16(_value: &str) -> u16
{
    // TODO implement return_as_u16
    return 0;
}
