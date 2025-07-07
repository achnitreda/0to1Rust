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
        Cart{items:vec![],receipt:vec![]}
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for i in &s.products {
            if i.0 == ele {
                self.items.push((ele.clone(),i.1));
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        
        if prices.len() < 3 {
            let mut res = prices;
            res.sort_by(|a, b| a.partial_cmp(b).unwrap());
            self.receipt = res.clone();
            return res;
        }
        
        let mut sorted_prices = prices.clone();
        sorted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let free_items_count = prices.len() / 3;
        
        let total_discount: f32 = sorted_prices.iter().take(free_items_count).sum();
        
        let original_total: f32 = prices.iter().sum();
        
        let target_total = original_total - total_discount;
        
        let new_prices: Vec<f32> = prices.iter()
            .map(|price| {
                let proportional_price = (price / original_total) * target_total;
                (proportional_price * 100.0).round() / 100.0
            })
            .collect();
        
        let mut res = new_prices;
        res.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        self.receipt = res.clone();
        res
    }
}