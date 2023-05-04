#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
      
        if self.v == 0 || self.v == 1 {
            return None;
        }

        if self.v % 2 == 0 {
            self.v /= 2;
            Some(Collatz::new(self.v * 2))
        } else {
            self.v = 3 * self.v + 1;
            Some(Collatz::new((self.v - 1) / 3))
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}