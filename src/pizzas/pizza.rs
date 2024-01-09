use crate::ingredients::{PizzaType, Dough};

pub struct Pizza {
    pizza_type: PizzaType,
    dough: Dough,
    topping_amount: u16,
}

impl Pizza {
    pub fn new(
        pizza_type: PizzaType,
        dough: Dough,
        topping_amount: u16,
        tomato_sauce: u32,
    ) -> Self {
        Self {
            pizza_type,
            dough,
            topping_amount,
        }
    }
    pub fn pizza_type(&self) -> PizzaType {
        self.pizza_type
    }
    
    pub fn dough(&self) -> Dough {
        self.dough
    }
    
    pub fn topping_amount(&self) -> u16 {
        self.topping_amount
    }
}