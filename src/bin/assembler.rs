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

fn parse_register(input: Option<&str>) -> Result<u8, String> {
    if let Some(input) = input {
        match input {
            "A" => Ok(0x0),
            "B" => Ok(0x1),
            "C" => Ok(0x2),
            "D" => Ok(0x3),
            "E" => Ok(0x4),
            "F" => Ok(0x5),
            "G" => Ok(0x6),
            "H" => Ok(0x7),
            "S" => Ok(0x8),
            "T" => Ok(0x9),
            "U" => Ok(0xa),
            "V" => Ok(0xb),
            "W" => Ok(0xc),
            "X" => Ok(0xd),
            "Y" => Ok(0xe),
            "Z" => Ok(0xf),
            _ => Err(format!("not a register: {}", input)),
        }
    } else {
        Err("missing register".to_string())
    }
}

fn parse_u16(input: Option<&str>) -> u16 {
    input.expect("missing value").parse().expect("not a u16")
}

fn add_instruction_0(buffer: &mut Vec<u8>, opcode: Opcode) {
    buffer.push(opcode as u8);
}

fn add_instruction_1(buffer: &mut Vec<u8>, opcode: Opcode, operand_1: Option<&str>) {
    buffer.push(opcode as u8);
    let register = parse_register(operand_1).unwrap();
    buffer.push(register);
}

fn add_instruction_2(
    buffer: &mut Vec<u8>,
    opcode: Opcode,
    operand_1: Option<&str>,
    operand_2: Option<&str>,
) {
    let r1 = parse_register(operand_1).unwrap();
    if let Ok(r2) = parse_register(operand_2) {
        buffer.push(opcode as u8 | 0x80);
        buffer.push(r1);
        buffer.push(r2);
    } else {
        let value = parse_u16(operand_2);
        buffer.push(opcode as u8);
        buffer.push(r1);
        buffer.extend_from_slice(&value.to_le_bytes());
    }
}

fn add_label(buffer: &mut Vec<u8>, labels: &Labels, label: Option<&str>) {
    let addr = labels.get(label).unwrap();
    buffer.extend_from_slice(&addr.to_le_bytes());
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

    // Version
    out.push(0x01);

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
                "DRW" => add_instruction_0(&mut out, Opcode::DRW),
                "FWD" => add_instruction_0(&mut out, Opcode::FWD),
                "HLT" => add_instruction_0(&mut out, Opcode::HLT),
                "INC" => add_instruction_1(&mut out, Opcode::INC, parts.next()),
                "DEC" => add_instruction_1(&mut out, Opcode::DEC, parts.next()),
                "STO" => add_instruction_2(&mut out, Opcode::STO, parts.next(), parts.next()),
                "MUL" => add_instruction_2(&mut out, Opcode::MUL, parts.next(), parts.next()),
                "DIV" => add_instruction_2(&mut out, Opcode::DIV, parts.next(), parts.next()),
                "ADD" => add_instruction_2(&mut out, Opcode::ADD, parts.next(), parts.next()),
                "SUB" => add_instruction_2(&mut out, Opcode::SUB, parts.next(), parts.next()),
                "JNZ" => {
                    add_instruction_1(&mut out, Opcode::JNZ, parts.next());
                    add_label(&mut out, &labels, parts.next());
                }
                "JGT" => {
                    add_instruction_2(&mut out, Opcode::JGT, parts.next(), parts.next());
                    add_label(&mut out, &labels, parts.next());
                }
                "JLT" => {
                    add_instruction_2(&mut out, Opcode::JLT, parts.next(), parts.next());
                    add_label(&mut out, &labels, parts.next());
                }
                "JEQ" => {
                    add_instruction_2(&mut out, Opcode::JEQ, parts.next(), parts.next());
                    add_label(&mut out, &labels, parts.next());
                }
                "JNE" => {
                    add_instruction_2(&mut out, Opcode::JNE, parts.next(), parts.next());
                    add_label(&mut out, &labels, parts.next());
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
