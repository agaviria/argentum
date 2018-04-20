use ast::nodes::node_type;
use parser::{SilverParser, Rule};
use pest::inputs::StrInput;
use pest::iterators::Pair;

pub fn consume<I: StrInput>(pair: Pair<Rule, I>) -> node_type::KindOf {
    match pair.as_rule() {
        Rule::float_literal => build_float(pair),
        _                   => unexpected_token(pair),
    }
}

pub fn build_map<I: StrInput>(pair: Pair<Rule, I>) -> node_type::KindOf {
    unimplemented!();
}

fn build_float<I: StrInput>(pair: Pair<Rule, I>) -> node_type::KindOf {
    let value = pair.into_span().as_str().replace("_", "");
    float64!(&value)
}

fn unexpected_token<I: StrInput>(pair: Pair<Rule, I>) -> ! {
    let msg = format!("Unexpected token: {:#}", pair);
    panic!(msg);
}

pub fn parse_str_wrapper(src: &str) -> node_type::KindOf {
    let parser = SilverParser::parse_str(Rule::input, src);
    if parser.is_err() { panic!(format!("{:#}", parser.err().unwrap())); }

    let pair = parser.unwrap().next().unwrap();
    consume(pair)
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse(input: &str) -> node_type::KindOf {
        println!("Attempting to parse {:?}", input);
        parse_str_wrapper(input)
    }

    #[test]
    fn float_literal() {
        assert_eq!(parse("0.0"), float32!("0.0"));
        assert_eq!(parse("3.2250738585072014"), float64!("2.2250738585072014"));
    }
}
