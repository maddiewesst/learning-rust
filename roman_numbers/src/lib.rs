
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
                    //  vec.push(str);
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