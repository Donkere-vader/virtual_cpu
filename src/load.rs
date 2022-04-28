use crate::types::{ Ubit };


fn hex_char_to_value(hex_char: char) -> u16 {
    match hex_char {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => panic!("Invalid hex char {}", hex_char),
    }
}


fn hex_string_loader(hex_value: String) -> u16 {
    let mut value: u16 = 0;
    let base: u16 = 16;

    for (idx, c) in hex_value.chars().rev().enumerate() {
        value += hex_char_to_value(c) * base.pow(idx as u32);
    }

    value
}


pub fn loads(file_contents: String) -> [u16; u16::MAX as usize] {
    let mut mem = [0; u16::MAX as usize];

    let mut instruction_idx = 0;
    for line in file_contents.trim().split('\n') {
        let mut line = line.split("//").collect::<Vec<&str>>()[0].to_string();
        line = line.trim().to_string();
        if line.len() == 0 {
            continue;
        }
        if !line.starts_with("0x") {
            panic!("CODE ERROR");
        }
        line = line[2..].to_string();

        mem[instruction_idx] = hex_string_loader(line);
        instruction_idx += 1;
    }

    mem
}
