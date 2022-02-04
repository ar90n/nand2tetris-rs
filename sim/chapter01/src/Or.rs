#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_Or(){
let mut m = Or::new();
m.a = false;
m.b = false;
m.prop();
assert_eq!(m.b, false);
assert_eq!(m.out, false);
assert_eq!(m.a, false);
m.a = false;
m.b = true;
m.prop();
assert_eq!(m.b, true);
assert_eq!(m.out, true);
assert_eq!(m.a, false);
m.a = true;
m.b = false;
m.prop();
assert_eq!(m.b, false);
assert_eq!(m.out, true);
assert_eq!(m.a, true);
m.a = true;
m.b = true;
m.prop();
assert_eq!(m.b, true);
assert_eq!(m.out, true);
assert_eq!(m.a, true);
}
}
