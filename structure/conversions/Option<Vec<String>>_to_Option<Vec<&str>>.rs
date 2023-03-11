let option_vec_str: Option<Vec<String>> = Some(vec!["sdf".to_string(), "pif".to_string()]);

let option_vec_slice: Option<Vec<&str>> = option_vec_str.as_deref().map(|vec| vec.iter().map(String::as_str).collect());
