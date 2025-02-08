pub fn goldbach_conjecture() -> String {
    let is_prime = |n: u64| {
        if n < 2 {
            return false;
        }
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
        true
    };
    let mut prime_vec: Vec<u64> = Vec::new();
    let mut result_vec: Vec<u64> = Vec::new();
    let mut temp: u64 = 3;

    prime_vec.push(2);

    while result_vec.len() < 2 {
        if is_prime(temp) {
            prime_vec.push(temp);
        } else {
            let mut index: usize = 0;
            let mut flag: bool = true;
            while index < prime_vec.len() && temp > prime_vec[index] {
                let mut tmp: u64 = 1;
                while tmp * tmp * 2 + prime_vec[index] < temp {
                    tmp += 1;
                }
                if tmp * tmp * 2 + prime_vec[index] == temp {
                    flag = false;
                    break;
                }
                index += 1;
            }

            if flag {
                result_vec.push(temp);
            }
        }

        temp += 2;
    }

    return format!("{},{}", result_vec[0], result_vec[1]);
}
