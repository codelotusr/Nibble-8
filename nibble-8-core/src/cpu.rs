use crate::{Bus, memory::ROM_START};

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

    fn fetch(&mut self, bus: &Bus) -> u16 {
        0
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
        // bytes should should be successfully fetched combined into a u16 opcode (Big Endian)
        assert_eq!(opcode, 0x1234);
        // pc should move forward upon reading bytes from memory (2 bytes at a time)
        assert_eq!(cpu.pc, 0x202);
    }
}
