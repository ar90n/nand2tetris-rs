use super::module::Module;
use super::ram::Ram;

pub(crate) struct Screen {
    ram: Ram,
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let size = (width * height + 15) / 16;
        let ram = Ram::new(size);

        Self { ram, width, height }
    }
}

impl Module for Screen {
    fn address(&mut self, value: u32) {
        self.ram.address(value);
    }

    fn in_(&mut self, value: u32) {
        self.ram.in_(value);
    }

    fn load(&mut self, value: bool) {
        self.ram.load(value);
    }

    fn out(&self) -> u32 {
        self.ram.out()
    }

    fn posedge_clk(&mut self) {
        self.ram.posedge_clk();

        // TODO: draw screen here
    }

    fn prop(&mut self) {
        self.ram.prop();
    }

    fn read(&mut self, address: u32) -> u32 {
        self.ram.read(address)
    }

    fn write(&mut self, address: u32, value: u32) {
        self.ram.write(address, value);
    }
}
