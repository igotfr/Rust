let option_vec_str: Option<Vec<String>> = Some(vec!["sdf".to_string(), "pif".to_string()]);

let option_vec_tuple: Option<(&str, &str)> = option_vec_str.as_ref().and_then(|v| v.get(0).and_then(|s1| v.get(1).map(|s2| (s1.as_str(), s2.as_str()))));
