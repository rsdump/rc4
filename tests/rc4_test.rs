use rc4;
use std::iter::repeat;

#[test]
fn rc4_main() {
    let mut c = rc4::Cipher::new("Secret".as_bytes()).unwrap();
    let src = "Attack at dawn";
    let mut dst: Vec<u8> = repeat(0).take(src.len()).collect();
    c.xor(src.as_bytes(), &mut dst);
    assert_eq!(
        dst,
        vec![0x45, 0xA0, 0x1F, 0x64, 0x5F, 0xC3, 0x5B, 0x38, 0x35, 0x52, 0x54, 0x4B, 0x9B, 0xF5]
    );
}
