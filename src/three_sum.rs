// 15

#![allow(dead_code)]

struct ThreeSum {}

impl ThreeSum {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut items = nums.clone();

        items.sort_unstable();

        let target = 0;
        let mut answer = vec![];

        for i in 0..items.len() {
            if i > 0 && items[i] == items[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = items.len() - 1;

            while l < r {
                let sum = items[i] + items[l] + items[r];

                if sum == target {
                    answer.push(vec![items[i], items[l], items[r]]);
                    l += 1;
                    while l < r && items[l] == items[l - 1] {
                        l += 1;
                    }
                    continue;
                }

                if sum > target {
                    r -= 1;
                    continue;
                }

                if sum < target {
                    l += 1;
                }
            }
        }

        answer
    }
}

// [-8, -3, -3, 0, 0, 2, 3, 6]

#[cfg(test)]
mod tests {
    use crate::three_sum::ThreeSum;

    #[test]
    fn t1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];

        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        let result = ThreeSum::three_sum(nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn t2() {
        let nums = vec![-8, -3, -3, 0, 0, 2, 3, 6];

        let expected = vec![vec![-8, 2, 6], vec![-3, -3, 6], vec![-3, 0, 3]];

        let result = ThreeSum::three_sum(nums);

        assert_eq!(result, expected);
    }
}
