pub mod binop;
pub mod unop;

use std::fmt;

use self::binop::{ArithmeticOp, BitWiseOp, ComparisonOp, LogicalOp};
use self::unop::UnaryOp;

//use pest::prec_climber::{Assoc, Operator, PrecClimber};

/// The core AST node enumerator value.
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum NodeExprRule {
  /// Identifier is a non-empty string of ASCII letters and underscores.
  Identifier(String),
  /// Boolean is true or false.
  Boolean(bool),
  /// String literal.
  String(String),
  /// Integer literal.
  Int(i64),
  /// Float literal.
  Float(f64),
  /// ::  ->  [  ]  (  )  {  }  ,  ;
  Symbol(SymbolOp),
  /// Comparison operators:  >  <  >=  <=  !=  ==
  Comparative(ComparisonOp),
  /// Mathematical operators:  +  -  ++  --  *  /  %
  Arithmetic(ArithmeticOp),
  /// BitWise operators:  &  |  ^  ~  <<  >>
  BitWise(BitWiseOp),
  /// Lazy boolean operators:  &&  ||
  Logical(LogicalOp),
  /// Unary operators:  !  -
  Unary(UnaryOp),
  /// Assignment operator:  =
  Assign(AssignmentOp),
}

/// Assigns values from the right side operand to the left side operand.
#[derive(Copy, Debug, PartialEq, Clone)]
pub enum AssignmentOp  {
  /// (=): Simple assignment operator.
  ///
  /// i.e.  C = A + B;  will assign value of A + B into C
  Assign,
}

impl fmt::Display for AssignmentOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let assignment_op = match *self {
      AssignmentOp::Assign => "=",
    };
    write!(f, "{}", assignment_op)
  }
}

/// Symbols are general class printable tokens that play structural roles in a
/// variety of grammar productions.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SymbolOp {
  /// (::): Path symbol.
  Path,
  /// (->): Cast symbol.
  Cast,
  /// ([): left square symbol.
  LSquare,
  /// (]): right square symbol.
  RSquare,
  /// (,): comma symbol.
  Comma,
  /// ({): left brace symbol.
  LBrace,
  /// (}): right brace symbol.
  RBrace,
  /// (;): semi colon symbol.
  SemiColon,
}

impl fmt::Display for SymbolOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let symbol = match *self {
      SymbolOp::Path      => "::",
      SymbolOp::Cast      => "->",
      SymbolOp::LBrace    => "{",
      SymbolOp::RBrace    => "}",
      SymbolOp::LSquare   => "[",
      SymbolOp::RSquare   => "]",
      SymbolOp::Comma     => ",",
      SymbolOp::SemiColon => ";",
    };
    write!(f, "{}", symbol)
  }
}
