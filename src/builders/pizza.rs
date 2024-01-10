use crate{
    pizza::Pizza,
    ingredients::{PizzaType,Dough,self}
};

use super::Builder;

pub const DEFAULT_TOMATO_SAUCE: u32 = 5u32;

#[derive(Default)]
pub struct PizzaBuilder {
    pizza_type: Option<PizzaType>,
    dough: Option<Dough>,
    topping_amount: u16,
}

impl Builder for PizzaBuilder {
    type OutputType = Pizza;

    fn set_pizza_type(&mut self, _pizza_type: PizzaType) {
        self.pizza_type = Some(_pizza_type);
    }
    fn set_dough(&mut self, _dough: Dough) {
        self.dough = Some(self.dough);
    }
    fn set_topping_amount(&mut self, _topping_amount: u16) {
        self.topping_amount = _topping_amount;
    }
    fn build(self) -> Pizza {
        Pizza::new(
            self.pizza_type.expect("Set a pizza type!"),
            self.dough.expect("Set a dough!"),
            self.topping_amount.expect("Set the total amounts of toppings!"),
            DEFAULT_TOMATO_SAUCE,
        )
    }
}