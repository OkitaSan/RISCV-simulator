use riscv_emulator::MEMORY;
use riscv_emulator::instructions;
fn main() {
    let r_inst:instructions::RTypeDebug = instructions::RType(0b0000000_00111_01001_000_10011_0110011u32).into();
    let json = serde_json::to_string(&r_inst).unwrap();
    println!("{}",json);
}
