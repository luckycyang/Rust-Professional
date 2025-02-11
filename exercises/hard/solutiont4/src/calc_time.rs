use std::collections::HashSet;

pub fn time_info(time: &str) -> String {
    // 将输入的日期解析为年、月、日
    let parts: Vec<usize> = time.split('-').map(|s| s.parse().unwrap()).collect();
    let year = parts[0];
    let month = parts[1];
    let day = parts[2];

    // 定义每个月的天数（默认非闰年）
    let mut days_in_month = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // 判断是否为闰年
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        days_in_month[1] = 29;
    }

    // 计算当前是第几天
    let day_of_year = days_in_month.iter().take(month - 1).sum::<usize>() + day;

    // 计算当前是第几周和周几
    let first_day_of_year = 2; // 假设2025年1月1日是周三（2025年实际是周三）
    let total_days = first_day_of_year + day_of_year - 1;
    let week_day = (total_days % 7) + 1; // 周几（1代表周一，7代表周日）
    let week_of_year = if day_of_year <= 11 {
        1 // 1月1日到1月11日为第一周
    } else {
        ((day_of_year - 11 - 1) / 7) + 2 // 从1月12日开始，每7天为一周
    };

    // 计算今年还剩多少天
    let days_in_year = if days_in_month[1] == 29 { 366 } else { 365 };
    let days_remaining = days_in_year - day_of_year;

    // 计算距离正月初一还有多少天（假设春节是2025年1月29日）
    let spring_festival = 29; // 假设正月初一是1月29日
    let days_to_spring_festival = if month == 1 && day <= spring_festival {
        spring_festival - day - 1
    } else if month == 1 {
        0
    } else {
        0 // 春节已过，返回0
    };

    // 定义 A 股休假日期（包括周末）
    let holidays: HashSet<(usize, usize)> = [
        // 元旦
        (1, 1),
        // 春节
        (1, 28),
        (1, 29),
        (1, 30),
        (1, 31),
        (2, 3),
        (2, 4),
        // 清明节
        (4, 4),
        // 劳动节
        (5, 1),
        (5, 2),
        (5, 5),
        // 端午节
        (6, 2),
        // 国庆节和中秋节
        (10, 1),
        (10, 2),
        (10, 3),
        (10, 6),
        (10, 7),
        (10, 8),
    ]
    .iter()
    .cloned()
    .collect();

    // 计算距离下次 A 股开盘还有多少天
    let mut next_stock_open = day_of_year;
    loop {
        // 增加一天
        next_stock_open += 1;

        // 转换为月和日
        let mut current_day = next_stock_open;
        let mut current_month = 1;
        for &days in &days_in_month {
            if current_day > days {
                current_day -= days;
                current_month += 1;
            } else {
                break;
            }
        }

        // 检查是否是周末
        let current_week_day = ((first_day_of_year + next_stock_open - 1) % 7) + 1;
        if current_week_day == 6 || current_week_day == 7 {
            continue; // 周六或周日跳过
        }

        // 检查是否是节假日
        if holidays.contains(&(current_month, current_day)) {
            continue; // 节假日跳过
        }

        // 找到下一个开盘日
        break;
    }
    //
    let days_to_next_stock_open = next_stock_open - day_of_year - 1;

    // 构造结果字符串
    format!(
        "{},{},{},{},{},{}",
        week_of_year,
        week_day,
        day_of_year,
        days_remaining,
        days_to_spring_festival,
        days_to_next_stock_open
    )
}
