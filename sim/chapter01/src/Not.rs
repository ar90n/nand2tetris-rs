#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_Not(){
let mut m = Not::new();
m.in_ = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.out, true);
m.in_ = true;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.out, false);
}
}
