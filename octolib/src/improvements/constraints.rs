use octo_types::constraints_t;
use octo_agent::{agent_constr_create};

pub struct Constraint {
    constraints: constraints_t
}

impl Constraint {
    pub fn new() {
        let stressConstraints = agent_constr_create();
        Constraint {constraints: stressConstraints};
    }

    pub fn to_contraints_t(&self) -> constraints_t {
        return self.constraints;
    }
}