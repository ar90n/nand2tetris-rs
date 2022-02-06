#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_Inc16(){
let mut m = Inc16::new();
m.in_ = 0;
m.prop();
assert_eq!(m.in_, 0);
assert_eq!(m.out, 1);
m.in_ = 65535;
m.prop();
assert_eq!(m.in_, 65535);
assert_eq!(m.out, 0);
m.in_ = 5;
m.prop();
assert_eq!(m.in_, 5);
assert_eq!(m.out, 6);
m.in_ = 65531;
m.prop();
assert_eq!(m.in_, 65531);
assert_eq!(m.out, 65532);
}
}
