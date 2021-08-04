use std::convert::TryFrom;

pub mod buffer;
pub mod instruction;
pub mod vm;

#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
    DRW = 0x01,
    FWD = 0x02,
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
    DIV = 0x0f,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        if input <= 0x0f {
            // Safety: Opcode is repr(u8) and the input is <= the largest Opcode varient
            Ok(unsafe { std::mem::transmute::<u8, Self>(input) })
        } else {
            Err(())
        }
    }
}

impl TryFrom<&str> for Opcode {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "DRW" => Ok(Opcode::DRW),
            "FWD" => Ok(Opcode::FWD),
            "STO" => Ok(Opcode::STO),
            "INC" => Ok(Opcode::INC),
            "ADD" => Ok(Opcode::ADD),
            "DEC" => Ok(Opcode::DEC),
            "JNZ" => Ok(Opcode::JNZ),
            "HLT" => Ok(Opcode::HLT),
            "MUL" => Ok(Opcode::MUL),
            "JGT" => Ok(Opcode::JGT),
            "SUB" => Ok(Opcode::SUB),
            "JEQ" => Ok(Opcode::JEQ),
            "JNE" => Ok(Opcode::JNE),
            "JLT" => Ok(Opcode::JLT),
            "DIV" => Ok(Opcode::DIV),
            _ => Err(()),
        }
    }
}
