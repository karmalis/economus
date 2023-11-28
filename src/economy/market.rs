

pub struct Market {
    price: u32,
    velocity: i32,
    demand: u32,
    supply: u32,
}

impl Market {

    pub fn new(price: u32) -> Self {
        Self {
            price,
            velocity: 0,
            demand: 0,
            supply: 0,
        }
    }

    pub fn open(&mut self) {
        self.demand = 0;
        self.supply = 0;
    }

}


