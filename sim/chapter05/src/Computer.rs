use behavior;
use super::modules;

pub struct Computer {
    pub reset: bool,

    cpu: modules::CPU,
    memory: behavior::memory::Memory,
    rom: behavior::rom::Rom,
}

impl Computer{
    pub fn new(program: Vec<u16>) -> Self {
        let mut cpu = modules::CPU::new();
        let mut memory = behavior::memory::Memory::new();
        let mut rom = behavior::rom::Rom::new(program);

        Self {
            reset: false,
            cpu,
            memory,
            rom,
        }
    }

    pub fn posedge_clk(&mut self) {
        self.cpu.posedge_clk();
        self.memory.posedge_clk();
        self.rom.posedge_clk();
    }
    
    pub fn prop(&mut self) {
        self.cpu.reset = self.reset;
        self.cpu.prop();

        self.cpu.instruction = self.rom.read(self.cpu.pc);
        self.cpu.inM = self.memory.read(self.cpu.addressM);
        self.cpu.prop();

        self.memory.address = self.cpu.addressM;
        self.memory.in_ = self.cpu.outM;
        self.memory.load = self.cpu.writeM;
        self.memory.prop();

        self.rom.address = self.cpu.pc;
        self.rom.prop();
    }

    pub fn read_memory(&mut self, address: u32) -> u32 {
        self.memory.read(address)
    }

    pub fn read_rom(&mut self, address: u32) -> u32 {
        self.rom.read(address)
    }

    pub fn read_a_reg(&mut self) -> u32 {
        self.cpu.ARegister
    }

    pub fn read_d_reg(&mut self) -> u32 {
        self.cpu.DRegister
    }

    pub fn write_memory(&mut self, address: u32, value: u32) {
        self.memory.write(address, value)
    }
}