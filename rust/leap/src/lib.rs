pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 // on every year that is evenly divisible by 4
    && (year % 100 != 0 // except every year that is evenly divisible by 100
      || year % 400 == 0) // unless the year is also evenly divisible by 400
}
