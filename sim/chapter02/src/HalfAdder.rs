#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_HalfAdder(){
let mut m = HalfAdder::new();
m.a = false;
m.b = false;
m.prop();
assert_eq!(m.sum, false);
assert_eq!(m.carry, false);
assert_eq!(m.a, false);
assert_eq!(m.b, false);
m.a = false;
m.b = true;
m.prop();
assert_eq!(m.sum, true);
assert_eq!(m.carry, false);
assert_eq!(m.a, false);
assert_eq!(m.b, true);
m.a = true;
m.b = false;
m.prop();
assert_eq!(m.sum, true);
assert_eq!(m.carry, false);
assert_eq!(m.a, true);
assert_eq!(m.b, false);
m.a = true;
m.b = true;
m.prop();
assert_eq!(m.sum, false);
assert_eq!(m.carry, true);
assert_eq!(m.a, true);
assert_eq!(m.b, true);
}
}
