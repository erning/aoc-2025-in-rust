fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<i64> =
                line.split(',').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        })
        .collect()
}

fn distance_squared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return false;
        }

        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
            self.size[py] += self.size[px];
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
            self.size[px] += self.size[py];
        } else {
            self.parent[py] = px;
            self.size[px] += self.size[py];
            self.rank[px] += 1;
        }
        true
    }

    fn get_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = Vec::new();
        for i in 0..n {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

fn solve(input: &str, connections: usize) -> u64 {
    let points = parse_input(input);
    let n = points.len();

    // Calculate all pairwise distances
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_squared(&points[i], &points[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort by distance
    edges.sort_by_key(|e| e.0);

    // Connect the closest pairs
    let mut uf = UnionFind::new(n);
    let mut connected = 0;

    for (_, i, j) in edges {
        uf.union(i, j);
        connected += 1;
        if connected >= connections {
            break;
        }
    }

    // Get circuit sizes and multiply top 3
    let mut sizes = uf.get_sizes();
    sizes.sort_by(|a, b| b.cmp(a));

    sizes.iter().take(3).map(|&s| s as u64).product()
}

pub fn part_one(input: &str) -> u64 {
    solve(input, 1000)
}

pub fn part_two(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(8);
        assert_eq!(solve(&input, 10), 40);
    }
}
