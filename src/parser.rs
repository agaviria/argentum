#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("argentum.pest");

#[derive(Parser)]
#[grammar = "argentum.pest"]
pub struct SilverParser;

#[test]
fn t_literal() {
    parses_to! {
        parser: SilverParser,
        input: "true",
        rule: Rule::bool,
        tokens: [
            bool(0, 4, [
                 t_literal(0, 4)
            ])
        ]
    };
}

#[test]
fn f_literal() {
    parses_to! {
        parser: SilverParser,
        input: "false",
        rule: Rule::bool,
        tokens: [
            bool(0, 5, [
                 f_literal(0, 5)
            ])
        ]
    };
}

#[test]
fn zero() {
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
fn starts_with_zero() {
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
fn zero_with_underscores() {
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
fn million() {
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
fn zero_dot() {
    parses_to! {
        parser: SilverParser,
        input: "0.",
        rule: Rule::float,
        tokens: [
            float(0, 2, [
                  int(0, 1)
            ])
        ]
    };
}

#[test]
fn zero_dot_zero() {
    parses_to! {
        parser: SilverParser,
        input: "0.0",
        rule: Rule::float,
        tokens: [
            float(0, 3, [
                  int(0, 1),
                  int(2, 3)
            ])
        ]
    };
}
