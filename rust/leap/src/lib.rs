/// The algorithm applies to proleptic Gregorian calendar years before 1, but only if the year is expressed with astronomical year numbering.
/// It is not valid for the BC or BCE notation.
/// The algorithm is not necessarily valid for years in the Julian calendar, such as years before 1752 in the British Empire.
/// The year 1700 was a leap year in the Julian calendar, but not in the Gregorian calendar.
pub fn is_leap_year(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (_, _, 0) => true,
        (_, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}
