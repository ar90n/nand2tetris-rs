#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_Not16(){
let mut m = Not16::new();
m.in_ = 0;
m.prop();
assert_eq!(m.out, 65535);
assert_eq!(m.in_, 0);
m.in_ = 65535;
m.prop();
assert_eq!(m.out, 0);
assert_eq!(m.in_, 65535);
m.in_ = 43690;
m.prop();
assert_eq!(m.out, 21845);
assert_eq!(m.in_, 43690);
m.in_ = 15555;
m.prop();
assert_eq!(m.out, 49980);
assert_eq!(m.in_, 15555);
m.in_ = 4660;
m.prop();
assert_eq!(m.out, 60875);
assert_eq!(m.in_, 4660);
}
}
