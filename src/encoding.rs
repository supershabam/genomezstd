use core::panic;

fn letter_to_u4(c: &u8) -> u8 {
    match c.to_ascii_lowercase() {
        b'a' => 0b0000,
        b'c' => 0b0001,
        b'g' => 0b0010,
        b't' => 0b0011,
        b'n' => 0b0100,
        b'm' => 0b0101,
        b'r' => 0b0110,
        b'y' => 0b0111,
        b'w' => 0b1000,
        b'k' => 0b1001,
        b'b' => 0b1010,
        b's' => 0b1011,
        _ => panic!("invalid input {}", c),
    }
}

pub fn encode(seq: &[u8]) -> Vec<u8> {
    let s = seq.chunks(2).map(|c| match c {
        [a, b] => letter_to_u4(a) << 4 | letter_to_u4(b),
        [a] => letter_to_u4(a) << 4 | 0b1111,
        _ => panic!("unexpected chunk size"),
    });
    s.collect()
}
