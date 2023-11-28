pub mod enterprise;
pub mod gdp;
pub mod market;
pub mod order;

pub type Credit = u32;

pub trait TradeItem {
    fn name(&self) -> String;
    fn price(&self, modifiers: Vec<f32>) -> u32;
}

pub struct Good {
    pub name: String,
    pub base_price: u32,
}

impl TradeItem for Good {
    fn name(&self) -> String{
        self.name.clone()
    }

    fn price(&self, modifiers: Vec<f32>) -> u32 {
        self.base_price + modifiers
            .iter()
            .map(|modifier| self.base_price as f32 * modifier)
            .sum::<f32>() as u32            
    }
}

impl Good {
    pub fn new(name: String, base_price: u32) -> Self {
        Self {
            name,
            base_price,
        }
    }
}

