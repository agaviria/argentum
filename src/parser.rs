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

#[test]
fn one_exp() {
    parses_to! {
        parser: SilverParser,
        input: "1e10",
        rule: Rule::float,
        tokens: [
            float(0, 4, [
                  int(0, 1),
                  expo(1, 4, [
                       int(2, 4)
                  ])
            ])
        ]
    };
}

#[test]
fn zero_point_exp() {
    parses_to! {
        parser: SilverParser,
        input: "0.e0",
        rule: Rule::float,
        tokens: [
            float(0, 4, [
                  int(0, 1),
                  expo(2, 4, [
                       int(3, 4)
                  ])
            ])
        ]
    };
}

#[test]
fn zero_point_zero_exp_plus() {
    parses_to! {
        parser: SilverParser,
        input: "0.0e+0",
        rule: Rule::float,
        tokens: [
            float(0, 6, [
                  int(0, 1),
                  int(2, 3),
                  expo(3, 6, [
                       plus(4, 5),
                       int(5, 6)
                  ])
            ])
        ]
    };
}
