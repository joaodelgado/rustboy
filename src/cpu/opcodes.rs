pub const CALL_A16: u8 = 0xcd;
pub const DI: u8 = 0xf3;
pub const JP_A16: u8 = 0xc3;
pub const JP_C_A16: u8 = 0xda;
pub const JP_HL: u8 = 0xe9;
pub const JP_NC_A16: u8 = 0xd2;
pub const JP_NZ_A16: u8 = 0xc2;
pub const JP_Z_A16: u8 = 0xca;
pub const JR_R8: u8 = 0x18;
pub const LDH_A8_A: u8 = 0xe0;
pub const LD_A16_A: u8 = 0xea;
pub const LD_A_D8: u8 = 0x3e;
pub const LD_A_A: u8 = 0x7f;
pub const LD_A_B: u8 = 0x78;
pub const LD_A_C: u8 = 0x79;
pub const LD_A_D: u8 = 0x7a;
pub const LD_A_E: u8 = 0x7b;
pub const LD_A_H: u8 = 0x7c;
pub const LD_A_L: u8 = 0x7d;
pub const LD_HL_D16: u8 = 0x21;
pub const LD_SP_HL: u8 = 0xf9;
pub const LD_SP_NN: u8 = 0x31;
pub const PUSH_A16_AF: u8 = 0xf5;
pub const PUSH_A16_BC: u8 = 0xc5;
pub const PUSH_A16_DE: u8 = 0xd5;
pub const PUSH_A16_HL: u8 = 0xe5;
pub const POP_A16_AF: u8 = 0xf1;
pub const POP_A16_BC: u8 = 0xc1;
pub const POP_A16_DE: u8 = 0xd1;
pub const POP_A16_HL: u8 = 0xe1;
pub const NOP: u8 = 0x00;
pub const RET: u8 = 0xc9;
pub const INC_A16_BC: u8 = 0x03;
pub const INC_A16_DE: u8 = 0x13;
pub const INC_A16_HL: u8 = 0x23;
pub const INC_A16_SP: u8 = 0x33;
