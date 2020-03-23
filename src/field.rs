use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub w: usize,
    pub h: usize,
    pub data: Vec<Vec<u8>>,
}

impl Field {
    //pub fn new(w: usize, h: usize) -> Self {
    //    assert! { h > 4 && w > 4 };
    //    let mut data = vec![];
    //    for _y in 0..h {
    //        data.push((0..w).map(|_| 0x20u8).collect::<Vec<u8>>());
    //    }
    //    Self { data, w, h }
    //}

    pub fn coords(&self) -> itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>> {
        return (0..self.w).cartesian_product(0..self.h);
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        assert!(x < self.w);
        assert!(y < self.h);
        self.data[y][x] = val;
    }

    pub fn get(&self, x: isize, y: isize) -> u8 {
        if x < 0 || y < 0 {
            return 0x20;
        }
        *self
            .data
            .get(y as usize)
            .and_then(|w| w.get(x as usize))
            .unwrap_or(&0x20)
    }

    pub fn neighborhood(&self, x: usize, y: usize) -> Vec<u8> {
        let mut res = vec![];
        for yo in -1..=1 {
            for xo in -1..=1 {
                res.push(self.get((x as isize) + xo, (y as isize) + yo));
            }
        }
        return res;
    }
}
