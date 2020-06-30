extern crate serde_json;

#[cfg(test)]
mod tests;

use std::cmp;
use std::collections::BTreeMap;
use std::error;
use std::io;

pub fn each_json_line<T, F>(
  mut reader: T,
  sep: &str,
  keys_orig: &Vec<String>,
  has_header: bool,
  mut cb: F,
) -> Result<(), Box<dyn error::Error>>
where
  T: io::prelude::BufRead,
  F: FnMut(&str),
{
  let mut keys = keys_orig.clone();
  let mut buf = String::new();

  let split = if sep.is_empty() {
    split_line_empty
  } else {
    split_line
  };

  if has_header {
    if reader.read_line(&mut buf)? == 0 {
      return Ok(());
    }

    keys = split(&buf.trim_end(), sep);
    buf.clear();
  }

  let to_json = if keys.len() > 0 {
    to_json_obj
  } else {
    to_json_array
  };

  while reader.read_line(&mut buf)? > 0 {
    let cols = split(&buf.trim_end(), sep);
    let json = to_json(&keys, &cols)?;
    cb(&json);
    buf.clear();
  }

  Ok(())
}

fn to_json_array(_keys: &Vec<String>, cols: &Vec<String>) -> Result<String, serde_json::Error> {
  serde_json::to_string(cols)
}

fn to_json_obj(keys: &Vec<String>, cols: &Vec<String>) -> Result<String, serde_json::Error> {
  let len = cmp::min(keys.len(), cols.len());
  let mut m = BTreeMap::new();

  for i in 0..len {
    m.insert(&keys[i], &cols[i]);
  }

  serde_json::to_string(&m)
}

fn split_line(s: &str, sep: &str) -> Vec<String> {
  if s == "" {
    vec![]
  } else {
    s.split(sep).map(|c| c.to_string()).collect()
  }
}

fn split_line_empty(s: &str, _sep: &str) -> Vec<String> {
  vec![s.to_string()]
}
