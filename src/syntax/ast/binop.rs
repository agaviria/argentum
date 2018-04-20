use std::fmt;

/// Arithmetic operators are a variant of binary operator expressions.
/// Can only be used with operands of number type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticOp {
  /// (+): Addition operator.
  Add,
  /// (++): Increment operator.
  Incr,
  /// (-): Subtraction operator.  Sub operator has left-right associativity.
  Sub,
  /// (--): Decrement operator.
  Decr,
  /// (*): Multiplication operator.
  Mul,
  /// (/): Division operator.
  Div,
  /// (%): Modulo operator.
  Modulo
}

impl fmt::Display for ArithmeticOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let math_op = match *self {
      ArithmeticOp::Add    => "+",
      ArithmeticOp::Incr   => "++",
      ArithmeticOp::Sub    => "-",
      ArithmeticOp::Decr   => "--",
      ArithmeticOp::Mul    => "*",
      ArithmeticOp::Div    => "/",
      ArithmeticOp::Modulo => "%",
    };
    write!(f, "{}", math_op)
  }
}

/// BitWise operators are a variant of binary expressions.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BitWiseOp {
  /// (&): BitWise AND operator copies a bit to the result if it exists in both
  /// operands.
  ///
  /// i.e.  (A & B) will give 12 which is 0000 1100
  BwAnd,
  /// (|): BitWise OR operator copies a bit if it exists in either operand.
  ///
  /// i.e.  (A | B) will give 61 which is 0011 0001
  BwOr,
  /// (^): BitWise exclusive XOR operator copies the bit if it is set in one
  /// operand but not both.
  ///
  /// i.e.  (A ^ B) will give 49 which is 011 0001
  BwXor,
  /// (~): BitWise complement operator is unary and has the effect of 'flipping'
  /// bits. Has right to left associativity.
  ///
  /// i.e.  (~A) will give -61 which is 1100 0011 in 2's complement form due to
  ///            a signed binary number.
  BwCompl,
  /// (<<): BitWise shift left operator moves the left operand value by the
  /// number of bits specified by the right operand.
  ///
  /// i.e.  A << 2 will give 240 which is 1111 0000
  BwShftL,
  /// (>>): BitWise shift right operator moves the right operand value by the
  /// number of bits specified by the left operand.
  ///
  /// i.e.  A >> 2 will give 15 which is 0000 1111
  BwShftR,
}

impl fmt::Display for BitWiseOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let bw_op = match *self {
      BitWiseOp::BwAnd   => "&",
      BitWiseOp::BwOr    => "|",
      BitWiseOp::BwXor   => "^",
      BitWiseOp::BwCompl => "~",
      BitWiseOp::BwShftL => "<<",
      BitWiseOp::BwShftR => ">>",
    };
    write!(f, "{}", bw_op)
  }
}

/// Comparison operators can be used with integer type values and are a variant
/// of binary operator expressions.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ComparisonOp {
  /// (>): Greater than
  Gt,
  /// (>=): Greater than or Equal to
  GtEql,
  /// (<): Less than
  Lt,
  /// (<=): Less than or Equal to
  LtEql,
  /// (==): Equal to
  Eql,
  /// (!=): Not Equal checks if the value of two operands are equal or not,
  /// if values are not equal then condition becomes true.
  NotEql,
}

impl fmt::Display for ComparisonOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let cmp_op = match *self {
      ComparisonOp::Gt    => ">",
      ComparisonOp::GtEql  => ">=",
      ComparisonOp::Lt    => "<",
      ComparisonOp::LtEql  => "<=",
      ComparisonOp::Eql   => "==",
      ComparisonOp::NotEql => "!=",
    };
    write!(f, "{}", cmp_op)
  }
}

/// Logical operators can only be used with operands of boolean type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicalOp {
  /// (&&): Lazy boolean AND operator.  If both the operands are non-zero, then
  /// condition becomes true.
  ///
  /// i.e.  (A && B) is false
  And,
  /// (||): Lazy boolean OR operator.  If any of the two operands is non-zero,
  /// then condition becomes true.
  ///
  /// i.e.  (A || B) is true
  Or,
}

impl fmt::Display for LogicalOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let lazy_bool_op = match *self {
      LogicalOp::And  => "&&",
      LogicalOp::Or   => "||",
    };
    write!(f, "{}", lazy_bool_op)
  }
}
