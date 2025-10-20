use rand::Rng;

#[derive(Debug)]
pub struct ByteArray {
    data: Vec<u8>,
}

impl ByteArray {
    pub fn zero(size: usize) -> Self {
        ByteArray { data: vec![0; size] }
    }

    pub fn random(size: usize) -> Self {
        let data = rand::rng().random_iter().take(size).collect();
        ByteArray { data }
    
    }
    
    pub fn xor(&self, other: &Self) -> Self {
        let data = self.data.iter().zip(&other.data).map(|(a, b)| a ^ b).collect();
        ByteArray { data }
    }

    pub fn from_string(input: &str) -> Self {
        let data = input.as_bytes().to_vec();
        ByteArray { data }
    }

    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    pub fn from_hex(input: &str) -> Result<Self, std::num::ParseIntError> {
        let data = (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()?;
        Ok(ByteArray { data })
    }

    pub fn to_hex(&self) -> String {
        self.data.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
