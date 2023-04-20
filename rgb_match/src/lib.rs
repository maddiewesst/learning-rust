#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut col = self.clone();
        if self.r == first {col.r = second}
        if self.g == first {col.g = second}
        if self.b == first {col.b = second}
        if self.a == first {col.a = second}
        if self.r == second {col.r = first}
        if self.g == second {col.g = first}
        if self.b == second {col.b = first}
        if self.a == second {col.a = first}
        return col
    
    }
}