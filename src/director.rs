use crate::{
    builder::Builder,
    ingredients::{PizzaType,Dough}
};

pub struct Director;

impl Director {
    pub fn construct_hawaii_pizza(builder: &mut impl Builder) {
        build.set_pizza_type(PizzaType::PizzaHawaii);
        build.set_topping_amount(5);
    }
}