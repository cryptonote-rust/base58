use rust_base58::{ToBase58, FromBase58};

const TO_SIZE: usize = 8;
const FROM_SIZE: usize = 11;
// const padding: [u8: "1", FROM_SIZE] = ["1": FROM_SIZE];

#[derive(Debug)]
pub enum Base58Error {
    InvalidChunkData,
    InvalidChunkLength(usize),
}

pub fn to_base58(bytes: Vec<u8>) -> Result<String, Base58Error> {
    let mut base58 = String::new();
    for chunk in bytes.as_slice().chunks(TO_SIZE) {
        println!(" chunk: {:x?}", chunk);
        let mut part = chunk.to_base58();
        println!(" chunk len : {}", chunk.len());
        let exp_len = match chunk.len() {
            8 => 11,
            6 => 9,
            5 => 7,
            len => return Err(Base58Error::InvalidChunkLength(len)),
        };
        let missing = exp_len - part.len();
        if missing > 0 {
            part.insert_str(0, &"11111111111"[..missing]);
        }
        println!(" part : {}", part);
        base58.push_str(&part);
    }
    println!("to base58: \n{:x?}", base58.as_bytes());
    println!("to base58 str: \n{:x?}", base58);
    Ok(base58)
}

pub fn from_base58(base58: String) -> Result<Vec<u8>, Base58Error> {
    let mut extracted: Vec<u8> = vec![];
    let bytes = base58.as_bytes();
    println!("string hex: {:x?}", bytes);
    for chunk in bytes.chunks(FROM_SIZE) {
        let idx = chunk.iter().position(|&c| c != b'1')
            .ok_or(Base58Error::InvalidChunkData)?;
        let part = std::str::from_utf8(&chunk[idx..]).unwrap();
        println!("part: {}", part);
        println!("after extracted part: {}", part);
        let exp_len = match chunk.len() {
            11 => 8,
            9 => 6,
            7 => 5,
            len => return Err(Base58Error::InvalidChunkLength(len)),
        };

        let mut base58_chunk:Vec<u8> = part.from_base58().unwrap();
        if exp_len != base58_chunk.len() {
            return Err(Base58Error::InvalidChunkLength(chunk.len()));
        }
        println!("chunk: {:x?}", base58_chunk);
        extracted.append(&mut base58_chunk);
    }
    println!("from base 58: \n{:x?}", extracted);
    Ok(extracted)
}

#[cfg(test)]
mod tests {
    use super::{to_base58, from_base58};
    #[test]
    fn it_works() {
        let original_data = "BMv8aSohtktbARXdSBrVRkgJhyoNBmxmjHWJaVhFx31gjHRqzxXH5G7DRy6uPdPJhkdrfNpF1K5Di3Cfr78ZuEA3PiaUPG9";
        let expected_decoded = vec![
            0x3d, 0xf1, 0x2e, 0x7, 0xad, 0x81, 0xf0, 0x9d, 0xcc, 0x3e, 0x78,
            0x7b, 0x68, 0xc6, 0xdc, 0x73, 0xea, 0xfd, 0xae, 0xa6, 0x43, 0x33,
            0x6d, 0xfe, 0x62, 0xad, 0xa4, 0xab, 0x3c, 0xc4, 0x9d, 0x67, 0xfc,
            0xcb, 0x86, 0x6f, 0xf0, 0xba, 0xc3, 0xfc, 0x4a, 0x51, 0x22, 0xa,
            0x47, 0x82, 0xcb, 0x8f, 0xdc, 0x59, 0x5a, 0x56, 0xba, 0x90, 0xf,
            0x41, 0xd, 0x28, 0xfe, 0x70, 0xb8, 0x4d, 0x8a, 0x60, 0xc9, 0x5a,
            0x22, 0xb5, 0xee
        ];

        let decoded = from_base58(String::from(original_data))
            .expect("Error decoding from base 58");
        assert_eq!(expected_decoded, decoded);

        let encoded = to_base58(expected_decoded)
            .expect("Error encoding to base 58");
        assert_eq!(encoded, original_data);
    }
}
