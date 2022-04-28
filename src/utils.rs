use crate::types::{ Ubit, OppCode };

pub fn parse_instruction(instruction: u16) -> OppCode {
    let instruction_value = instruction / 0x0100;
    let value: Ubit = (instruction - instruction_value * 0x0100).try_into().unwrap();    

    match instruction_value {
        0x00 => OppCode::Setr(value),
        0x01 => OppCode::Load(value),
        0x02 => OppCode::Store(value),
        0x03 => OppCode::Add(value),
        0x04 => OppCode::Sub(value),
        0x05 => OppCode::Mul(value),
        0x06 => OppCode::Div(value),
        0x0A => OppCode::Jump(value),
        _ => panic!("Invalid instruction: {}", instruction),
    }
}
