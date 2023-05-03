#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last()
        // if self.numbers.is_empty() {
        //     return None;
        // } else {
        //     let x = self.numbers.last().unwrap();
        //     Some(*x)
        // }
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut num = self.numbers.to_vec();
        num.sort_by(|a, b| b.cmp(a));
        let result = num.iter().take(3).copied().collect();
        result
    }
}
