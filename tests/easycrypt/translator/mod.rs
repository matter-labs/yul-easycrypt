#[cfg(test)]
use crate::easycrypt::test_translation;

pub mod expression;
pub mod statement;

#[test]
fn definition() {
    test_translation(
        r#"
object "Test" {
    code { }
    object "Test_deployed" {
        code {
        let x
    }
    }
}
"#,
        r#"
(* Begin _Test *)
module _Test = {
  proc _BODY(): unit = {
    var x: uint256;
    }


  }.
(* End _Test *)
"#,
    )
}

#[test]
fn definition_with_init() {
    test_translation(
        r#"
object "Test" {
    code { }
    object "Test_deployed" {
        code {
        let x
        let a := 4
    }
    }
}
"#,
        r#"
(* Begin _Test *)
module _Test = {
  proc _BODY(): unit = {
    var x, a: uint256;
    a <- (W256.of_int 4);
    }


  }.
(* End _Test *)
"#,
    )
}
#[test]
fn function() {
    test_translation(
        r#"
object "Test" {
    code { }
    object "Test_deployed" {
        code {
        function test_fun(p) -> ret {
        ret := p
    }

    }
    }
}
"#,
        r#"
(* Begin _Test *)
module _Test = {
  proc _BODY(): unit = {
    }

  proc test_fun(p : uint256): uint256 = {
    var ret :uint256;
    ret <- p;
    return ret;
    }


  }.
(* End _Test *)
"#,
    )
}
#[test]
fn bad_names() {
    test_translation(
        r#"
object "Test" {
    code { }
    object "Test_deployed" {
        code {
let x$w := 20

function Test_Fun(p) -> ret {
ret := p
}
function end() -> x {
x := 1
}
}
    }
}
"#,
        r#"(* Begin _Test *)
module _Test = {
   proc _BODY(): unit = {
    var x_w :uint256;
    x_w <- (W256.of_int 20);
    }
   proc _Test_Fun(p : uint256): uint256 = {
    var ret :uint256;
    ret <- p;
    return ret;
    }
   proc _end(): uint256 = {
    var x :uint256;
    x <- (W256.of_int 1);
    return x;
    }
  }.
(* End _Test *)
 "#,
    );
}
