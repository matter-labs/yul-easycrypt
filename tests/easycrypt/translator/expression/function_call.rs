#![cfg(test)]

use crate::easycrypt::test_translation;

#[test]
fn fun() {
    test_translation(
        r#"
object "Test" {
  code {}
  object "Test_deployed" {
    code {
      function test_fun(p) -> ret {
        ret := add(1, p)
       }
    }
  }
}
"#,
        r#"
(* Begin _Test *)
module _Test = {
  proc _BODY(): unit = { }

  proc test_fun(p : uint256): uint256 = {
    var ret :uint256;
    ret <- ((W256.of_int 1) + p);
    return ret;
  }
}.
(* End _Test *)
"#,
    )
}
