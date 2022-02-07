use super::module::Module;
use super::ram::Ram;

pub(crate) struct Keyboard{
    out: u32,
    code: u32,
}

impl Keyboard {
    pub fn new() -> Self {
        Self{
            out: Default::default(),
            code: Default::default(),
        }
    }
}

impl Module for Keyboard {
    fn address(&mut self, _: u32) {}

    fn in_(&mut self, _: u32) {}

    fn load(&mut self, _: bool) {}

    fn out(&self) -> u32 {
        self.out
    }

    fn posedge_clk(&mut self) {
        // TODO: scan keyboard here
        self.code = Default::default();
    }

    fn prop(&mut self) {
        self.out = self.code;
    }

    fn read(&mut self, _: u32) -> u32 {
        self.code
    }

    fn write(&mut self, _: u32, value: u32) {
        self.code = value;
    }
}
