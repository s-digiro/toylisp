use super::*;

#[test]
pub fn lambda_with_1_arg_and_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
        ]),
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::number(1),
        ]),
    ]);

    let expected =  SXRef::function(Function::new(
        vec!["x".into()],
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::number(1),
        ]),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_3_arg_and_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into(),
            "z".into(),
        ],
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_0_arg_and_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::nil(),
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
    ]);

    let empty: Vec<String> = Vec::new();

    let expected =  SXRef::function(Function::new(
        empty,
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_number_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::number(1),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_string_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::string("foo".into()),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::string("foo".into()),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_symbol_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::symbol("foo".into()),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::symbol("foo".into()),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_quote_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::quote(SXRef::symbol("foo".into())),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::quote(SXRef::symbol("foo".into())),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_nil_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::nil(),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::nil(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_function_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::function("()".try_into().unwrap()),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::function("()".try_into().unwrap()),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_macro_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::r#macro("()".try_into().unwrap()),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::r#macro("()".try_into().unwrap()),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_rust_function_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::rust_function(RustFunction::new(dummy_fn)),
    ]);

    let expected_args: Vec<String> = vec![
        "x".into(),
        "y".into()
    ];

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    match &*actual {
        SX::Function(f) => {
            assert_eq!(&expected_args, f.args());

            assert!(f.definition().is_rust_function());
        },
        _ => panic!("Expected SX::Function. Recieved: {:?}", actual),
    }
}

#[test]
pub fn lambda_with_2_arg_and_rust_macro_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::rust_macro(RustMacro::new(dummy_macro)),
    ]);

    let expected_args: Vec<String> = vec![
        "x".into(),
        "y".into()
    ];

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    match &*actual {
        SX::Function(f) => {
            assert_eq!(&expected_args, f.args());

            assert!(f.definition().is_rust_macro());
        },
        _ => panic!("Expected SX::Function. Recieved: {:?}", actual),
    }
}

#[test]
pub fn lambda_with_2_arg_and_no_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
    ]);

    let expected =  SXRef::function(Function::new(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::nil(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_no_args_and_no_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
    ]);

    let empty: Vec<String> = Vec::new();

    let expected =  SXRef::function(Function::new(
        empty,
        SXRef::nil(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_non_list_args_returns_function_that_takes_no_args() {
    let empty: Vec<String> = Vec::new();

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::number(1),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::new(
        empty.clone(),
        SXRef::number(1),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::symbol("foo".into()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::new(
        empty.clone(),
        SXRef::number(1),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::string("foo".into()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::new(
        empty.clone(),
        SXRef::number(1),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::quote(SXRef::string("foo".into())),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::new(
        empty.clone(),
        SXRef::number(1),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::r#macro("()".try_into().unwrap()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::new(
        empty.clone(),
        SXRef::number(1),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::function("()".try_into().unwrap()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::new(
        empty.clone(),
        SXRef::number(1),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::rust_function(RustFunction::new(dummy_fn)),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::new(
        empty.clone(),
        SXRef::number(1),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::rust_macro(RustMacro::new(dummy_macro)),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::new(
        empty.clone(),
        SXRef::number(1),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual);
}
