use pest;
use pest::Parser;
use pest::iterators::Pairs;

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
    };

}
