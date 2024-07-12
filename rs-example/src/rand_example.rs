use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    // 定义字符集，包括大小写字母和数字
    const CHARSET: &[u8] = b"abcdefg123456789";
    let mut rng = rand::thread_rng();
    let random_string: String = std::iter::repeat(())
        .map(|()| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .take(length)
        .collect();
    random_string
}

#[test]
fn test_generate_random_string() {
    let random_string = generate_random_string(10);
    assert_eq!(random_string.len(), 10);
}
