use crate::CsvParser;

#[test]
fn parser_base() {
    let expect: Vec<Vec<String>> = vec![
        vec![String::from("1"), String::from("2"), String::from("3")],
        vec![String::from("a"), String::from("b"), String::from("c")],
    ];

    let str = expect.iter().fold(String::new(), |mut str, row| {
        str.push_str(
            row.iter()
                .enumerate()
                .fold(String::new(), |mut inner_str, (i, field)| {
                    inner_str.push_str(field.as_str());
                    if i < row.len() - 1 {
                        inner_str.push(',');
                    }
                    inner_str
                })
                .as_str(),
        );
        str.push('\r');
        str
    });
    let mut parser = CsvParser::from_string(str);

    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };

    println!("token: {tokens:?}");
    for (token, exp) in tokens.iter().zip(expect) {
        assert_eq!(token.value, exp);
    }
}

#[test]
fn parser_trim_fail() {
    let expect: Vec<Vec<String>> = vec![
        vec![String::from("1 "), String::from("2 "), String::from("3")],
        vec![String::from("a"), String::from("b "), String::from("c")],
    ];

    let str = expect.iter().fold(String::new(), |mut str, row| {
        str.push_str(
            row.iter()
                .enumerate()
                .fold(String::new(), |mut inner_str, (i, field)| {
                    inner_str.push_str(field.as_str());
                    if i < row.len() - 1 {
                        inner_str.push(',');
                    }
                    inner_str
                })
                .as_str(),
        );
        str.push('\r');
        str
    });
    let mut parser = CsvParser::from_string(str);

    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };

    for (token, exp) in tokens.iter().zip(expect) {
        let trimmed_exp = exp
            .iter()
            .map(|x| x.trim().to_owned())
            .collect::<Vec<String>>();
        assert_ne!(token.value, trimmed_exp);
    }
}

#[test]
fn parser_trim() {
    let expect: Vec<Vec<String>> = vec![
        vec![String::from("1 "), String::from("2 "), String::from("3")],
        vec![String::from("a"), String::from("b "), String::from("c")],
    ];

    let str = expect.iter().fold(String::new(), |mut str, row| {
        str.push_str(
            row.iter()
                .enumerate()
                .fold(String::new(), |mut inner_str, (i, field)| {
                    inner_str.push_str(field.as_str());
                    if i < row.len() - 1 {
                        inner_str.push(',');
                    }
                    inner_str
                })
                .as_str(),
        );
        str.push('\r');
        str
    });
    let mut parser = CsvParser::from_string(str).trim_space(true);

    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };

    for (token, exp) in tokens.iter().zip(expect) {
        let trimmed_exp = exp
            .iter()
            .map(|x| x.trim().to_owned())
            .collect::<Vec<String>>();
        assert_eq!(token.value, trimmed_exp);
    }
}

#[test]
fn parser_all_whitespace_fail() {
    let expect: Vec<Vec<String>> = vec![
        vec![String::from("1 "), String::from("  "), String::from("3")],
        vec![String::from("a"), String::from(" "), String::from("")],
    ];

    let str = expect.iter().fold(String::new(), |mut str, row| {
        str.push_str(
            row.iter()
                .enumerate()
                .fold(String::new(), |mut inner_str, (i, field)| {
                    inner_str.push_str(field.as_str());
                    if i < row.len() - 1 {
                        inner_str.push(',');
                    }
                    inner_str
                })
                .as_str(),
        );
        str.push('\r');
        str
    });
    let mut parser = CsvParser::from_string(str);

    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };

    for (token, exp) in tokens.iter().zip(expect) {
        let trimmed_exp = exp
            .iter()
            .map(|x| {
                if x.trim().is_empty() {
                    x.trim().to_owned()
                } else {
                    x.to_string()
                }
            })
            .collect::<Vec<String>>();
        assert_ne!(token.value, trimmed_exp);
    }
}

#[test]
fn parser_all_whitespace() {
    let expect: Vec<Vec<String>> = vec![
        vec![String::from("1 "), String::from("  "), String::from("3")],
        vec![String::from("a"), String::from(" "), String::from("")],
    ];

    let str = expect.iter().fold(String::new(), |mut str, row| {
        str.push_str(
            row.iter()
                .enumerate()
                .fold(String::new(), |mut inner_str, (i, field)| {
                    inner_str.push_str(field.as_str());
                    if i < row.len() - 1 {
                        inner_str.push(',');
                    }
                    inner_str
                })
                .as_str(),
        );
        str.push('\r');
        str
    });
    let mut parser = CsvParser::from_string(str).all_whitespace_empty(true);

    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };

    for (token, exp) in tokens.iter().zip(expect) {
        let trimmed_exp = exp
            .iter()
            .map(|x| {
                if x.trim().is_empty() {
                    x.trim().to_owned()
                } else {
                    x.to_string()
                }
            })
            .collect::<Vec<String>>();
        assert_eq!(token.value, trimmed_exp);
    }
}

