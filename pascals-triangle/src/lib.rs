pub struct PascalsTriangle {
    tri: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        fn ncr(n: u32, k: u32) -> u32 {
            assert!(k <= n);
            if k == 0 || n == k {
                return 1u32;
            }
            ncr(n - 1, k - 1) + ncr(n - 1, k)
        }

        let mut top = Vec::new();
        if row_count == 1 {
            let temp: Vec<u32> = vec![1];
            top.push(temp);
            return PascalsTriangle { tri: top };
        }

        for n in 0..row_count {
            let temp = (0..n + 1).map(|k| ncr(n, k)).collect::<Vec<u32>>();
            top.push(temp);
        }

        PascalsTriangle { tri: top }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.tri.clone()
    }
}
