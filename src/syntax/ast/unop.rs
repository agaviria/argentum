use std::fmt;

#[derive(Copy, Debug, PartialEq, Clone)]
pub enum UnaryOp  {
  /// (!): Lazy boolean NOT operator.  Used to reverse the logical state of its
  /// operand.  If condition is true, then logical NOT operator will make false.
  /// Has right to left associativity.
  ///
  /// i.e.  !(A && B); is true
  Not,
  /// (-): Used with integer operand types. Unary minus holds higher precedence
  /// than binary minus and has right to left associativity.
  Minus,
}

impl fmt::Display for UnaryOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let unary_op = match *self {
      UnaryOp::Not    => "!",
      UnaryOp::Minus  => "-",
    };
    write!(f, "{}", unary_op)
  }
}

// TODO:
// Type cast expressions - Cast operator to build cast expr for conversion to a given type.
// e.g.   type_cast_expr : value "as" type ;
