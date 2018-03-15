use pest;
use pest::Parser;
use pest::iterators::Pairs;
// use parser::Rule::inner_str;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("argentum.pest");

#[derive(Parser)]
#[grammar = "argentum.pest"]
pub struct SilverParser;

/// Error encountered while decoding Silver data.
#[derive(Debug)]
pub enum ParseError<'i> {
    Pest(pest::Error<'i, Rule>),
}

/// Parse Silver data contained in a string slice.
pub fn parse(input: &str) -> Result<Pairs<Rule>, ParseError> {
    // TODO: need to implement a toplevel rule in silverparser.
    SilverParser::parse(Rule::top_lvl, input).map_err(|error| ParseError::Pest(error))
}

#[test]
fn bool_literal_true() {
    parses_to! {
        parser: SilverParser,
        input: "true",
        rule: Rule::bool_literal,
        tokens: [
            bool_literal(0, 4, [
                         bool_true(0, 4)
            ])
        ]
    };
}

#[test]
fn bool_literal_false() {
    parses_to! {
        parser: SilverParser,
        input: "false",
        rule: Rule::bool_literal,
        tokens: [
            bool_literal(0, 5, [
                         bool_false(0, 5)
            ])
        ]
    };
}

#[test]
fn int_zero() {
    parses_to! {
        parser: SilverParser,
        input: "0",
        rule: Rule::int,
        tokens: [
            int(0, 1)
        ]
    };
}

#[test]
fn int_start_with_zero() {
    parses_to! {
        parser: SilverParser,
        input: "01",
        rule: Rule::int,
        tokens: [
            int(0, 2)
        ]
    };
}

#[test]
fn int_with_zero_and_underscores() {
    parses_to! {
        parser: SilverParser,
        input: "0___",
        rule: Rule::int,
        tokens: [
            int(0, 4)
        ]
    };
}

#[test]
fn int_one_million() {
    parses_to! {
        parser: SilverParser,
        input: "1_000_000",
        rule: Rule::int,
        tokens: [
            int(0, 9)
        ]
    };
}

#[test]
fn float_zero_dot() {
    parses_to! {
        parser: SilverParser,
        input: "0.",
        rule: Rule::float_literal,
        tokens: [
            float_literal(0, 2)
        ]
    };
}

#[test]
fn float_zero_dot_zero() {
    parses_to! {
        parser: SilverParser,
        input: "0.0",
        rule: Rule::float_literal,
        tokens: [
            float_literal(0, 3)
        ]
    };
}

#[test]
fn float_one_dot_five_exponent() {
    parses_to! {
        parser: SilverParser,
        input: "1.5e10",
        rule: Rule::float_literal,
        tokens: [
            float_literal(0, 6)
        ]
    };
}

#[test]
fn float_zero_dot_zero_exp_plus() {
    parses_to! {
        parser: SilverParser,
        input: "0.0e-0",
        rule: Rule::float_literal,
        tokens: [
            float_literal(0, 6)
        ]
    };
}

#[test]
fn escape_with_new_line_sequence() {
    parses_to! {
        parser: SilverParser,
        input: r#"\n"#,
        rule: Rule::escape,
        tokens: [
            escape(0, 2)
        ]
    };

}

#[test]
fn escape_with_carriage_return_sequence() {
    parses_to! {
        parser: SilverParser,
        input: r#"\r"#,
        rule: Rule::escape,
        tokens: [
            escape(0, 2)
        ]
    };

}

#[test]
fn escape_with_horizontal_tab_sequence() {
    parses_to! {
        parser: SilverParser,
        input: r#"\t"#,
        rule: Rule::escape,
        tokens: [
            escape(0, 2)
        ]
    };
}

#[test]
fn escape_with_vertical_tab_sequence() {
    parses_to! {
        parser: SilverParser,
        input: r#"\v"#,
        rule: Rule::escape,
        tokens: [
            escape(0, 2)
        ]
    };
}

#[test]
fn escape_with_null_sequence() {
    parses_to! {
        parser: SilverParser,
        input: r#"\0"#,
        rule: Rule::escape,
        tokens: [
            escape(0, 2, [
                   octal_escape(0, 2, [
                                digit_octal(1, 2)
                   ])
            ])
        ]
    };

}

#[test]
fn escape_with_hex_representation() {
    parses_to! {
        parser: SilverParser,
        input: r#"\x08"#,
        rule: Rule::escape,
        tokens: [
            escape(0, 4, [
                   hex_escape(0, 4, [
                              hex(2, 3),
                              hex(3, 4)
                   ])
            ])
        ]
    };

}

