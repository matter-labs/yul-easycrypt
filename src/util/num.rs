use ethereum_types::U256;
use hex::FromHex;

///
/// Parses a 256-bit unsigned hexadecimal number from a string. String may be
/// prefixed with "0x".
///
pub fn from_hex_literal(literal: &str) -> U256 {
    let trimmed = literal.trim_start_matches("0x");
    let padded = format!("{:0>64}", trimmed);
    let from_hex = <[u8; 32]>::from_hex(padded).unwrap();
    U256::from_big_endian(&from_hex)
}

#[cfg(test)]
mod tests {
    fn test(s: &str, reference: &str) {
        assert_eq!(super::from_hex_literal(s).to_string(), reference)
    }

    #[test]
    fn hex_int_conversion_zero() {
        test("0x0", "0");
    }

    #[test]
    fn hex_int_conversion_one() {
        test("0x1", "1");
    }

    #[test]
    fn hex_int_conversion_multidigit() {
        test("0x101", "257");
    }

    #[test]
    fn hex_int_conversion_huge() {
        test(
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            "452312848583266388373324160190187140051835877600158453279131187530910662655",
        );
    }
}
