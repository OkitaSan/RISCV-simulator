use std::ops::{Deref, DerefMut, Shl, Shr, Sub};
pub const INSTRUCTION_LEN:u32 = 32;
pub enum Instructions{

}
/// The R-type instruction in RISC-V
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
pub struct RType(u32);

impl Deref for RType{
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RType{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl RType{
    pub const FUNC7T_LEN:u32 = 7;
    pub const RS2_LEN:u32 = 5;
    pub const RS1_LEN:u32 = 5;
    pub const FUNCT3_LEN:u32 = 3;
    pub const RD_LEN:u32 = 5;
    pub const OPCODE_LEN:u32 = 7;
    /// get funct7 field
    pub fn get_funct7(&self) -> u32{
        self.shr(INSTRUCTION_LEN - Self::FUNC7T_LEN) & 127u32
    }
    /// set funct7 field
    pub fn set_funct7(&mut self,funct7:u32){
        let mut new_rtype_inst = 0u32;
        // set funct7 field
        new_rtype_inst |= funct7 << (INSTRUCTION_LEN - Self::FUNC7T_LEN);
        // set the rest part
        new_rtype_inst |= self.0 & 2u32.pow(INSTRUCTION_LEN - Self::FUNC7T_LEN).sub(1);
        // and write back
        self.0 = new_rtype_inst;
    }
    /// get rs2 field
    pub fn get_rs2(&self) -> u32{
        self.shr(INSTRUCTION_LEN - Self::FUNC7T_LEN - Self::RS2_LEN) & 31u32
    }
    /// set rs2 field
    pub fn set_rs2(&mut self,rs2:u32){
        let mut new_rtype_inst = 0u32;
        // set rs2 field
        new_rtype_inst |= rs2 << (32u32 - 7u32 - 5u32);
        // set the rest part
        // eg, a digit series `a b c d` , if I want to set b
        // then , first I need to set b , by shift the bits I need to shift to len(a b c d) - len(a b) and then or it with [0;len(a b c d)]
        // second I need to set a and c d back to the new result

        // set c and d back
        new_rtype_inst |= self.0 & 2u32.pow(32u32 - 7u32 - 5u32).sub(1);
        // set a back
        new_rtype_inst |= self.0 & (u32::MAX - 2u32.pow(32u32 - 7u32).sub(1));
        // and write back
        self.0 = new_rtype_inst;
    }
    /// get rs1 field
    pub fn get_rs1(&self) -> u32{
        self.shr(32u32 - 7u32 - 5u32 - 5u32) & 31u32
    }
    /// set rs1 field
    pub fn set_rs1(&mut self,rs1:u32){
        let mut new_rtype_inst = 0u32;
        // set rs1 field
        new_rtype_inst |= rs1 << (32u32 - 7u32 - 5u32 - 5u32);
        // set the rest part
        new_rtype_inst |= self.shl(7u32 + 5u32 + 5u32).shr(7u32 + 5u32 + 5u32);
        // and write back
        self.0 = new_rtype_inst;
    }
    /// get funct3 field
    pub fn get_funct3(&self) -> u32{
        self.shr(32u32 - 7u32 - 5u32 - 5u32 - 3u32) & 7u32
    }
    /// set funct3 field
    pub fn set_funct3(&mut self,funct3:u32){
        let mut new_rtype_inst = 0u32;
        // set funct3 field
        new_rtype_inst |= funct3 << (32u32 - 7u32 - 5u32 - 5u32 - 3u32);
        // set the rest part
        new_rtype_inst |= self.shl(7u32 + 5u32 + 5u32 + 3u32).shr(7u32 + 5u32 + 5u32 + 3u32);
        // and write back
        self.0 = new_rtype_inst;
    }
}