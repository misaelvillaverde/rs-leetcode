// 729

#![allow(dead_code)]

/**
 *  ["MyCalendar", "book", "book", "book"]
 *  [[], [10, 20], [15, 25], [20, 30]]
 */

struct MyCalendar {
    registry: Vec<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        MyCalendar { registry: vec![] }
    }

    // O(n), cuadratic if taking calls into account
    fn book_cuadratic(&mut self, start: i32, end: i32) -> bool {
        for &(s, e) in &self.registry {
            if start < e && s < end {
                return false;
            }
        }

        self.registry.push((start, end));

        true
    }

    // O(logn), nlogn if taking calls into account
    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.registry.len() <= 0 {
            self.registry.push((start, end));
            return true;
        }

        let mut left = 0;
        let mut right = self.registry.len() - 1;

        let mut place_at: Option<usize> = None;

        while left <= right {
            let middle = left + (right - left) / 2; // (x+y) / 2 => (x-y+2y) / 2 => [(x-y) / 2] + y

            let (cur_start, cur_end) = self.registry[middle];

            if start >= cur_end {
                if self.registry.len() > middle + 1 {
                    let (next_start, _) = self.registry[middle + 1];

                    if end <= next_start {
                        place_at = Some(middle + 1);
                        break;
                    }
                    left = middle + 1;
                    continue;
                } else {
                    place_at = Some(middle + 1);
                    break;
                }
            }

            if end <= cur_start {
                if middle > 0 {
                    let (_, prev_end) = self.registry[middle - 1];
                    if start >= prev_end {
                        place_at = Some(middle);
                        break;
                    }
                    right = middle - 1;
                    continue;
                } else {
                    place_at = Some(0);
                    break;
                }
            }

            break;
        }

        return match place_at {
            Some(idx) => {
                self.registry.insert(idx, (start, end));
                true
            }
            None => false,
        };
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

#[cfg(test)]
mod tests {
    use super::MyCalendar;

    #[test]
    fn t1() {
        let mut calendar = MyCalendar::new();

        let booking = vec![
            calendar.book(10, 20),
            calendar.book(15, 25),
            calendar.book(20, 30),
            calendar.book(5, 40),
            calendar.book(5, 30),
            calendar.book(5, 5),
            calendar.book(5, 10),
        ];

        println!("1");
        println!("books: {:#?}", calendar.registry);
        println!("booking result: {:#?}", booking);

        let expected = vec![true, false, true, false, false, true, true];

        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(e, booking[i]);
        }
    }

    #[test]
    fn t2() {
        let mut calendar = MyCalendar::new();

        let booking = vec![
            calendar.book(10, 20),
            calendar.book(10, 13),
            calendar.book(20, 40),
        ];

        println!("2");
        println!("books: {:#?}", calendar.registry);
        println!("booking result: {:#?}", booking);

        let expected = vec![true, false, true];

        println!("booking result: {:#?}", booking);

        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(e, booking[i]);
        }
    }

    #[test]
    fn t3() {
        let mut calendar = MyCalendar::new();

        let booking = vec![
            calendar.book(37, 50),
            calendar.book(33, 50),
            calendar.book(4, 17),
            calendar.book(35, 48),
            calendar.book(8, 25),
        ];

        println!("3");
        println!("books: {:#?}", calendar.registry);
        println!("booking result: {:#?}", booking);

        let expected = vec![true, false, true, false, false];

        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(e, booking[i]);
        }
    }
}
