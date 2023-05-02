use std::collections::HashSet;

use bitvec::view::BitView;
use bitvec::order::Lsb0;

use crate::{ValueType, register::Register, ops::Operation, jump::Jump};

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
                let value_bytes = value.view_bits::<Lsb0>();
                // we want to treat this u16 as a 15-bit number,
                // so we ignore the MOST significant bit
                let bits = &value_bytes[0..15];
                assert_eq!(bits.len(), 15);
                let bit_string: String = bits.iter().map(|b| b.to_string()).collect();
                bit_string
            }
            Instruction::C { dest, comp, jump } => {
                todo!()
            }
        }
    }
}
