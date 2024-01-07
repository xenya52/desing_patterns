mod car;
mod car_manual;

use crate::ingredients::{PizzaType,Dough,Cheese};

//How to assemble a carGibts
pub trait {
    Builder {
        type OutputType;
        fn set_pizza_type(&mut self, pizza_type: PizzaType);
        fn set_topping_amount(&mut self, toppings: u16);
        fn set_dough(&mut self, dough: Dough);
        fn set_cheese(&mut self, cheese: Cheese);
        fn build(self) -> Self::OutputType;
    }
}

pub use pizza: PizzaBuilder;
pub use pizza_manual::PizzaManualBuilder;