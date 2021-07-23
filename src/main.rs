use riscv_emulator::MEMORY;

fn main() {
    let res = u64::from_le_bytes((-1i64).to_le_bytes()) >> 2 == u64::MAX;
    println!("{}",res);
}
