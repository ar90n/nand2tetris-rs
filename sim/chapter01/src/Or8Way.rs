#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_Or8Way(){
let mut m = Or8Way::new();
m.in_ = 0;
m.prop();
assert_eq!(m.out, false);
assert_eq!(m.in_, 0);
m.in_ = 255;
m.prop();
assert_eq!(m.out, true);
assert_eq!(m.in_, 255);
m.in_ = 16;
m.prop();
assert_eq!(m.out, true);
assert_eq!(m.in_, 16);
m.in_ = 1;
m.prop();
assert_eq!(m.out, true);
assert_eq!(m.in_, 1);
m.in_ = 38;
m.prop();
assert_eq!(m.out, true);
assert_eq!(m.in_, 38);
}
}
