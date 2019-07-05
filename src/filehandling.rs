///
/// 2019 (C) Christopher Höllriegl
///
///
use std::fs::File;
use std::io::{Read, BufReader, BufRead, Seek};
use core::borrow::Borrow;

fn open_file() -> File
{
    //let mut file  = File::open("application.config");
    //return file;
    return File::open("D:\\WorkspaceMQ\\MQSystem\\Test\\testtool_rust\\target\\debug\\application.config").unwrap();

    // TODO error handling, not safe!
    // TODO Which Filepath to use?
}

///
///  Return the value from file
///
pub fn get_string(_value: &str) -> String
{
    let f: File = open_file();
    let mut buf_reader: BufReader<File> = BufReader::new(f);
    let mut result: String = String::new();


    for line in buf_reader.lines() {
        let mut s: String = String::new();
        s.clear();
        s = line.unwrap();
        //buf_reader.read_line(&mut s);
        if s.contains(_value)
        {
            //result.add(s);
            //result = s;
            result.push_str(&s);
            break;
        }
    }

    let mut split  = result.split("=");

    let ret: Vec<&str> = split.collect();
    result = ret[1].to_string();
    return result;
}