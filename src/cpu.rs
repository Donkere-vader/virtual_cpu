use crate::types::{ Ubit, OppCode };
use crate::utils::{ parse_instruction };

pub struct Cpu {
    pub counter: u16,
    pub registries: [u16; Ubit::MAX as usize],
    pub active_res: Ubit,
    pub mem: [u16; u16::MAX as usize]
}

impl Cpu {
    pub fn new(mem: [u16; u16::MAX as usize]) -> Cpu {
        Cpu {
            counter: 0,
            registries: [0; Ubit::MAX as usize],
            active_res: 0,
            mem,
        }
    }

    pub fn tick(&mut self) {
        let instruction = parse_instruction(self.mem[self.counter as usize]);

        match instruction {
            OppCode::Setr(value) => self.o_setr(value),
            OppCode::Load(value) => self.o_load(value),
            OppCode::Store(value) => self.o_store(value),
            OppCode::Add(value) => self.o_add(value),
            OppCode::Sub(value) => self.o_sub(value),
            OppCode::Mul(value) => self.o_mul(value),
            OppCode::Div(value) => self.o_div(value),
            OppCode::Jump(value) => self.o_jump(value)
        }

        self.counter += 1;
    }
}

impl Cpu {
    fn o_setr(&mut self, value: Ubit) {
        self.active_res = value;
    }

    fn o_load(&mut self, value: Ubit) {
        self.registries[self.active_res as usize] = self.mem[value as usize];
    }

    fn o_store(&mut self, value: Ubit) {
        self.mem[value as usize] = self.registries[self.active_res as usize];
    }

    fn o_add(&mut self, value: Ubit) {
        self.registries[self.active_res as usize] += self.mem[value as usize];
    }

    fn o_sub(&mut self, value: Ubit) {
        self.registries[self.active_res as usize] -= self.mem[value as usize];
    }

    fn o_mul(&mut self, value: Ubit) {
        self.registries[self.active_res as usize] *= self.mem[value as usize];
    }

    fn o_div(&mut self, value: Ubit) {
        self.registries[self.active_res as usize] /= self.mem[value as usize];
    }

    fn o_jump(&mut self, value: Ubit) {
        self.counter = self.mem[value as usize];
    }
}