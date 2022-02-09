use super::module::Module;

pub(crate) struct Ram {
    address: u32,
    in_: u32,
    load: bool,
    out: u32,
    buffer: Vec<u32>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        let buffer = vec![0; size];

        Self {
            address: Default::default(),
            in_: Default::default(),
            load: Default::default(),
            out: Default::default(),
            buffer,
        }
    }
}

impl Module for Ram {
    fn address(&mut self, value: u32) {
        self.address = value;
    }

    fn in_(&mut self, value: u32) {
        self.in_ = value;
    }

    fn load(&mut self, value: bool) {
        self.load = value;
    }

    fn out(&self) -> u32 {
        self.out
    }

    fn posedge_clk(&mut self) {
        if self.load {
            self.write(self.address, self.in_);
        }
    }

    fn prop(&mut self) {
        self.out = self.read(self.address);
    }

    fn read(&mut self, address: u32) -> u32 {
        self.buffer[address as usize]
    }

    fn write(&mut self, address: u32, value: u32) {
        self.buffer[address as usize] = value;
    }
}
