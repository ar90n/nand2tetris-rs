#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_DMux(){
let mut m = DMux::new();
m.in_ = false;
m.sel = false;
m.prop();
assert_eq!(m.a, false);
assert_eq!(m.in_, false);
assert_eq!(m.sel, false);
assert_eq!(m.b, false);
m.sel = true;
m.prop();
assert_eq!(m.a, false);
assert_eq!(m.in_, false);
assert_eq!(m.sel, true);
assert_eq!(m.b, false);
m.in_ = true;
m.sel = false;
m.prop();
assert_eq!(m.a, true);
assert_eq!(m.in_, true);
assert_eq!(m.sel, false);
assert_eq!(m.b, false);
m.sel = true;
m.prop();
assert_eq!(m.a, false);
assert_eq!(m.in_, true);
assert_eq!(m.sel, true);
assert_eq!(m.b, true);
}
}
