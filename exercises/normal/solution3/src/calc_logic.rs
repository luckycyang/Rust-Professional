pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    //todo!()
    let result: f64 = 1.0;
    let mut temp: f64 = 1.0;
    for i in 0..n {
        temp *= (365 - i) as f64 / 365.0;
    }

    return result - temp;
}
