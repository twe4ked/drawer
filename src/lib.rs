use std::convert::TryFrom;

pub mod buffer;
pub mod vm;

#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
    DRW = 0x01,
    MOV = 0x02,
    STO = 0x03,
    INC = 0x04,
    ADD = 0x05,
    DEC = 0x06,
    JNZ = 0x07,
    HLT = 0x08,
    MUL = 0x09,
    JGT = 0x0a,
    SUB = 0x0b,
    JEQ = 0x0c,
    JNE = 0x0d,
    JLT = 0x0e,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        if input <= 0x0e {
            // Safety: Opcode is repr(u8) and the input is <= the largest Opcode varient
            Ok(unsafe { std::mem::transmute::<u8, Self>(input) })
        } else {
            Err(())
        }
    }
}
