// Tests #[warn(unused_variables)] overrides and triggers a warning

// compile-pass

#[allow(warnings)]
#[warn(unused_variables)]
fn main() {
    let x = 3;
}
