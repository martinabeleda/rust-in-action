#![allow(dead_code, unused_variables)]

struct Cpu {
    // Having 16 registers means that a single hexadecimal number can address those
    registers: [u8; 16],

    // Also known as the position in memory
    program_counter: usize,

    // 4096 bytes of RAM
    memory: [u8; 0x1000],

    // Maximum height of the stack is 16. After
    // 16 nested function calls, we encounter a
    // stack overflow
    stack: [u16; 16],

    // Used to index values within the stack
    stack_pointer: usize,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            registers: [0; 16],
            memory: [0; 4096],
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;
            let kk = opcode & 0x00FF;

            match (c, x, y, d) {
                // STOP condition
                (0, 0, 0, 0) => return,

                // ADD opcode
                (0x8, _, _, 0x4) => self.add_xy(x, y),

                // CALL opcode sets `program_counter` to `nnn`, the address of
                // the function
                (0x2, _, _, _) => self.call(nnn),

                // RETURN opcode sets `program_counter` to the memory address of
                // the previous opcode
                (0x0, 0x0, 0xE, 0xE) => self.ret(),

                // Catch not implemented
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("stack overflow");
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack underflow")
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.program_counter = call_addr as usize;
    }
}

fn main() {
    // Compute 5 + 10 + 10 + 10
    let mut cpu = Cpu::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14; // Add register 0 to register 1
    mem[2] = 0x80;
    mem[3] = 0x24; // Add register 0 to register 2
    mem[4] = 0x80;
    mem[5] = 0x34; // Add register 0 to register 3

    cpu.run();

    // Result of each addition is stored in register 0
    assert_eq!(cpu.registers[0], 35);

    // Compute 5 + (10 * 2) + (10 * 2)
    let mut cpu = Cpu::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // Call the function at 0x100 twice
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    // Create a function at 0x100 that adds register 0 to 1 twice
    // and then returns
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    // Result of each addition is stored in register 0
    assert_eq!(cpu.registers[0], 45);
}
