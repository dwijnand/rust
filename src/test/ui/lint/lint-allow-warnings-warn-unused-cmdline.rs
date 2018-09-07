// Tests -W unused-variables overrides and triggers the warning

// compile-pass
// compile-flags: -A warnings -W unused-variables

fn main() {
    let x = 3;
}
