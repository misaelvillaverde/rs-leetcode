#![allow(dead_code)]

struct QueueReconstructionByHeight {}

impl QueueReconstructionByHeight {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people_clone = people.clone();

        people_clone.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut result: Vec<Vec<i32>> = vec![vec![-1, 2]; people_clone.len()];

        for person in &people_clone {
            let mut k = person[1];
            let mut res_idx = 0;

            while k > 0 || result[res_idx][0] != -1 {
                if result[res_idx][0] >= person[0] || result[res_idx][0] == -1 {
                    k -= 1;
                }
                res_idx += 1;
            }

            result[res_idx] = person.to_vec();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::queue_reconstruction_by_height::QueueReconstructionByHeight;

    #[test]
    fn t1() {
        let people = vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2],
        ];

        let expected = vec![
            vec![5, 0],
            vec![7, 0],
            vec![5, 2],
            vec![6, 1],
            vec![4, 4],
            vec![7, 1],
        ];

        let result = QueueReconstructionByHeight::reconstruct_queue(people);

        assert_eq!(result, expected);
    }

    #[test]
    fn t2() {
        let people = vec![
            vec![6, 0],
            vec![5, 0],
            vec![4, 0],
            vec![3, 2],
            vec![2, 2],
            vec![1, 4],
        ];

        let expected = vec![
            vec![4, 0],
            vec![5, 0],
            vec![2, 2],
            vec![3, 2],
            vec![1, 4],
            vec![6, 0],
        ];

        let result = QueueReconstructionByHeight::reconstruct_queue(people);

        assert_eq!(result, expected);
    }
}
