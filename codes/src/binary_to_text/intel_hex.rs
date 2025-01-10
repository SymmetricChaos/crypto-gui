use super::BinaryToText;
use crate::{errors::CodeError, traits::Code};
use utils::byte_formatting::ByteFormat;

const EOF: &str = ":00000001FF";

pub struct IntelHex {
    pub mode: ByteFormat,
    pub address: u16,
    pub line_length: u8,
}

impl Default for IntelHex {
    fn default() -> Self {
        Self {
            mode: ByteFormat::Utf8,
            address: 0,
            line_length: 16,
        }
    }
}

impl IntelHex {}

impl BinaryToText for IntelHex {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError> {
        let mut address = self.address;
        let mut out = String::new();
        for chunk in bytes.chunks(self.line_length as usize) {
            let mut check: u8 = 0;
            out.push(':');
            out.push_str(&format!("{:02X?}", chunk.len()));
            check = check.wrapping_add(chunk.len() as u8);
            for byte in address.to_be_bytes() {
                out.push_str(&format!("{:02X?}", byte));
                check = check.wrapping_add(byte);
            }
            out.push_str("00");
            for byte in chunk {
                out.push_str(&format!("{:02X?}", byte));
                check = check.wrapping_add(*byte);
            }
            out.push_str(&format!("{:02X?}", (!check).wrapping_add(1)));
            out.push('\n');
            address = address.wrapping_add(chunk.len() as u16);
        }

        out.push_str(EOF);
        Ok(out)
    }
}

impl Code for IntelHex {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        match self.mode {
            ByteFormat::Hex => self.encode_hex(text),
            ByteFormat::Utf8 => self.encode_utf8(text),
            ByteFormat::Base64 => self.encode_base64(text),
            ByteFormat::Binary => self.encode_bits(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        for record in text.split(":").skip(1).map(|s| s.trim()) {
            if record.is_empty() {
                return Err(CodeError::input("records cannot be empty"));
            }
            let rec = match ByteFormat::Hex.text_to_bytes(record) {
                Ok(s) => s,
                Err(e) => return Err(CodeError::Input(e.to_string())),
            };
            let mut check: u8 = 0;
            for byte in &rec {
                check = check.wrapping_add(*byte);
            }
            if check != 0 {
                return Err(CodeError::Input(format!(
                    "checksum dected an error in record: {}",
                    record
                )));
            }
            if rec[0] != (rec.len() - 5) as u8 {
                return Err(CodeError::Input(format!(
                    "length is incorrect in record: {}",
                    record
                )));
            }
            out.push_str(&self.mode.byte_slice_to_text(&rec[4..4 + (rec[0] as usize)]));
        }
        Ok(out)
    }
}

#[cfg(test)]
mod intel_hex_tests {
    use super::*;

    #[test]
    fn encode_test_short() {
        let mut code = IntelHex::default();
        code.mode = ByteFormat::Hex;
        code.address = 0x0010;
        assert_eq!(
            ":0B0010006164647265737320676170A7\n:00000001FF",
            code.encode("6164647265737320676170").unwrap()
        );
    }

    #[test]
    fn encode_test_medium() {
        let mut code = IntelHex::default();
        code.mode = ByteFormat::Hex;
        code.address = 0x0100;
        code.line_length = 16;
        assert_eq!(
            ":10010000214601360121470136007EFE09D2190140\n:100110002146017E17C20001FF5F16002148011928\n:10012000194E79234623965778239EDA3F01B2CAA7\n:100130003F0156702B5E712B722B732146013421C7\n:00000001FF",
            code.encode("214601360121470136007EFE09D219012146017E17C20001FF5F160021480119194E79234623965778239EDA3F01B2CA3F0156702B5E712B722B732146013421").unwrap()
        );
    }

