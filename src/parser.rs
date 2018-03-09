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

//expressions - here be dragons, lmao
//parser(uop);        parser(asop);   //unary operators;          assignment operators
//parser(pexp);       parser(pfexp);  //primary expression;       prefix expression
//parser(params);     parser(cast);   //function call parameters; cast expression
//parser(uexp);       parser(mexp);   //unary expression;         multiplicative expression
//parser(aexp);       parser(sexp);   //addition expression;      shift expression
//parser(rexp);       parser(eexp);   //relation expression;      equivalence expression
//parser(bexp);       parser(lexp);   //bit-wise expression;      boolean expression
//parser(elexp);      parser(asexp);  //conditional expression;   assignment expression
//parser(cexp); parser(exp); //constant expression;      expression (cheers!)
