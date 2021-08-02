use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{stdin, Read, Write};

use drawer::Opcode;

fn read_stdin() -> String {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("unable to read from STDIN");
    buffer
}

fn try_parse_register(input: &str) -> Option<u8> {
    match input {
        "A" => Some(0x0),
        "B" => Some(0x1),
        "C" => Some(0x2),
        "D" => Some(0x3),
        "E" => Some(0x4),
        "F" => Some(0x5),
        "G" => Some(0x6),
        "H" => Some(0x7),
        "S" => Some(0x8),
        "T" => Some(0x9),
        "U" => Some(0xa),
        "V" => Some(0xb),
        "W" => Some(0xc),
        "X" => Some(0xd),
        "Y" => Some(0xe),
        "Z" => Some(0xf),
        _ => None,
    }
}

fn parse_register(input: Option<&str>) -> u8 {
    try_parse_register(input.expect("missing register")).expect("not a register")
}

fn parse_u16(input: Option<&str>) -> u16 {
    input.expect("missing value").parse().expect("not a u16")
}

enum Value {
    Register(u8),
    Uint(u16),
}

fn parse_value(input: &str) -> Value {
    if let Some(r) = try_parse_register(input) {
        Value::Register(r)
    } else {
        let value = parse_u16(Some(input));
        Value::Uint(value)
    }
}

fn add_instruction(buffer: &mut Vec<u8>, opcode: Opcode, r1: u8, operand: Option<&str>) {
    let operand = operand.expect("missing operand");
    match parse_value(operand) {
        Value::Register(r2) => {
            buffer.push(opcode as u8 | 0x80);
            buffer.push(r1);
            buffer.push(r2);
        }
        Value::Uint(value) => {
            buffer.push(opcode as u8);
            buffer.push(r1);
            buffer.extend_from_slice(&value.to_le_bytes());
        }
    }
}

struct Labels<'a> {
    inner: HashMap<&'a str, u16>,
}

impl<'a> Labels<'a> {
    fn new(input: &'a str) -> Self {
        let mut labels = HashMap::new();
        let mut instruction_count = 0;

        for line in input.lines() {
            let mut parts = line.trim().split_whitespace();

            if let Some(prefix) = parts.next() {
                if let Ok(_) = Opcode::try_from(prefix) {
                    instruction_count += 1;
                } else if prefix.ends_with(':') {
                    if labels.contains_key(prefix) {
                        panic!("re-used label: {}", prefix);
                    } else {
                        labels.insert(prefix, instruction_count);
                    }
                }
            }
        }

        Labels { inner: labels }
    }

    fn get(&self, label: Option<&str>) -> Result<u16, String> {
        if let Some(label) = label {
            self.inner
                .get(label)
                .ok_or_else(|| format!("label not found {}", label))
                .map(|n| *n)
        } else {
            Err("missing label".to_string())
        }
    }
}

fn main() {
    let input = read_stdin();

    let labels = Labels::new(&input);

    // Find width and height
    let mut width = None;
    let mut height = None;

    for line in input.lines() {
        let mut parts = line.trim().split_whitespace();

        if width.is_some() && height.is_some() {
            break;
        }

        if let Some(prefix) = parts.next() {
            match prefix {
                "WIDTH" => {
                    width = Some(parse_u16(parts.next()));
                }
                "HEIGHT" => {
                    height = Some(parse_u16(parts.next()));
                }
                _ => continue,
            }
        }
    }

    let mut out = Vec::new();

    // Width
    out.extend_from_slice(&width.expect("missing width").to_le_bytes());

    // Height
    out.extend_from_slice(&height.expect("missing height").to_le_bytes());

    for line in input.lines() {
        let mut parts = line.trim().split_whitespace();

        if let Some(prefix) = parts.next() {
            match prefix {
                "#" | ";" => continue,
                "WIDTH" | "HEIGHT" => continue,
                "DRW" => {
                    out.push(Opcode::DRW as u8);
                }
                "MOV" => {
                    out.push(Opcode::MOV as u8);
                }
                "STO" => {
                    let r1 = parse_register(parts.next());
                    add_instruction(&mut out, Opcode::STO, r1, parts.next())
                }
                "INC" => {
                    out.push(Opcode::INC as u8);
                    let register = parse_register(parts.next());
                    out.push(register);
                }
                "DEC" => {
                    out.push(Opcode::DEC as u8);
                    let register = parse_register(parts.next());
                    out.push(register);
                }
                "JNZ" => {
                    out.push(Opcode::JNZ as u8);
                    let register = parse_register(parts.next());
                    out.push(register);
                    let addr = labels.get(parts.next()).unwrap();
                    out.extend_from_slice(&addr.to_le_bytes());
                }
                "JGT" => {
                    let register = parse_register(parts.next());
                    add_instruction(&mut out, Opcode::JGT, register, parts.next());
                    let addr = labels.get(parts.next()).unwrap();
                    out.extend_from_slice(&addr.to_le_bytes());
                }
                "JLT" => {
                    let register = parse_register(parts.next());
                    add_instruction(&mut out, Opcode::JLT, register, parts.next());
                    let addr = labels.get(parts.next()).unwrap();
                    out.extend_from_slice(&addr.to_le_bytes());
                }
                "JEQ" => {
                    let register = parse_register(parts.next());
                    add_instruction(&mut out, Opcode::JEQ, register, parts.next());
                    let addr = labels.get(parts.next()).unwrap();
                    out.extend_from_slice(&addr.to_le_bytes());
                }
                "JNE" => {
                    let register = parse_register(parts.next());
                    add_instruction(&mut out, Opcode::JNE, register, parts.next());
                    let addr = labels.get(parts.next()).unwrap();
                    out.extend_from_slice(&addr.to_le_bytes());
                }
                "MUL" => {
                    let register = parse_register(parts.next());
                    add_instruction(&mut out, Opcode::MUL, register, parts.next())
                }
                "DIV" => {
                    let register = parse_register(parts.next());
                    add_instruction(&mut out, Opcode::DIV, register, parts.next())
                }
                "ADD" => {
                    let r1 = parse_register(parts.next());
                    let operand_2 = parts.next().expect("missing operand 2");
                    if let Some(r2) = try_parse_register(operand_2) {
                        out.push(Opcode::ADD as u8 | 0x80);
                        out.push(r1);
                        out.push(r2);
                    } else {
                        out.push(Opcode::ADD as u8);
                        out.push(r1);
                        let value = parse_u16(Some(operand_2));
                        out.extend_from_slice(&value.to_le_bytes());
                    }
                }
                "SUB" => {
                    let register = parse_register(parts.next());
                    add_instruction(&mut out, Opcode::SUB, register, parts.next())
                }
                "HLT" => {
                    out.push(Opcode::HLT as u8);
                }
                _ => {
                    if prefix.ends_with(':') {
                        // Labels are already processed, move on
                    } else {
                        panic!("bad prefix: {}", prefix)
                    }
                }
            }
        }

        assert!(parts.next().is_none());
    }

    let mut file = File::create("program.bin").expect("unable to create file");
    file.write_all(&out).expect("unable to write to file");
}
