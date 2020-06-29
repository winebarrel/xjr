use super::each_json_line;

#[test]
fn test_each_json_line() {
  let mut reader = b"foo,bar\nbar,zoo" as &[u8];
  let mut jsonl = vec![];

  each_json_line(&mut reader, &",", &vec![], false, |line| {
    jsonl.push(line);
  })
  .unwrap();

  assert_eq!(jsonl, vec![r#"["foo","bar"]"#, r#"["bar","zoo"]"#]);
}

#[test]
fn test_each_json_line_with_sep() {
  let mut reader = b"foo\tbar\nbar\tzoo" as &[u8];
  let mut jsonl = vec![];

  each_json_line(&mut reader, &"\t", &vec![], false, |line| {
    jsonl.push(line);
  })
  .unwrap();

  assert_eq!(jsonl, vec![r#"["foo","bar"]"#, r#"["bar","zoo"]"#]);
}

#[test]
fn test_each_json_line_without_sep() {
  let mut reader = b"foo,bar\nbar,zoo" as &[u8];
  let mut jsonl = vec![];

  each_json_line(&mut reader, &"", &vec![], false, |line| {
    jsonl.push(line);
  })
  .unwrap();

  assert_eq!(jsonl, vec![r#"["foo,bar"]"#, r#"["bar,zoo"]"#]);
}

#[test]
fn test_each_json_line_with_keys() {
  let mut reader = b"foo,bar\nbar,zoo" as &[u8];
  let mut jsonl = vec![];

  each_json_line(
    &mut reader,
    &",",
    &vec!["a".to_string(), "b".to_string()],
    false,
    |line| {
      jsonl.push(line);
    },
  )
  .unwrap();

  assert_eq!(
    jsonl,
    vec![r#"{"a":"foo","b":"bar"}"#, r#"{"a":"bar","b":"zoo"}"#]
  );
}

#[test]
fn test_each_json_line_with_header() {
  let mut reader = b"foo,bar\nzoo,baz\n1,2" as &[u8];
  let mut jsonl = vec![];

  each_json_line(&mut reader, &",", &vec![], true, |line| {
    jsonl.push(line);
  })
  .unwrap();

  assert_eq!(
    jsonl,
    vec![r#"{"bar":"baz","foo":"zoo"}"#, r#"{"bar":"2","foo":"1"}"#]
  );
}

#[test]
fn test_each_json_line_with_tab_separated_header() {
  let mut reader = b"foo\tbar\nzoo\tbaz\n1\t2" as &[u8];
  let mut jsonl = vec![];

  each_json_line(&mut reader, &"\t", &vec![], true, |line| {
    jsonl.push(line);
  })
  .unwrap();

  assert_eq!(
    jsonl,
    vec![r#"{"bar":"baz","foo":"zoo"}"#, r#"{"bar":"2","foo":"1"}"#]
  );
}
