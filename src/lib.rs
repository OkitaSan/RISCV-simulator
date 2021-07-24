pub mod parser;

pub mod registers;

pub mod instructions;

pub mod util;

/// memory size
pub const MEMORY_SIZE:usize = 134217728;
/// memory(byte array)
pub static MEMORY:[u8;MEMORY_SIZE] = [0;MEMORY_SIZE];

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_get_digit_works(){
        let case1 = 0b110011010000000000000000u32;
        assert_eq!(util::get_digits(case1, 16, 19),0b1101);
        assert_eq!(util::get_digits(case1, 0, 31),0b110011010000000000000000u32);
        // test 'gentleman's agreement'
        assert_eq!(util::get_digits(case1, 0, 0),0);
        assert_eq!(util::get_digits(case1, 16, 16),1);
        assert_eq!(util::get_digits(case1, 31, 31),0);
    }
    #[test]
    fn test_rtype(){
        let r_inst1:instructions::RType = 0b0000000_10111_00111_000_01100_0110011u32.into();
        let r_inst2:instructions::RType = 0b0100000_01001_10110_000_10110_0110011u32.into();
        let r_inst3 = instructions::RTypeDebug::from(instructions::RType::new(0b0000000, 0b10111, 0b00111, 0b000, 0b01100, 0b0110011));
        let r_inst4 = instructions::RTypeDebug::from(instructions::RType::new(0b0100000,0b01001,0b10110,0b000,0b10110,0b0110011));
        assert_eq!(
            instructions::RTypeDebug::from(r_inst1),
            instructions::RTypeDebug{
                funct7:0b0000000,
                rs2:0b10111,
                rs1:0b00111,
                funct3:0b000,
                rd:0b01100,
                opcode:0b0110011
            }
        );
        assert_eq!(
            instructions::RTypeDebug::from(r_inst2),
            instructions::RTypeDebug{
                funct7:0b0100000,
                rs2:0b01001,
                rs1:0b10110,
                funct3:0b000,
                rd:0b10110,
                opcode:0b0110011
            }
        );
        assert_eq!(
            instructions::RTypeDebug::from(r_inst3),
            instructions::RTypeDebug{
                funct7:0b0000000,
                rs2:0b10111,
                rs1:0b00111,
                funct3:0b000,
                rd:0b01100,
                opcode:0b0110011
            }
        );
        assert_eq!(
            instructions::RTypeDebug::from(r_inst4),
            instructions::RTypeDebug{
                funct7:0b0100000,
                rs2:0b01001,
                rs1:0b10110,
                funct3:0b000,
                rd:0b10110,
                opcode:0b0110011
            }
        );
    }
}