#[test]
fn escape_with_alert_sequence() {
    parses_to! {
        parser: SilverParser,
        input: r#"\a"#,
        rule: Rule::escape,
        tokens: [
            escape(0, 2)
        ]
    }
}

#[test]
fn char_without_escape() {
    parses_to! {
        parser: SilverParser,
        input: "'A'",
        rule: Rule::char_literal,
        tokens: [
            char_literal(0, 3, [
                         raw_char(1, 2)
            ])
        ]
    };
}

#[test]
fn char_with_escape() {
    parses_to! {
        parser: SilverParser,
        input: "'\\''",
        rule: Rule::char_literal,
        tokens: [
            char_literal(0, 4, [
                         escape(1, 3)
            ])
        ]
    };

}

#[test]
fn string_without_escape() {
    parses_to! {
        parser: SilverParser,
        input: "\"The Matrix\"",
        rule: Rule::str_literal,
        tokens: [
            str_literal(0, 12, [
                        raw_str(1, 11)
            ])
        ]
    };

}

#[test]
fn string_with_escape() {
    parses_to! {
        parser: SilverParser,
        input: r#""a\nb\x0Fc\u2107Now""#,
        rule: Rule::str_literal,
        tokens: [
            str_literal(0, 20, [
                        raw_str(1, 2),
                        escape(2, 4),
                        raw_str(4, 5),
                        escape(5, 9, [
                               hex_escape(5, 9, [
                                          hex(7, 8),
                                          hex(8, 9)
                               ])
                        ]),
                        raw_str(9, 10),
                        escape(10, 16, [
                               unicode_escape(10, 16, [
                                              hex(12, 13),
                                              hex(13, 14),
                                              hex(14, 15),
                                              hex(15, 16)
                               ])
                        ]),
                        raw_str(16, 19)
            ])
            ]
    };

}

#[test]
fn typecheck_char() {
    parses_to! {
        parser: SilverParser,
        input: "char",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 4, [
                      char_type(0, 4)
            ])
        ]
    };
}

#[test]
fn typecheck_void() {
    parses_to! {
        parser: SilverParser,
        input: "void",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 4, [
                      void_type(0, 4)
            ])
        ]
    };
}

#[test]
fn typecheck_str() {
    parses_to! {
        parser: SilverParser,
        input: "str",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 3, [
                      str_type(0, 3)
            ])
        ]
    };
}

#[test]
fn typecheck_bool() {
    parses_to! {
        parser: SilverParser,
        input: "bool",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 4, [
                      bool_type(0, 4)
            ])
        ]
    };
}

#[test]
fn typecheck_i8() {
    parses_to! {
        parser: SilverParser,
        input: "i8",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 2, [
                      i8_type(0, 2)
            ])
        ]
    };
}

#[test]
fn typecheck_u8() {
    parses_to! {
        parser: SilverParser,
        input: "u8",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 2, [
                      u8_type(0, 2)
            ])
        ]
    };
}

#[test]
fn typecheck_i16() {
    parses_to! {
        parser: SilverParser,
        input: "i16",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 3, [
                      i16_type(0, 3)
            ])
        ]
    };
}

#[test]
fn typecheck_u16() {
    parses_to! {
        parser: SilverParser,
        input: "u16",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 3, [
                      u16_type(0, 3)
            ])
        ]
    };
}

#[test]
fn typecheck_u32() {
    parses_to! {
        parser: SilverParser,
        input: "u32",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 3, [
                      u32_type(0, 3)
            ])
        ]
    };
}

#[test]
fn typecheck_i32() {
    parses_to! {
        parser: SilverParser,
        input: "i32",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 3, [
                      i32_type(0, 3)
            ])
        ]
    };
}

#[test]
fn typecheck_f32() {
    parses_to! {
        parser: SilverParser,
        input: "f32",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 3, [
                      f32_type(0, 3)
            ])
        ]
    };
}

#[test]
fn typecheck_i64() {
    parses_to! {
        parser: SilverParser,
        input: "i64",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 3, [
                      i64_type(0, 3)
            ])
        ]
    };
}

#[test]
fn typecheck_u64() {
    parses_to! {
        parser: SilverParser,
        input: "u64",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 3, [
                      u64_type(0, 3)
            ])
        ]
    };
}

#[test]
fn typecheck_f64() {
    parses_to! {
        parser: SilverParser,
        input: "f64",
        rule: Rule::prim_type,
        tokens: [
            prim_type(0, 3, [
                      f64_type(0, 3)
            ])
        ]
    };
}
