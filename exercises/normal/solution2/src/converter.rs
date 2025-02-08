pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    //todo!()
    let (num, from_base) = if num_str.contains('(') {
        let num = num_str.split('(').next().unwrap();
        let from_base = num_str
            .split('(')
            .nth(1)
            .unwrap()
            .split(')')
            .next()
            .unwrap();
        (num, from_base)
    } else {
        (num_str, "10")
    };
    let num = u32::from_str_radix(num, from_base.parse().unwrap()).unwrap();
    let mut result = String::new();
    let mut num = num;
    while num > 0 {
        let rem = num % to_base;
        let rem = if rem < 10 {
            (rem as u8 + b'0') as char
        } else {
            (rem - 10 + 'a' as u32) as u8 as char
        };
        result.push(rem);
        num /= to_base;
    }
    result.chars().rev().collect()
}
