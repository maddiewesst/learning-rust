
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(_: u32) -> Self {
        RomanDigit::X
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        let mut number = n.clone();
        let mut vec: Vec<RomanDigit> = Vec::new();
        let conversions = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        for (i, str) in conversions {
            while number >= i {
                for ch in str.chars() {
                    let el = match ch {
                        'M' => RomanDigit::M,
                        'D' => RomanDigit::D,
                        'C' => RomanDigit::C,
                        'L' => RomanDigit::L,
                        'X' => RomanDigit::X,
                        'V' => RomanDigit::V,
                        'I' => RomanDigit::I,
                        _ => RomanDigit::Nulla,
                    };
                    vec.push(el);
                }
                number -= i;
            }
        }

        RomanNumber { 0: vec }
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vec = self.clone();

       
        if self.0.len() == 1 && self.0[0] == RomanDigit::I {
            return Some(RomanNumber(vec![]))
        }
        if let Some(&last_digit) = self.0.last() {
            let el = match last_digit {
                RomanDigit::M => RomanDigit::D,
                RomanDigit::D => RomanDigit::C,
                RomanDigit::C => RomanDigit::L,
                RomanDigit::L => RomanDigit::X,
                RomanDigit::X => RomanDigit::V,
                RomanDigit::V => RomanDigit::I,
                RomanDigit::I => return Some(self.clone()),
                _ => return None,
            };
            vec.0.push(el);
            Some(vec)
        } else {
            return Some(RomanNumber(vec![]))
        }
    }
    
    
}
