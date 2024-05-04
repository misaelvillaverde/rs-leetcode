// 684

#![allow(dead_code)]

use std::collections::HashMap;

struct RedundantConnectionDFS {}

impl RedundantConnectionDFS {
    fn dfs(
        node: i32,
        target: i32,
        graph: &HashMap<i32, Vec<i32>>,
        visited: &mut Vec<bool>,
    ) -> bool {
        if visited[node as usize - 1] {
            return false;
        }

        if node == target {
            return true;
        }

        visited[node as usize - 1] = true;

        if let Some(neighbors) = graph.get(&node) {
            if neighbors
                .iter()
                .any(|&neighbor| Self::dfs(neighbor, target, graph, visited))
            {
                return true;
            }
        }

        false
    }

    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for edge in &edges {
            let u = edge[0];
            let v = edge[1];

            if graph.contains_key(&u)
                && graph.contains_key(&v)
                && Self::dfs(u, v, &graph, &mut vec![false; edges.len()])
            {
                return vec![u, v];
            }

            graph.entry(u).and_modify(|n| n.push(v)).or_insert(vec![v]);
            graph.entry(v).and_modify(|n| n.push(u)).or_insert(vec![u]);
        }

        vec![]
    }
}

struct RedundantConnectionUnionFind {}

impl RedundantConnectionUnionFind {
    fn find_root(node: i32, parents: &mut HashMap<i32, i32>) -> i32 {
        let mut parent: i32;
        let mut curr = node;

        loop {
            parent = *parents
                .get(&curr)
                .expect(format!("could not get parent node of {}", curr).as_str());

            // Path compression
            parent = *parents
                .get(&parent)
                .expect(format!("could not get parent node of {}", node).as_str());
            parents.entry(curr).and_modify(|n| *n = parent);

            if parent == curr {
                break;
            }

            curr = parent;
        }

        curr
    }

    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parents = HashMap::new();
        let mut level = HashMap::new();

        for edge in &edges {
            for &node in edge {
                parents.insert(node, node);
                level.insert(node, 0);
            }
        }

        for edge in &edges {
            let root1 = Self::find_root(edge[0], &mut parents);
            let root2 = Self::find_root(edge[1], &mut parents);

            if root1 == root2 {
                return edge.clone();
            }

            let lvl1 = level.get(&root1).expect("level 1 not found");
            let lvl2 = level.get(&root2).expect("level 2 not found");

            if lvl1 > lvl2 {
                parents.entry(root2).and_modify(|n| *n = root1);
                continue;
            }

            if lvl1 < lvl2 {
                parents.entry(root1).and_modify(|n| *n = root2);
                continue;
            }

            parents.entry(root2).and_modify(|n| *n = root1);
            level.entry(root1).and_modify(|n| *n += 1);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::redundant_connection::{RedundantConnectionDFS, RedundantConnectionUnionFind};

    #[test]
    fn dfs1() {
        let edges = vec![vec![1, 2], vec![3, 4], vec![2, 3], vec![1, 4], vec![1, 5]];

        let redundant = RedundantConnectionDFS::find_redundant_connection(edges);

        assert_eq!(redundant, vec![1, 4]);
    }

    #[test]
    fn union_find1() {
        let edges = vec![vec![1, 2], vec![3, 4], vec![2, 3], vec![1, 4], vec![1, 5]];

        let redundant = RedundantConnectionUnionFind::find_redundant_connection(edges);

        assert_eq!(redundant, vec![1, 4]);
    }
}
