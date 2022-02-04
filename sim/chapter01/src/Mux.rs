#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_Mux(){
let mut m = Mux::new();
m.a = false;
m.b = false;
m.sel = false;
m.prop();
assert_eq!(m.a, false);
assert_eq!(m.b, false);
assert_eq!(m.sel, false);
assert_eq!(m.out, false);
m.sel = true;
m.prop();
assert_eq!(m.a, false);
assert_eq!(m.b, false);
assert_eq!(m.sel, true);
assert_eq!(m.out, false);
m.a = false;
m.b = true;
m.sel = false;
m.prop();
assert_eq!(m.a, false);
assert_eq!(m.b, true);
assert_eq!(m.sel, false);
assert_eq!(m.out, false);
m.sel = true;
m.prop();
assert_eq!(m.a, false);
assert_eq!(m.b, true);
assert_eq!(m.sel, true);
assert_eq!(m.out, true);
m.a = true;
m.b = false;
m.sel = false;
m.prop();
assert_eq!(m.a, true);
assert_eq!(m.b, false);
assert_eq!(m.sel, false);
assert_eq!(m.out, true);
m.sel = true;
m.prop();
assert_eq!(m.a, true);
assert_eq!(m.b, false);
assert_eq!(m.sel, true);
assert_eq!(m.out, false);
m.a = true;
m.b = true;
m.sel = false;
m.prop();
assert_eq!(m.a, true);
assert_eq!(m.b, true);
assert_eq!(m.sel, false);
assert_eq!(m.out, true);
m.sel = true;
m.prop();
assert_eq!(m.a, true);
assert_eq!(m.b, true);
assert_eq!(m.sel, true);
assert_eq!(m.out, true);
}
}
