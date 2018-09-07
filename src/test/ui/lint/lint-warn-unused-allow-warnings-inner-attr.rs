// Tests #[allow(warnings)] overrides and suppresses the warning

// compile-pass

#![warn(unused_variables)]
#![allow(warnings)]

fn main() {
    let x = 3;
}
