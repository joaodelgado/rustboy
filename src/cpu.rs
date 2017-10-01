pub struct Cpu {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    F: u8,
    G: u8,
    H: u8,
    sp: u16,
    pc: u16,
    status: u8, // status flag: sign, zero, parity, carry, aux carry
}

pub enum StatusRegBit {
    Sign,
    Zero,
    Parity,
    Carry,
    AuxCarry,
}

impl Cpu {
    pub fn new() -> Cpu {
        let cpu = Cpu {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: 0,
            G: 0,
            H: 0,
            sp: 0x100,
            pc: 0xFFFE,
            status: 0,
        };
        cpu
    }

    // Check if a certain flag is set
    pub fn status_is_set(&self, bit_enum: StatusRegBit) -> bool {
        match bit_enum {
            StatusRegBit::Sign => (self.status & 0b10000000) == 0b10000000,
            StatusRegBit::Zero => (self.status & 0b01000000) == 0b01000000,
            StatusRegBit::Parity => (self.status & 0b00100000) == 0b00100000,
            StatusRegBit::Carry => (self.status & 0b00010000) == 0b00010000,
            StatusRegBit::AuxCarry => (self.status & 0b000010000) == 0b00010000,
        }
    }

    // Set the defined status flag
    pub fn status_set(&mut self, bit_enum: StatusRegBit) {
        match bit_enum {
            StatusRegBit::Sign => self.status |= 0b10000000,
            StatusRegBit::Zero => self.status |= 0b01000000,
            StatusRegBit::Parity => self.status |= 0b00100000,
            StatusRegBit::Carry => self.status |= 0b00010000,
            StatusRegBit::AuxCarry => self.status |= 0b000010000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_reg() {
        let mut cpu = Cpu::new();

        cpu.status_set(StatusRegBit::Sign);
        assert_eq!(cpu.status_is_set(StatusRegBit::Sign), true);

        cpu.status_set(StatusRegBit::Zero);
        assert_eq!(cpu.status_is_set(StatusRegBit::Zero), true);

        cpu.status_set(StatusRegBit::Parity);
        assert_eq!(cpu.status_is_set(StatusRegBit::Parity), true);

        cpu.status_set(StatusRegBit::Carry);
        assert_eq!(cpu.status_is_set(StatusRegBit::Carry), true);

        cpu.status_set(StatusRegBit::AuxCarry);
        assert_eq!(cpu.status_is_set(StatusRegBit::AuxCarry), true);

        cpu.status = 0;
        assert_eq!(cpu.status_is_set(StatusRegBit::AuxCarry), false);
    }
}
