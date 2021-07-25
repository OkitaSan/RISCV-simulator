use std::ops::{Shl, Shr, Sub};

/// a function that gets the specific range of digits
///
/// use 'Gentleman's agreement':
/// * `start <= end`
/// * `0 <= start <= 31`
/// * `0 <= end <= 31`
#[inline]
pub fn get_digits(number: u32, start: u32, end: u32) -> u32 {
    let mask;
    if end - start + 1 < 32 {
        mask = 2u32.pow(end - start + 1).sub(1) << start;
    } else {
        mask = u32::MAX;
    }
    (number & mask) >> start
}

/// A function that fill the one bits in the specific range
/// follows the agreement:
/// use 'Gentleman's agreement':
/// * `start <= end`
/// * `0 <= start <= 31`
/// * `0 <= end <= 31`
/// ## Examples
/// ```
/// let number = 0u32;
/// // test for setting the whole bits
/// assert_eq!(riscv_emulator::util::set_one_digits(0u32,0,31),u32::MAX);
/// // test for setting the regular bits
/// assert_eq!(riscv_emulator::util::set_one_digits(20u32,0,31),u32::MAX);
/// assert_eq!(riscv_emulator::util::set_one_digits(0b00000000_00001111_00011110_11110111u32,0,15),0b00000000_00001111_11111111_11111111u32);
/// assert_eq!(riscv_emulator::util::set_one_digits(0b00000000_00001111_00011110_11110111u32,20,23),0b00000000_11111111_00011110_11110111u32);
/// ```
#[inline]
pub fn set_one_digits(number: u32, start: u32, end: u32) -> u32 {
    if end - start + 1 < 32 {
        2u32.pow(end - start + 1).sub(1).shl(start) | number
    } else {
        u32::MAX
    }
}

/// A function that fills the zero bits in the specific range
/// follows the agreement:
/// use 'Gentleman's agreement':
/// * `start <= end`
/// * `0 <= start <= 31`
/// * `0 <= end <= 31`
/// ## Examples
/// ```
/// assert_eq!(riscv_emulator::util::set_zero_digits(20u32,0,31),0);
/// assert_eq!(riscv_emulator::util::set_zero_digits(0b00000000_00001111_00011110_11110111u32,0,15),0b00000000_00001111_00000000_00000000u32);
/// assert_eq!(riscv_emulator::util::set_zero_digits(0b00000000_00001111_00011110_11110111u32,16,19),0b00000000_00000000_00011110_11110111u32);
/// ```
#[inline]
pub fn set_zero_digits(number: u32, start: u32, end: u32) -> u32 {
    if end - start + 1 < 32 {
        u32::MAX - 2u32.pow(end - start + 1).sub(1).shl(start) & number
    } else {
        0u32
    }
}
