use std::string;

#[derive(Copy, Clone, Debug)]
pub enum PizzaType {
    PizzaHawaii,
    PizzaVeggi,
    PizzaSalami,
}

pub struct Dough {
    volume: f64,
    oat: f64,
}

impl Dough {
    pub fn new(volume: f64, oat: f64) -> Self {
        Self {
            volume,
            oat,
        }
    }
    pub fn volume(&self) -> f64 {
        self.volume
    }
    pub fn oat(&self) -> f64 {
        self.oat
    }
} 