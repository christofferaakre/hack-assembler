use std::collections::HashSet;

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
