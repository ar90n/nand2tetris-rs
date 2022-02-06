#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_Add16(){
let mut m = Add16::new();
m.a = 0;
m.b = 0;
m.prop();
assert_eq!(m.out, 0);
assert_eq!(m.a, 0);
assert_eq!(m.b, 0);
m.a = 0;
m.b = 65535;
m.prop();
assert_eq!(m.out, 65535);
assert_eq!(m.a, 0);
assert_eq!(m.b, 65535);
m.a = 65535;
m.b = 65535;
m.prop();
assert_eq!(m.out, 65534);
assert_eq!(m.a, 65535);
assert_eq!(m.b, 65535);
m.a = 43690;
m.b = 21845;
m.prop();
assert_eq!(m.out, 65535);
assert_eq!(m.a, 43690);
assert_eq!(m.b, 21845);
m.a = 15555;
m.b = 4080;
m.prop();
assert_eq!(m.out, 19635);
assert_eq!(m.a, 15555);
assert_eq!(m.b, 4080);
m.a = 4660;
m.b = 39030;
m.prop();
assert_eq!(m.out, 43690);
assert_eq!(m.a, 4660);
assert_eq!(m.b, 39030);
}
}
