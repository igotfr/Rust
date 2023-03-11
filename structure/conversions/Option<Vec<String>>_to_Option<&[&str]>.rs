let option_vec_str: Option<Vec<String>> = Some(vec!["sdf".to_string(), "pif".to_string()]);

let tmp = option_vec_str.as_ref().map(|v| v.iter().map(String::as_str).collect::<Vec<&str>>());
let option_slice_str: Option<&[&str]> = tmp.as_ref().map(|s| s.as_slice());
