extern crate argentum;

use std::fs::File;
use std::io::prelude::*;

use argentum::syntax::parser;

macro_rules! integration_test {
  ($id:ident, $name:expr, $is_ok:expr) => {
    #[test]
    fn $id() {
      let mut fixture = File::open(format!("tests/fixtures/{}.ag", $name))
        .expect(&format!("failed to open tests/fixtures/{}.ag", $name));
      let mut buf = String::new();
      fixture.read_to_string(&mut buf)
        .expect(&format!("failed to read tests/fixtures/{}.ag", $name));
      let outcome = parser::parse(&buf);
      assert_eq!(
        outcome.is_ok(),
        $is_ok,
        "failed to parse tests/fixtures/{}.ag: {:?}",
        $name,
        outcome
        );
    }
  }
}

integration_test!(test_comment, "comment", true);
