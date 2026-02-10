use crate::{Bus, decoder::decode, instruction::Instruction, memory::ROM_START};

pub struct Cpu {
    v_registers: [u8; 16],
    pc: u16,
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v_registers: [0; 16],
            pc: ROM_START,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn clear_screen(&mut self) {}

    fn draw_sprite(&mut self, x: u8, y: u8, n: u8) {}

    fn fetch(&mut self, bus: &Bus) -> u16 {
        let byte1: u16 = (bus.memory[self.pc as usize] as u16) << 8;
        let byte2: u16 = bus.memory[self.pc as usize + 1] as u16;

        // BUG what if it goes past RAM_SIZE?
        // TODO: Move this out of here to somewhere else, maybe main loop or maybe not? Idk bro.
        self.pc += 2;

        byte1 | byte2
    }

    fn execute(&mut self, opcode: u16) -> bool {
        let mut should_redraw = false;
        let instruction = decode(opcode).expect("Invalid Opcode");

        match instruction {
            Instruction::Cls => {
                self.clear_screen();
                should_redraw = true;
            }
            Instruction::Jump(nnn) => self.pc = nnn,
            Instruction::SetRegVX(x, kk) => self.v_registers[x as usize] = kk,
            Instruction::AddValueToVX(x, kk) => self.v_registers[x as usize] += kk,
            Instruction::SetIndex(nnn) => self.i = nnn,
            Instruction::Draw(x, y, n) => {
                self.draw_sprite(x, y, n);
                should_redraw = true;
            }
        }

        should_redraw
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_opcode_logic() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();

        let dummy_rom = [0x12, 0x34];

        bus.load_rom(&dummy_rom).unwrap();

        let opcode = cpu.fetch(&bus);
        // bytes should should be successfully fetched and combined into a u16 opcode (Big Endian)
        assert_eq!(opcode, 0x1234);
        // pc should move forward upon reading bytes from memory (2 bytes at a time)
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_op_cls() {}

    #[test]
    fn test_op_6xkk_set_vx() {}

    #[test]
    fn test_op_1nnn_jump() {}

    #[test]
    fn test_op_7xkk_add_to_vx() {}

    #[test]
    fn test_op_annn_set_index() {}

    #[test]
    fn test_op_dxyn_draw() {}

    // PC should never go past RAM_SIZE or should wrap if it does?
}
