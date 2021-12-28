use std::rc::Rc;

use byteorder::{BigEndian, ByteOrder};

#[derive(Debug)]
pub struct CodeReader {
    code: Rc<Vec<u8>>,
    pub pc: usize,
}

impl CodeReader {
    pub fn new(code: Rc<Vec<u8>>, pc: usize) -> CodeReader {
        CodeReader { code, pc }
    }

    pub fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc
    }

    pub fn read_u8(&mut self) -> u8 {
        let u8_value = self.code[self.pc];
        self.pc += 1;
        u8_value
    }

    pub fn read_i8(&mut self) -> i8 {
        let u8_value = self.code[self.pc];
        let i8_value = unsafe { std::mem::transmute::<u8, i8>(u8_value) };
        self.pc += 1;
        i8_value
    }

    pub fn read_u16(&mut self) -> u16 {
        let u8_array_slice = &self.code[self.pc..(self.pc + 2)];
        self.pc += 2;
        BigEndian::read_u16(u8_array_slice)
    }

    pub fn read_i16(&mut self) -> i16 {
        let u8_array_slice = &self.code[self.pc..(self.pc + 2)];
        self.pc += 2;
        BigEndian::read_i16(u8_array_slice)
    }
}