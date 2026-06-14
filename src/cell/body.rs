use std::collections::HashMap;

use crate::{
    cell::{chemical::Chemical, cmd::CommandBody},
    environment::Environment,
};

pub type Amount = f32;

#[derive(Debug, Clone)]
pub struct Body {
    energy: f32,
    chemicals: HashMap<Chemical, Amount>,
}

impl Body {
    pub fn new() -> Self {
        Self {
            energy: 0.0,
            chemicals: HashMap::new(),
        }
    }

    pub fn execute_cmd(&mut self, cmd: &CommandBody, environment: &Environment, dt: f32) {
        match cmd {
            CommandBody::Glycolysis => self.glycolysis(dt),
            CommandBody::Photosynthesis => self.photosynthesize(environment.light, dt),
        }
    }

    fn glycolysis(&mut self, dt: f32) {
        if let Some(amount) = self.chemicals.get_mut(&Chemical::Glucose) {
            let value = amount.min(1.0);
            let energy = 32.0 * value * dt;
            self.energy += energy;
            *amount -= value;
        }
    }

    fn photosynthesize(&mut self, light: f32, dt: f32) {
        let amount_light = light.min(5.0);
        let value = amount_light / 5.0 * dt;
        self.add_chemical(Chemical::Glucose, value);
    }

    pub fn add_chemical(&mut self, chemical: Chemical, amount: Amount) {
        match self.chemicals.get_mut(&chemical) {
            Some(amount_chamical) => *amount_chamical += amount,
            None => {
                self.chemicals.insert(chemical, amount);
            }
        }
    }
}
