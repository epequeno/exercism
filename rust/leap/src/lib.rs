pub fn is_leap_year(year: usize) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 && year % 400 == 0 {
            return true
        }
        return year % 100 != 0
    }
    false
}
