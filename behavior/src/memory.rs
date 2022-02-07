use super::keyboard::Keyboard;
use super::ram::Ram;
use super::screen::Screen;

use super::module::Module;

pub(crate) struct Map {
    pub origin: u32,
    pub size: u32,
    pub module: Box<dyn Module>,
}

pub struct Memory {
    pub address: u32,
    pub in_: u32,
    pub load: bool,
    pub out: u32,

    modules: Vec<Map>,
}

impl Memory {
    pub fn new() -> Self {
        let modules = vec![
            Map {
                origin: 0x0000,
                size: 0x4000,
                module: Box::new(Ram::new(0x4000)),
            },
            Map {
                origin: 0x4000,
                size: 0x2000,
                module: Box::new(Screen::new(512, 256)),
            },
            Map {
                origin: 0x6000,
                size: 0x0001,
                module: Box::new(Keyboard::new()),
            },
        ];

        Self {
            address: Default::default(),
            in_: Default::default(),
            load: Default::default(),
            out: Default::default(),
            modules,
        }
    }

    pub fn posedge_clk(&mut self) {
        for map in self.modules.iter_mut() {
            map.module.posedge_clk();
        }
    }

    pub fn prop(&mut self) {
        for map in self.modules.iter_mut() {
            let (address, load) =
                if map.origin <= self.address && self.address < map.origin + map.size {
                    let offset = self.address - map.origin;
                    (offset, self.load)
                } else {
                    (0, false)
                };
            map.module.in_(self.in_);
            map.module.load(load);
            map.module.address(address);

            map.module.prop();

            if map.origin <= self.address && self.address < map.origin + map.size {
                self.out = map.module.out();
            }
        }
    }

    pub fn read(&mut self, address: u32) -> u32 {
        for map in self.modules.iter_mut() {
            if map.origin <= address && address < map.origin + map.size {
                let offset = address - map.origin;
                return map.module.read(offset);
            }
        }
        0
    }

    pub fn write(&mut self, address: u32, value: u32) {
        for map in self.modules.iter_mut() {
            if map.origin <= address && address < map.origin + map.size {
                let offset = address - map.origin;
                map.module.write(offset, value);
                return;
            }
        }
    }
}
