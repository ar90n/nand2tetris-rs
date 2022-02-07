pub(crate) trait Module {
    fn address(&mut self, value:u32);
    fn in_(&mut self, value:u32);
    fn load(&mut self, value:bool);
    fn out(& self) -> u32;

    fn posedge_clk(&mut self);
    fn prop(&mut self);

    fn read(&mut self, address: u32) -> u32;
    fn write(&mut self, address: u32, value: u32);
}