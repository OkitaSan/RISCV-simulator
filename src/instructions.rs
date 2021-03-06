use crate::util::get_digits;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::ops::{Deref, DerefMut, Shl, Shr, Sub};
pub const INSTRUCTION_LEN: u32 = 32;
pub enum Instructions {
    R(RType),
}
/// The R-type instruction in RISC-V
///
/// instruction layout : `|funct7|rs2|rs1|funct3|rd|opcode`
///
/// ## Instruction format
/// `funct7` : 7
///
/// An additional opcode field.
///
/// `rs2` : 5
///
///  The second register source operand.
///  
/// `rs1` : 5
///
///  The first register source operand.
///
/// `funct3` : 3
///
///  An additional opcode field.
///
/// `rd` : 5
///
///  The register destination operand. It gets the result of the operation.
///
/// `opcode` : 7
///
///  Basic operation of the instruction, and this abbreviation is its traditional name
///
pub struct RType(pub u32);
/// R-type debug only,due to the limitation of serde-rs
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct RTypeDebug {
    pub funct7: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub opcode: u8,
}
impl From<RType> for RTypeDebug {
    fn from(rtype: RType) -> Self {
        RTypeDebug {
            funct7: rtype.get_func7() as u8,
            rs2: rtype.get_rs2() as u8,
            rs1: rtype.get_rs1() as u8,
            funct3: rtype.get_funct3() as u8,
            rd: rtype.get_rd() as u8,
            opcode: rtype.get_opcode() as u8,
        }
    }
}
impl From<u32> for RType {
    fn from(inst: u32) -> Self {
        RType(inst)
    }
}
impl Deref for RType {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl RType {
    pub const FUNC7T_LEN: u32 = 7;
    pub const RS2_LEN: u32 = 5;
    pub const RS1_LEN: u32 = 5;
    pub const FUNCT3_LEN: u32 = 3;
    pub const RD_LEN: u32 = 5;
    pub const OPCODE_LEN: u32 = 7;
    /// construct RType instruction
    pub fn new(func7: u8, rs2: u8, rs1: u8, funct3: u8, rd: u8, opcode: u8) -> RType {
        let mut instruction = 0u32;
        // set opcode
        instruction |= (opcode & 0b01111111) as u32;
        // set rd
        instruction |= ((rd & 0b0001_1111) as u32) << Self::OPCODE_LEN;
        // set funct3
        instruction |= ((rd & 0b0000_0111) as u32) << (Self::OPCODE_LEN + Self::RD_LEN);
        // set rs1
        instruction |=
            ((rd & 0b0001_1111) as u32) << (Self::OPCODE_LEN + Self::RD_LEN + Self::FUNCT3_LEN);
        // set rs2
        instruction |= ((rd & 0b0001_1111) as u32)
            << (Self::OPCODE_LEN + Self::RD_LEN + Self::FUNCT3_LEN + Self::RS1_LEN);
        // set funct7
        instruction |= ((rd & 0b0111_1111) as u32)
            << (Self::OPCODE_LEN + Self::RD_LEN + Self::FUNCT3_LEN + Self::RS1_LEN + Self::RS2_LEN);
        RType(instruction)
    }
    #[inline]
    pub fn get_func7(&self) -> u32 {
        get_digits(
            self.0,
            Self::OPCODE_LEN + Self::RD_LEN + Self::FUNCT3_LEN + Self::RS1_LEN + Self::RS2_LEN,
            Self::OPCODE_LEN
                + Self::RD_LEN
                + Self::FUNCT3_LEN
                + Self::RS1_LEN
                + Self::RS2_LEN
                + Self::FUNC7T_LEN
                - 1,
        )
    }
    #[inline]
    pub fn get_rs2(&self) -> u32 {
        get_digits(
            self.0,
            Self::OPCODE_LEN + Self::RD_LEN + Self::FUNCT3_LEN + Self::RS1_LEN,
            Self::OPCODE_LEN + Self::RD_LEN + Self::FUNCT3_LEN + Self::RS1_LEN + Self::RS2_LEN - 1,
        )
    }
    #[inline]
    pub fn get_rs1(&self) -> u32 {
        get_digits(
            self.0,
            Self::OPCODE_LEN + Self::RD_LEN + Self::FUNCT3_LEN,
            Self::OPCODE_LEN + Self::RD_LEN + Self::FUNCT3_LEN + Self::RS1_LEN - 1,
        )
    }
    #[inline]
    pub fn get_funct3(&self) -> u32 {
        get_digits(
            self.0,
            Self::OPCODE_LEN + Self::RD_LEN,
            Self::OPCODE_LEN + Self::RD_LEN + Self::FUNCT3_LEN - 1,
        )
    }
    #[inline]
    pub fn get_rd(&self) -> u32 {
        get_digits(
            self.0,
            Self::OPCODE_LEN,
            Self::OPCODE_LEN + Self::RD_LEN - 1,
        )
    }
    #[inline]
    pub fn get_opcode(&self) -> u32 {
        get_digits(self.0, 0, Self::OPCODE_LEN - 1)
    }
}
