use std::collections::HashSet;

use bitvec::prelude::Msb0;
use bitvec::view::BitView;

use crate::{jump::Jump, ops::Operation, register::Register, ValueType};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    A {
        value: ValueType,
    },
    C {
        dest: HashSet<Register>,
        comp: Operation,
        jump: Jump,
    },
}

impl Instruction {
    pub fn codegen(&self) -> String {
        match self {
            Instruction::A { value } => {
                let value_bytes = value.view_bits::<Msb0>();
                // we want to treat this u16 as a 15-bit number,
                // so we ignore the MOST significant bit
                let bits = &value_bytes[1..16];
                assert_eq!(bits.len(), 15);

                let chars: Vec<char> = bits
                    .iter()
                    .map(|b| b.as_ref().clone())
                    .map(|b| match b {
                        false => '0',
                        true => '1',
                    })
                    .collect();

                ['0'].iter().chain(chars.iter()).collect()
            }

            Instruction::C { dest, comp, jump } => {
                // I feel like there must be a cleaner way to do this
                let mut bits = [false; 16];
                bits[0] = true;
                bits[1] = true;
                bits[2] = true;

                let next_seven_bits = comp.codegen();
                assert_eq!(next_seven_bits.len(), 7);
                // sets bits 3-9
                for (i, bit) in next_seven_bits.iter().enumerate() {
                    bits[3 + i] = *bit;
                }

                if dest.contains(&Register::A) {
                    bits[10] = true;
                }

                if dest.contains(&Register::D) {
                    bits[11] = true;
                }

                if dest.contains(&Register::M) {
                    bits[12] = true;
                }

                let last_three_bits = match jump {
                    Jump::NoJump => [false, false, false],
                    Jump::GreaterThan => [false, false, true],
                    Jump::Equal => [false, true, false],
                    Jump::GreaterThanOrEqual => [false, true, true],
                    Jump::LessThan => [true, false, false],
                    Jump::NotEqual => [true, false, true],
                    Jump::LessThanOrEqual => [true, true, false],
                    Jump::Always => [true, true, true],
                };

                bits[13] = last_three_bits[0];
                bits[14] = last_three_bits[1];
                bits[15] = last_three_bits[2];

                let chars = bits
                    .map(|b| match b {
                        false => '0',
                        true => '1',
                    })
                    .to_vec();

                assert_eq!(chars.len(), 16);

                chars.iter().collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_codegen_a() {
        let instruction = Instruction::A { value: 2 };
        let codegen = instruction.codegen();
        let expected = String::from("0000000000000010");

        assert_eq!(codegen, expected);
    }
}
