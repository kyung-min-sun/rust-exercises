pub fn find_min_difference(time_points: Vec<&str>) -> i32 {
    if time_points.len() == 0 {
        return -1;
    }

    let mut minute_points: Vec<i32> = time_points
        .iter()
        .filter_map(|x| {
            let parts: Vec<&str> = x.split(":").collect();
            let hour: i32 = parts.get(0).unwrap().parse().unwrap();
            let minute: i32 = parts.get(1).unwrap().parse().unwrap();
            Some((hour * 60) + minute)
        })
        .collect();

    minute_points.sort();

    println!("{:#?}", minute_points);

    let mut minimum_difference = -1;
    let mut last_minute = minute_points[0];

    for minute_point in minute_points[1..].iter() {
        if minute_point - last_minute < minimum_difference || minimum_difference == -1 {
            minimum_difference = minute_point - last_minute;
        }
        last_minute = *minute_point;
    }
    minimum_difference
}

#[cfg(test)]
mod test {
    use crate::find_min_difference;

    #[test]
    pub fn no_points() {
        assert_eq!(find_min_difference(vec![]), -1)
    }

    #[test]
    pub fn no_time_difference() {
        assert_eq!(find_min_difference(vec!["12:01", "12:01", "12:01"]), 0)
    }

    #[test]
    pub fn one_minute_difference() {
        assert_eq!(find_min_difference(vec!["12:01", "12:02", "12:04"]), 1)
    }

    #[test]
    pub fn ten_minute_difference() {
        assert_eq!(find_min_difference(vec!["12:01", "12:11"]), 10)
    }

    #[test]
    pub fn sixty_minute_difference() {
        assert_eq!(find_min_difference(vec!["13:01", "12:01"]), 60)
    }

    #[test]
    pub fn full_day_difference() {
        assert_eq!(find_min_difference(vec!["00:00", "23:59"]), 60 * 23 + 59)
    }
}
