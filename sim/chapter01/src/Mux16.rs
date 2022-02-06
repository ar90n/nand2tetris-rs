#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_Mux16(){
let mut m = Mux16::new();
m.a = 0;
m.b = 0;
m.sel = false;
m.prop();
assert_eq!(m.out, 0);
assert_eq!(m.sel, false);
assert_eq!(m.a, 0);
assert_eq!(m.b, 0);
m.sel = true;
m.prop();
assert_eq!(m.out, 0);
assert_eq!(m.sel, true);
assert_eq!(m.a, 0);
assert_eq!(m.b, 0);
m.a = 0;
m.b = 4660;
m.sel = false;
m.prop();
assert_eq!(m.out, 0);
assert_eq!(m.sel, false);
assert_eq!(m.a, 0);
assert_eq!(m.b, 4660);
m.sel = true;
m.prop();
assert_eq!(m.out, 4660);
assert_eq!(m.sel, true);
assert_eq!(m.a, 0);
assert_eq!(m.b, 4660);
m.a = 39030;
m.b = 0;
m.sel = false;
m.prop();
assert_eq!(m.out, 39030);
assert_eq!(m.sel, false);
assert_eq!(m.a, 39030);
assert_eq!(m.b, 0);
m.sel = true;
m.prop();
assert_eq!(m.out, 0);
assert_eq!(m.sel, true);
assert_eq!(m.a, 39030);
assert_eq!(m.b, 0);
m.a = 43690;
m.b = 21845;
m.sel = false;
m.prop();
assert_eq!(m.out, 43690);
assert_eq!(m.sel, false);
assert_eq!(m.a, 43690);
assert_eq!(m.b, 21845);
m.sel = true;
m.prop();
assert_eq!(m.out, 21845);
assert_eq!(m.sel, true);
assert_eq!(m.a, 43690);
assert_eq!(m.b, 21845);
}
}