#[test]
fn parser_escaped() {
    let expect: Vec<Vec<String>> = vec![
        vec![
            String::from("\"\"\"1\"\"\""),
            String::from("\",\""),
            String::from("3"),
        ],
        vec![String::from("a"), String::from(" "), String::from("")],
    ];

    let str = expect.iter().fold(String::new(), |mut str, row| {
        str.push_str(
            row.iter()
                .enumerate()
                .fold(String::new(), |mut inner_str, (i, field)| {
                    inner_str.push_str(field.as_str());
                    if i < row.len() - 1 {
                        inner_str.push(',');
                    }
                    inner_str
                })
                .as_str(),
        );
        str.push('\r');
        str
    });
    let mut parser = CsvParser::from_string(str);

    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };

    for (token, exp) in tokens.iter().zip(expect) {
        let new_exp = exp
            .iter()
            .map(|field| {
                if field.starts_with("\"") {
                    field[1..field.len() - 1].replace("\"\"", "\"").to_owned()
                } else {
                    field.clone()
                }
            })
            .collect::<Vec<String>>();
        assert_eq!(token.value, new_exp);
    }
}

#[test]
fn parser_escaped_has_newline() {
    let expect: Vec<Vec<String>> = vec![
        vec![
            String::from("\"\"\"1\"\"\""),
            String::from("\",\""),
            String::from("\",\""),
            String::from("3"),
        ],
        vec![
            String::from("a"),
            String::from(" "),
            String::from("\"ba\""),
            String::from("\"ab\""),
        ],
    ];

    let str = expect.iter().fold(String::new(), |mut str, row| {
        str.push_str(
            row.iter()
                .enumerate()
                .fold(String::new(), |mut inner_str, (i, field)| {
                    inner_str.push_str(field.as_str());
                    if i < row.len() - 1 {
                        inner_str.push(',');
                    }
                    inner_str
                })
                .as_str(),
        );
        str.push('\r');
        str
    });
    let mut parser = CsvParser::from_string(str);

    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };

    for (token, exp) in tokens.iter().zip(expect) {
        let new_exp = exp
            .iter()
            .map(|field| {
                if field.starts_with("\"") {
                    field[1..field.len() - 1].replace("\"\"", "\"").to_owned()
                } else {
                    field.clone()
                }
            })
            .collect::<Vec<String>>();
        assert_eq!(token.value, new_exp);
    }
}

#[test]
#[should_panic]
fn parser_escaped_fail() {
    let expect: Vec<Vec<String>> = vec![
        vec![
            String::from("\"\"\"1\"\"\""),
            String::from("\",\""),
            String::from("\"\",\""),
            String::from("3"),
        ],
        vec![
            String::from("a"),
            String::from(" "),
            String::from(""),
            String::from("\"\r\r\""),
        ],
    ];

    let str = expect.iter().fold(String::new(), |mut str, row| {
        str.push_str(
            row.iter()
                .enumerate()
                .fold(String::new(), |mut inner_str, (i, field)| {
                    inner_str.push_str(field.as_str());
                    if i < row.len() - 1 {
                        inner_str.push(',');
                    }
                    inner_str
                })
                .as_str(),
        );
        str.push('\r');
        str
    });
    let mut parser = CsvParser::from_string(str);

    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };

    for (token, exp) in tokens.iter().zip(expect) {
        let new_exp = exp
            .iter()
            .map(|field| {
                if field.starts_with("\"") {
                    field[1..field.len() - 1].replace("\"\"", "\"").to_owned()
                } else {
                    field.clone()
                }
            })
            .collect::<Vec<String>>();
        assert_ne!(token.value, new_exp);
    }
}

#[test]
fn parser_empty_file() {
    let str = "";
    let mut parser = CsvParser::from_string(str.to_string());
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };
    assert!(tokens.is_empty());
}

#[test]
#[should_panic]
fn parser_irregular_rows() {
    let str = "1,2,3\n4,5\n6";
    let mut parser = CsvParser::from_string(str.to_string());
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].value, vec!["1", "2", "3"]);
    assert_eq!(tokens[1].value, vec!["4", "5"]);
    assert_eq!(tokens[2].value, vec!["6"]);
}

#[test]
fn parser_empty_fields() {
    let str = ",,,\n,,,";
    let mut parser = CsvParser::from_string(str.to_string());
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].value, vec!["", "", "", ""]);
    assert_eq!(tokens[1].value, vec!["", "", "", ""]);
}

#[test]
fn parser_trailing_newline() {
    let str = "1,2,3\n4,5,6\n";
    let mut parser = CsvParser::from_string(str.to_string());
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].value, vec!["1", "2", "3"]);
    assert_eq!(tokens[1].value, vec!["4", "5", "6"]);
}

/*
#[test]
fn parser_quoted_fields_with_commas() {
    let str = "\"field, with, commas\",2,3";
    let mut parser = CsvParser::from_string(str.to_string());
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, vec!["field, with, commas", "2", "3"]);
}

#[test]
fn parser_single_field() {
    let str = "singlefield";
    let mut parser = CsvParser::from_string(str.to_string());
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, vec!["singlefield"]);
}

#[test]
fn parser_quoted_fields_with_newline() {
    let str = "\"field\nwith\nnewlines\",2,3";
    let mut parser = CsvParser::from_string(str.to_string());
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, vec!["field\nwith\nnewlines", "2", "3"]);
}

#[test]
fn parser_escaped_quotes() {
    let str = "\"field with \"\"escaped quotes\"\"\",2,3";
    let mut parser = CsvParser::from_string(str.to_string());
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{err:?}"),
    };
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].value,
        vec!["field with \"escaped quotes\"", "2", "3"]
    );
}
*/
