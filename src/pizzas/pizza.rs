use crate::ingredients::{PizzaType, Dough, Cheese};

pub struct Pizza {
    pizza_type: PizzaType,
    dough: Dough,
    cheese: Cheese,
    topping_amount: u16,
}

impl Pizza {
    pub fn new(
        pizza_type: PizzaType,
        dough: Dough,
        cheese: Cheese,
        topping_amount: u16,
        tomato_sauce: u32,
    ) -> Self {
        Self {
            pizza_type,
            dough,
            cheese,
            topping_amount,
        }
    }
}

pub fn pizza_type(&self) -> PizzaType {
    self.pizza_type
}
pub fn dough(&self) -> Dough {
    self.dough
}
pub fn cheese(&self) -> Cheese {
    self.cheese
}

pub fn topping_amount(&self, _topping_amount: u16) -> u16 {
    self.topping_amount = _topping_amount;
}

pub fn tomato_sauce(&self, _tomato_sauce: u32) -> u32 {
    self.tomato_sauce = _tomato_sauce;
}