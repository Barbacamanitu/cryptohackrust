#[derive(Debug)]
struct HexString {
    hex: String
}

#[derive(Debug)]
struct ByteVec {
    bytes: Vec<u8>
}

impl From<&str> for HexString {
    fn from(s: &str) -> Self {
        HexString {
            hex: s.to_owned()
        }
    }
}

impl From<HexString> for ByteVec {
    fn from(h: HexString) -> Self {
        ByteVec {
            bytes: hex::decode(h.hex).unwrap()
        }
    }
}

impl From<Vec<u8>> for ByteVec {
    fn from(v: Vec<u8>) -> Self {
        ByteVec {
            bytes: v.clone(),
        }
    }
}

impl ByteVec {
    pub fn xor(&self, other: &ByteVec) -> ByteVec {
        let res: Vec<u8> = self.bytes.iter().zip(other.bytes.iter()).map(|(&x1, &x2)| x1 ^ x2).collect();
        ByteVec::from(res)
    }

    pub fn to_str(&self) -> &str {
        std::str::from_utf8(self.bytes.as_slice()).unwrap()
    }
}


//https://cryptohack.org/courses/intro/xor1/

fn main() {
    let key1: ByteVec = ByteVec::from( HexString::from("a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313"));
    let key1and2: ByteVec = ByteVec::from(HexString::from("37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e"));
    let key2and3: ByteVec = ByteVec::from(HexString::from("c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1"));
    let flagand1and3and2: ByteVec = ByteVec::from(HexString::from("04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf"));
    
    let key2 = key1and2.xor(&key1);
    let key3 = key2and3.xor(&key2);
    let flag = flagand1and3and2.xor(&key1).xor(&key2).xor(&key3);
    let flag_decoded = flag.to_str();
    println!("Decoded flag: {}", flag_decoded);
}


/*
KEY1 = a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313
KEY2 ^ KEY1 = 37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e
KEY2 ^ KEY3 = c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1
FLAG ^ KEY1 ^ KEY3 ^ KEY2 = 04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf */