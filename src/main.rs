#![allow(unused)]

mod director;
mod builders;
mod pizzas;
mod ingredients;

use builders::{Builder, PizzaBuilder, PizzaManualBuilder};
use pizzas::{Pizza,Manual};
use director::Director;

fn main() {
    let mut pizza_builder = PizzaBuilder::default();

    // Director gets the concrete builder object from the client
    // (application code). That's because application knows better which
    // builder to use to get a specific product.
    Director::construct_sports_car(&mut pizza_builder);

    let pizza: Pizza = pizza_builder.build();
    println!("Pizza made: {:?}",pizza.pizza_type());

    let mut manual_builder = PizzaManualBuilder::default();

    //With the Director
    Director::construct_hawaii_pizza(&mut manual_builder);
    let manual: Manual = manual_builder.build();
    println!("Pizza manual made: {}", manual);
}