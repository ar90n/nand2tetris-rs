#[cfg(test)]
mod tests {
use super::super::modules::*;
#[test]
fn test_Bit(){
let mut m = Bit::new();
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = false;
m.load = true;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, true);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, true);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = true;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, true);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, true);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = true;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, true);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, true);
assert_eq!(m.out, false);
m.in_ = true;
m.load = true;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, true);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, true);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = false;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, false);
assert_eq!(m.out, true);
m.in_ = false;
m.load = true;
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, true);
assert_eq!(m.out, true);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, false);
assert_eq!(m.load, true);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.in_ = true;
m.load = false;
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
m.posedge_clk();
m.prop();
assert_eq!(m.in_, true);
assert_eq!(m.load, false);
assert_eq!(m.out, false);
}
}
