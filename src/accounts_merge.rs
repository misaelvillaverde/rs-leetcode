#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash};

struct UnionFindMap<V> {
    parent: HashMap<V, V>,
    level: HashMap<V, i32>,
}

impl<V: Eq + Hash + Clone> UnionFindMap<V> {
    pub fn new() -> Self {
        UnionFindMap {
            parent: HashMap::new(),
            level: HashMap::new(),
        }
    }

    pub fn find_root(&mut self, node: V) -> V {
        let parent = self.parent.get(&node).unwrap().clone();

        if node == parent {
            return node;
        }

        let root = self.find_root(parent);

        // Path compression
        self.parent.insert(node, root.clone());

        root
    }

    pub fn merge(&mut self, a: &V, b: &V) {
        let root1 = self.find_root(a.clone());
        let root2 = self.find_root(b.clone());

        if root1 == root2 {
            return;
        }

        let level1 = *self.level.entry(root1.clone()).or_insert(0);
        let level2 = *self.level.entry(root2.clone()).or_insert(0);

        if level1 > level2 {
            self.parent.insert(root2, root1);
            return;
        }

        if level1 < level2 {
            self.parent.insert(root1, root2);
            return;
        }

        self.parent.insert(root2, root1.clone());
        self.level.entry(root1).and_modify(|level| *level += 1);
    }
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            par: (0..size).collect(),
            rank: vec![0; size]
        }
    }
}

struct AccountsMerge {}

impl AccountsMerge {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut names: HashMap<String, String> = HashMap::new();

        let mut union_find = UnionFindMap::new();

        for account in &accounts {
            for (index, data) in account.iter().enumerate() {
                if index == 0 {
                    continue;
                }

                names.insert(data.clone(), account[0].clone());
                union_find.parent.insert(data.clone(), data.clone());
            }
        }

        for account in accounts {
            let mut a = String::from("");
            for (index, data) in account.into_iter().enumerate() {
                match index {
                    0 => continue,
                    1 => {
                        a = data;
                        continue;
                    }
                    _ => union_find.merge(&a, &data),
                }
            }
        }

        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        let union_keys: Vec<_> = union_find.parent.keys().cloned().collect();
        for val in union_keys {
            let root = union_find.find_root(val.clone());
            graph
                .entry(root)
                .and_modify(|v| v.push(val.clone()))
                .or_insert(vec![val]);
        }

        let mut result: Vec<Vec<String>> = vec![];

        for (k, emails) in graph {
            let name = names[&k].clone();

            let mut curr = vec![name];

            for email in emails {
                match curr.binary_search(&email) {
                    Ok(mut pos) | Err(mut pos) => {
                        if pos == 0 {
                            pos += 1;
                        }
                        curr.insert(pos, email);
                    }
                }
            }

            result.push(curr);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::AccountsMerge;

    fn sort_accounts(v: &mut Vec<Vec<String>>) {
        v.sort_unstable();
    }

    #[test]
    fn t1() {
        let accounts: Vec<Vec<String>> = vec![
            vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            vec!["John", "johnsmith@mail.com", "john00@mail.com"],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();

        let mut merged_accounts = AccountsMerge::accounts_merge(accounts);

        let mut expected: Vec<Vec<String>> = [
            vec![
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com",
            ],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();

        assert_eq!(merged_accounts.len(), expected.len());

        sort_accounts(&mut merged_accounts);
        sort_accounts(&mut expected);

        println!(
            "Merged:\n{:#?}\nExpected:\n{:#?}",
            merged_accounts, expected
        );

        assert_eq!(merged_accounts, expected);
    }

    #[test]
    fn t2() {
        let accounts: Vec<Vec<String>> = [
            ["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
            ["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
            ["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
            ["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
            ["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();

        let mut merged_accounts = AccountsMerge::accounts_merge(accounts);

        let mut expected: Vec<Vec<String>> = [
            ["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
            ["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
            ["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
            ["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
            ["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();

        assert_eq!(merged_accounts.len(), expected.len());

        sort_accounts(&mut merged_accounts);
        sort_accounts(&mut expected);

        println!(
            "Merged:\n{:#?}\nExpected:\n{:#?}",
            merged_accounts, expected
        );

        assert_eq!(merged_accounts, expected);
    }

    #[test]
    fn t3() {
        let accounts: Vec<Vec<String>> = [
            vec!["David", "Avid0@m.co", "David0@m.co", "David1@m.co"],
            vec!["David", "Gvid3@m.co", "David3@m.co", "David4@m.co"],
            vec!["David", "David4@m.co", "David5@m.co"],
            vec!["David", "David2@m.co", "David3@m.co"],
            vec!["David", "David1@m.co", "David2@m.co"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();

        let mut merged_accounts = AccountsMerge::accounts_merge(accounts);

        let mut expected: Vec<Vec<String>> = [[
            "David",
            "Avid0@m.co",
            "David0@m.co",
            "David1@m.co",
            "David2@m.co",
            "David3@m.co",
            "David4@m.co",
            "David5@m.co",
            "Gvid3@m.co",
        ]]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();

        assert_eq!(merged_accounts.len(), expected.len());

        sort_accounts(&mut merged_accounts);
        sort_accounts(&mut expected);

        println!(
            "Merged:\n{:#?}\nExpected:\n{:#?}",
            merged_accounts, expected
        );

        assert_eq!(merged_accounts, expected);
    }
}