    #[test]
    fn encode_test_long() {
        let mut code = IntelHex::default();
        code.mode = ByteFormat::Hex;
        code.address = 0x0010;
        code.line_length = 255;
        assert_eq!(
            ":FF001000F17B73F8E5476A6EF042B94479B8D48AD96C858729E4C26E5C6E0DCE205B289ADB566C23D63B921A108A9FFF3A91832D36FD401AD5465642E681282A9388E51528F868A4769A476DB0374CF6B271F34B9A016CA2DEF9F37EBA7191EE85D8C3DF924E4C761AC106FC2FAF523B86DF40128204729895BFA0FEF598BA48DF16D6FD2A894D4A399415BA3CFDCE1D9860A2334B8D92A29DB18E1A6A9E60BA4AE41A0434DDAE388FC4810FDE1C0907B812CC40FA155B8AA046BD90C5A08B5B336415643C730D67A29E6210FA24BA968CEFC902DFCDF38F5D915D171BC40F51CC0B136B863D92C24C24A7B57F119B840481189378FDCBD2B2DAEDFF79F1A86FE17E7D43\n:2D001000153094DBD7FD091BC20B730C11BD71177C5FD37E39556728C8FAB55655C6A6FDB26DD6E9559C5C6B793835B7422C\n:00000001FF",
            code.encode("f17b73f8e5476a6ef042b94479b8d48ad96c858729e4c26e5c6e0dce205b289adb566c23d63b921a108a9fff3a91832d36fd401ad5465642e681282a9388e51528f868a4769a476db0374cf6b271f34b9a016ca2def9f37eba7191ee85d8c3df924e4c761ac106fc2faf523b86df40128204729895bfa0fef598ba48df16d6fd2a894d4a399415ba3cfdce1d9860a2334b8d92a29db18e1a6a9e60ba4ae41a0434ddae388fc4810fde1c0907b812cc40fa155b8aa046bd90c5a08b5b336415643c730d67a29e6210fa24ba968cefc902dfcdf38f5d915d171bc40f51cc0b136b863d92c24c24a7b57f119b840481189378fdcbd2b2daedff79f1a86fe17e7d153094dbd7fd091bc20b730c11bd71177c5fd37e39556728c8fab55655c6a6fdb26dd6e9559c5c6b793835b742").unwrap()
        );
    }

    #[test]
    fn decode_test() {
        let mut code = IntelHex::default();
        code.mode = ByteFormat::Hex;
        assert_eq!(
            "6164647265737320676170",
            code.decode(":0B0010006164647265737320676170A7\n:00000001FF")
                .unwrap()
        );
    }

    #[test]
    fn decode_test_medium() {
        let mut code = IntelHex::default();
        code.mode = ByteFormat::Hex;
        assert_eq!(
            "214601360121470136007efe09d219012146017e17c20001ff5f160021480119194e79234623965778239eda3f01b2ca3f0156702b5e712b722b732146013421",
            code.decode(":10010000214601360121470136007EFE09D2190140\n:100110002146017E17C20001FF5F16002148011928\n:10012000194E79234623965778239EDA3F01B2CAA7\n:100130003F0156702B5E712B722B732146013421C7\n:00000001FF").unwrap()
        );
    }

    #[test]
    fn decode_test_long() {
        let mut code = IntelHex::default();
        code.mode = ByteFormat::Hex;
        assert_eq!(
            "f17b73f8e5476a6ef042b94479b8d48ad96c858729e4c26e5c6e0dce205b289adb566c23d63b921a108a9fff3a91832d36fd401ad5465642e681282a9388e51528f868a4769a476db0374cf6b271f34b9a016ca2def9f37eba7191ee85d8c3df924e4c761ac106fc2faf523b86df40128204729895bfa0fef598ba48df16d6fd2a894d4a399415ba3cfdce1d9860a2334b8d92a29db18e1a6a9e60ba4ae41a0434ddae388fc4810fde1c0907b812cc40fa155b8aa046bd90c5a08b5b336415643c730d67a29e6210fa24ba968cefc902dfcdf38f5d915d171bc40f51cc0b136b863d92c24c24a7b57f119b840481189378fdcbd2b2daedff79f1a86fe17e7d153094dbd7fd091bc20b730c11bd71177c5fd37e39556728c8fab55655c6a6fdb26dd6e9559c5c6b793835b742",
            code.decode(":FF001000F17B73F8E5476A6EF042B94479B8D48AD96C858729E4C26E5C6E0DCE205B289ADB566C23D63B921A108A9FFF3A91832D36FD401AD5465642E681282A9388E51528F868A4769A476DB0374CF6B271F34B9A016CA2DEF9F37EBA7191EE85D8C3DF924E4C761AC106FC2FAF523B86DF40128204729895BFA0FEF598BA48DF16D6FD2A894D4A399415BA3CFDCE1D9860A2334B8D92A29DB18E1A6A9E60BA4AE41A0434DDAE388FC4810FDE1C0907B812CC40FA155B8AA046BD90C5A08B5B336415643C730D67A29E6210FA24BA968CEFC902DFCDF38F5D915D171BC40F51CC0B136B863D92C24C24A7B57F119B840481189378FDCBD2B2DAEDFF79F1A86FE17E7D43\n:2D001000153094DBD7FD091BC20B730C11BD71177C5FD37E39556728C8FAB55655C6A6FDB26DD6E9559C5C6B793835B7422C\n:00000001FF")
                .unwrap()
        );
    }
}
