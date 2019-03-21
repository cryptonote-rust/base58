use rust_base58::{ToBase58, FromBase58};
use std::mem::size_of;

const TO_SIZE: usize = 8;
const FROM_SIZE: usize = 11;
// const padding: [u8: "1", FROM_SIZE] = ["1": FROM_SIZE];

pub enum Base58Error {
    InvalidChunkData,
    InvalidChunkLength,
}

pub fn to_base58(bytes: Vec<u8>) -> String {
    let mut base58 = String::new();
    for chunk in bytes.as_slice().chunks(TO_SIZE) {
        print!(" chunk: {:x?} \n", chunk);
        let mut part = chunk.to_base58();
        print!(" chunk len : {} \n", chunk.len());
        let exp_len = match chunk.len() {
            8 => 11,
            6 => 9,
            5 => 7,
            _ => 13,
        };
        if (exp_len == 13) {
            panic!("Invalid chunk length: {}", chunk.len());
        }
        let missing = exp_len - part.len();
        if missing > 0 {
            part.insert_str(0, &"11111111111"[..missing]);
        }
        print!(" part : {} \n", part);
        base58.push_str(&part);
    }
    print!("to base58: \n{:x?}\n", base58.as_bytes());
    print!("to base58 str: \n{:x?}\n", base58);
    base58
}

pub fn from_base58(base58: String) -> Vec<u8> {
    let mut extracted: Vec<u8> = vec![];
    let bytes = base58.as_bytes();
    print!("string hex: {:x?}\n", bytes);
    for chunk in bytes.chunks(FROM_SIZE) {
       let mut idx = 0;
        loop {
            if (chunk[idx] != '1' as u8) {
                break;
            }
                idx += 1;
        }
        let part = std::str::from_utf8(&chunk[idx..]).unwrap();
        print!("part: {} \n", part);
        print!("after extracted part: {} \n", part);
        let exp_len = match chunk.len() {
            11 => 8,
            9 => 6,
            7 => 5,
            _ => 0,
        };
         if (exp_len == 0) {
         panic!("Invalid chunk length: {}", chunk.len());
        }
        
        let mut base58_chunk:Vec<u8> = part.from_base58().unwrap();
        if (exp_len != base58_chunk.len()) {
         panic!("Invalid chunk length: {}", chunk.len());
        }
        print!("chunk: {:x?} \n", base58_chunk);
        extracted.append(&mut base58_chunk);
    }
    print!("from base 58: \n{:x?}\n", extracted);
    extracted
}

#[cfg(test)]
mod tests {
    use super::{to_base58, from_base58};
    #[test]
    fn it_works() {
        let bytes = vec![0x3d, 0xf1, 0x2e, 0x7, 0xad, 0x81, 0xf0, 0x9d, 0xcc, 0x3e, 0x78, 0x7b, 
        0x68, 0xc6, 0xdc, 0x73, 0xea, 0xfd, 0xae, 0xa6, 0x43, 0x33, 0x6d,
         0xfe, 0x62, 0xad, 0xa4, 0xab, 0x3c, 0xc4, 0x9d, 0x67, 0xfc, 0xcb, 0x86, 0x6f, 0xf0, 0xba, 0xc3, 0xfc, 0x4a, 0x51, 0x22, 0xa, 0x47, 0x82, 0xcb, 0x8f, 0xdc, 0x59, 0x5a, 0x56, 0xba, 0x90, 0xf, 0x41, 0xd, 0x28, 0xfe, 0x70, 0xb8, 0x4d, 0x8a, 0x60, 0xc9, 0x5a, 0x22, 0xb5, 0xee];
        assert_eq!(bytes, from_base58(String::from("BMv8aSohtktbARXdSBrVRkgJhyoNBmxmjHWJaVhFx31gjHRqzxXH5G7DRy6uPdPJhkdrfNpF1K5Di3Cfr78ZuEA3PiaUPG9")));

        let address = to_base58(bytes);

        assert_eq!(address, ("BMv8aSohtktbARXdSBrVRkgJhyoNBmxmjHWJaVhFx31gjHRqzxXH5G7DRy6uPdPJhkdrfNpF1K5Di3Cfr78ZuEA3PiaUPG9"));
    }
}
