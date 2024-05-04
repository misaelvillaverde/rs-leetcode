#![allow(dead_code)]

struct SegmentTree {
    values: Vec<i32>,
    offset: usize,
}

impl SegmentTree {
    fn new(items: &Vec<i32>) -> Self {
        let n = items.len();
        let mut v = vec![0; 2 * n - 1];

        for (idx, &item) in items.iter().enumerate() {
            v[idx + n - 1] = item;
        }

        for idx in (0..v.len() / 2).rev() {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;

            let mut sum = 0;

            if left > 0 {
                sum = v[left];
            }

            if right < v.len() {
                sum += v[right];
            }

            v[idx] = sum;
        }

        SegmentTree {
            values: v,
            offset: n - 1,
        }
    }

    fn query(&self, left: usize, right: usize) -> i32 {
        let mut a = self.offset + left;
        let mut b = self.offset + right;

        let mut sum: i32 = 0;

        loop {
            if a % 2 == 0 {
                sum += self.values[a];
                a += 1;
            }

            if b % 2 != 0 {
                sum += self.values[b];
                b -= 1;
            }

            if a > b {
                break;
            }

            a = (a - 1) / 2;
            b = (b - 1) / 2;
        }

        sum
    }

    fn update(&mut self, index: usize, new: i32) {
        let mut idx = self.offset + index;

        self.values[idx] = new;

        idx = (idx - 1) / 2;

        loop {
            self.values[idx] = self.values[2 * idx + 1] + self.values[2 * idx + 2];

            if idx == 0 {
                return;
            }

            idx = (idx - 1) / 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTree;

    #[test]
    fn even() {
        let items = vec![5, 8, 6, 3, 2, 7, 2, 6];

        let mut tree = SegmentTree::new(&items);

        assert_eq!(tree.values, vec![39, 22, 17, 13, 9, 9, 8, 5, 8, 6, 3, 2, 7, 2, 6]);

        assert_eq!(tree.query(1, 3), 17);
        assert_eq!(tree.query(0, 1), 13);
        assert_eq!(tree.query(0, 2), 19);
        assert_eq!(tree.query(3, 5), 12);
        assert_eq!(
            tree.query(0, items.len() - 1),
            items
                .clone()
                .into_iter()
                .reduce(|acc, cur| acc + cur)
                .unwrap()
        );

        // Update
        tree.update(0, 6);
        assert_eq!(vec![40, 23, 17, 14, 9, 9, 8, 6, 8, 6, 3, 2, 7, 2, 6], tree.values);
        assert_eq!(tree.query(0, items.len() - 1), 40);

        tree.update(2, 1);
        assert_eq!(vec![35, 18, 17, 14, 4, 9, 8, 6, 8, 1, 3, 2, 7, 2, 6], tree.values);
        assert_eq!(tree.query(0, items.len() - 1), 35)
    }

    #[test]
    fn odd() {
        let items = vec![5, 8, 6, 3, 2, 7, 2];

        let tree = SegmentTree::new(&items);

        // println!("tree: {:?}", tree.values);

        let expected_values = vec![33, 19, 14, 14, 5, 9, 5, 8, 6, 3, 2, 7, 2];

        assert_eq!(tree.values, expected_values);
    }
}
