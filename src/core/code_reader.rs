pub struct CodeReader {
    code: Vec<u8>,
    pub pc: usize,
}

impl CodeReader {
    pub fn new(code: Vec<u8>) -> CodeReader {
        CodeReader { code, pc: 0 }
    }

    pub fn read_u8(mut self) -> u8 {
        let u8_value = self.code[self.pc];
        self.pc += 1;
        u8_value
    }

    pub fn read_i8(mut self)->i8{
        let u8_value = self.code[self.pc];
        let i8_value = unsafe {std::mem::transmute::<u8,i8>(u8_value)};
        self.pc += 1;
        i8_value
    }

    pub fn read_
}