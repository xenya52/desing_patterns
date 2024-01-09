use crate::{
    pizza::Manual,
    ingredients::{PizzaType, Dough},
};

use super::Builder;

#[derive(Default)]
pub struct PizzaManualBuilder {
    pizza_type: Option<PizzaType>,
    dough: Option<Dough>,
    topping_amount: Option<u16>,
}
impl Builder for PizzaManualBuilder {
    type OutputType = Pizza;

    fn set_pizza_type(&mut self, _pizza_type: PizzaType) {
        self.pizza_type = Some(_pizza_type);
    }
    fn set_dough(&mut self, _dough: Dough) {
        self.dough = Some(self.dough);
    }
    fn build(self) -> Pizza {
        Manual::new(
            self.pizza_type.expect("Set a pizza type!"),
            self.dough.expect("Set a dough!"),
            self.topping_amount.expect("Set the total amounts of toppings!"),
        )
    }
}