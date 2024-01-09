use crate::ingredients::{PizzaType, Dough};

pub struct Manual {
    pizza_type: PizzaType,
    dough: Dough,
    topping_amount: u16,
}

impl Manual {
    pub fn new(
        pizza_type: PizzaType,
        dough: Dough,
        topping_amount: u16,
    ) -> Self {
        Self {
            pizza_type,
            dough,
            topping_amount,
        }
    }
}

impl std::fmt::Display for Manual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Type of pizza: {:?}", self.pizza_type)?;
        writeln!(f, "Count of toppings: {}", self.topping_amount)?;
        writeln!(
            f,
            "Dough volume - {}; oat - {},", 
            self.dough.volume(),
            self.dough.oat()
        )?;
        Ok(())
    }
}