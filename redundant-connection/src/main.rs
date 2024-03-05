fn main() {
    println!("Hello, world!");
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    // Output: [2,3]
    // let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
    // Output: [1,4]
    println!("{:?}", find_redundant_connection(edges))
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut dsu = Dsu::new(edges.len() + 1);
    for edge in edges {
        println!("dsu={:?}", dsu);
        if !dsu.union_set(edge[0] as usize, edge[1] as usize) {
            return edge;
        }
    }
    unreachable!()
}

#[derive(Debug)]
struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
    forests: usize,
}

impl Dsu {
    fn new(forests: usize) -> Self {
        let mut parent = vec![0; forests];
        let mut rank = vec![0; forests];

        for i in 0..forests {
            parent[i] = i;
            rank[i] = i;
        }

        Self {
            parent,
            rank,
            forests,
        }
    }

    fn find_parent(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.parent[x] = self.find_parent(self.parent[x]);
        self.parent[x]
    }

    fn union_set(&mut self, x: usize, y: usize) -> bool {
        let (x, y) = (self.find_parent(x), self.find_parent(y));

        if x == y {
            return false;
        }

        Dsu::link(self, x, y);
        self.forests -= 1;

        // skip for now
        // self.parent[x] != self.parent[y]
        true
    }

    fn link(&mut self, x: usize, y: usize) {
        if self.rank[x] > self.rank[y] {
            self.parent[y] = x;
        } else if self.rank[x] < self.rank[y] {
            self.parent[x] = y
        } else {
            self.rank[y] += 1;
        }
    }
}
