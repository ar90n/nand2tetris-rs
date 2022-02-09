pub struct Rom {
    pub address: u32,
    pub out: u32,

    buf: Vec<u16>,
}

impl Rom {
    pub fn new(buf: Vec<u16>) -> Self {
        Self {
            address: Default::default(),
            out: Default::default(),
            buf,
        }
    }

    pub fn posedge_clk(&mut self) {}

    pub fn prop(&mut self) {
        self.out = self.read(self.address) as u32;
    }

    pub fn read(&mut self, address: u32) -> u32 {
        if self.buf.len() <= address as usize {
            return 0;
        }
        self.buf[address as usize] as u32
    }

    pub fn write(&mut self, address: u32, value: u32) {
        unimplemented!()
    }
}