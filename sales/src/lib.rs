#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|p| p.0 == ele) {
            self.items.push(product.clone());
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let promo_size = self.items.len() / 3;
        let mut prices: Vec<f32> = self.items.iter().map(|p| p.1).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let old_sum: f32 = prices.iter().sum();
        let new_prices: Vec<f32> = prices.iter().skip(promo_size).cloned().collect();
        let new_sum = new_prices.iter().sum::<f32>();
        let n = 1.0 - (old_sum - new_sum) / old_sum;

        let new_vec: Vec<f32> = prices.iter().map(|x| x * n).collect();

        let result = new_vec.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<f32>>();

        self.receipt = result.clone();
    
        result
    
    }
}



