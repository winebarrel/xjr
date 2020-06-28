extern crate serde_json;

use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io;

fn each_line(
  reader: &mut dyn io::prelude::BufRead,
  keys_orig: &Vec<String>,
  has_header: bool,
  split: fn(&String) -> Vec<String>,
  cb: fn(String),
) -> Result<(), Box<dyn std::error::Error>> {
  let mut keys = keys_orig.clone();
  let mut buf = String::new();

  if has_header {
    if reader.read_line(&mut buf)? == 0 {
      return Ok(());
    }

    keys = split(&buf.trim_end().to_string());
    buf.clear();
  }

  let to_json = if keys.len() > 0 {
    to_json_obj
  } else {
    to_json_array
  };

  while reader.read_line(&mut buf)? > 0 {
    let cols = split(&buf.trim_end().to_string());
    cb(to_json(&keys, &cols)?);
    buf.clear();
  }

  Ok(())
}

fn to_json_array(_keys: &Vec<String>, cols: &Vec<String>) -> Result<String, serde_json::Error> {
  serde_json::to_string(cols)
}

fn to_json_obj(keys: &Vec<String>, cols: &Vec<String>) -> Result<String, serde_json::Error> {
  let len = cmp::min(keys.len(), cols.len());
  let mut m = HashMap::new();

  for i in 0..len {
    m.insert(keys[i].clone(), cols[i].clone());
  }

  serde_json::to_string(&m)
}

fn split_line(str: &String) -> Vec<String> {
  if str == "" {
    vec![]
  } else {
    str.split(",").map(|s| s.to_string()).collect()
  }
}

fn split_line_empty(str: &String) -> Vec<String> {
  vec![str.clone()]
}

pub fn each_json_line(
  file: &String,
  sep: &String,
  keys: &Vec<String>,
  has_header: bool,
  cb: fn(String),
) -> Result<(), Box<dyn std::error::Error>> {
  let split = if sep.is_empty() {
    split_line_empty
  } else {
    split_line
  };

  if file == "-" {
    let mut reader = io::BufReader::new(io::stdin());
    each_line(&mut reader, keys, has_header, split, cb)
  } else {
    let f = fs::File::open(file)?;
    let mut reader = io::BufReader::new(f);
    each_line(&mut reader, keys, has_header, split, cb)
  }
}
