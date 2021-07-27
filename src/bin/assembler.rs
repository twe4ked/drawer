use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, Read, Write};

fn read_stdin() -> String {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("unable to read from STDIN");
    buffer
}

fn try_parse_register(input: &str) -> Option<u8> {
    match input {
        "A" => Some(0),
        "B" => Some(1),
        "C" => Some(2),
        "D" => Some(3),
        "E" => Some(4),
        "F" => Some(5),
        "G" => Some(6),
        "H" => Some(7),
        _ => None,
    }
}

fn parse_register(input: Option<&str>) -> u8 {
    try_parse_register(input.expect("missing register")).expect("not a register")
}

fn parse_u16(input: Option<&str>) -> u16 {
    input.expect("missing value").parse().expect("not a u16")
}

fn main() {
    let mut instruction_count = 0;
    let mut out = Vec::new();
    let mut labels = HashMap::<&str, u16>::new();

    for line in read_stdin().lines() {
        let mut parts = line.trim().split_whitespace();

        if let Some(prefix) = parts.next() {
            match prefix {
                "#" => continue,
                "DRW" => {
                    out.push(0x01);
                    instruction_count += 1;
                }
                "MOV" => {
                    out.push(0x02);
                    instruction_count += 1;
                }
                "STO" => {
                    let r1 = parse_register(parts.next());
                    let operand_2 = parts.next().expect("missing operand 2");
                    if let Some(r2) = try_parse_register(operand_2) {
                        out.push(0x03 | 0x80);
                        out.push(r1);
                        out.push(r2);
                    } else {
                        out.push(0x03);
                        out.push(r1);
                        let value = parse_u16(Some(operand_2));
                        out.extend_from_slice(&value.to_le_bytes());
                    }
                    instruction_count += 1;
                }
                "INC" => {
                    out.push(0x04);
                    let register = parse_register(parts.next());
                    out.push(register);
                    instruction_count += 1;
                }
                "DEC" => {
                    out.push(0x06);
                    let register = parse_register(parts.next());
                    out.push(register);
                    instruction_count += 1;
                }
                "JNZ" => {
                    out.push(0x07);
                    let register = parse_register(parts.next());
                    out.push(register);
                    let label = parts.next().expect("missing label");
                    let addr = labels.get(label).expect("label not found");
                    out.extend_from_slice(&addr.to_le_bytes());
                    instruction_count += 1;
                }
                "MUL" => {
                    out.push(0x09);
                    let register = parse_register(parts.next());
                    out.push(register);
                    let register = parse_register(parts.next());
                    out.push(register);
                    let value = parse_u16(parts.next());
                    out.extend_from_slice(&value.to_le_bytes());
                    instruction_count += 1;
                }
                "ADD" => {
                    out.push(0x05);
                    let register = parse_register(parts.next());
                    out.push(register);
                    let value = parse_u16(parts.next());
                    out.extend_from_slice(&value.to_le_bytes());
                    instruction_count += 1;
                }
                "HLT" => {
                    out.push(0x08);
                    instruction_count += 1;
                }
                _ => {
                    if prefix.ends_with(':') {
                        if labels.contains_key(prefix) {
                            panic!("re-used label: {}", prefix);
                        } else {
                            labels.insert(prefix, instruction_count);
                        }
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
