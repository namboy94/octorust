/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

// Imports
use octo_types::constraints_t;
use octo_agent::{agent_constr_create, agent_constr_set_quantity, agent_constr_set_tile_shareable};

/// Struct modelling the constraints for a claim. Makes use of a constraints_t struct
/// to do so.
///
/// # Members
///
/// * `constraints` - The constraints_t struct that this struct is wrapped around
pub struct Constraints {
    constraints: constraints_t
}


/// Implements the methods for the Constraints Struct
impl Constraints {

    /// Constructor for the Constraints struct. Creates a new constraints_t
    /// using the `agent_constr_create` function
    pub fn new() -> Constraints {
        let stressConstraints = agent_constr_create();
        Constraints {constraints: stressConstraints}
    }

    /// Sets the amount of processing elements of this constraint
    ///
    /// # Arguments
    ///
    /// * `min_pes` - The minimum amount of processing elements
    /// * `max_pes` - The maximum amount of processing elements
    pub fn set_pe_quantity(&mut self, min_pes: i32, max_pes: i32) {
        agent_constr_set_quantity(self.constraints, min_pes, max_pes, 0);
    }

    /// Sets the shareability of the tile
    ///
    /// # Arguments
    ///
    /// * `shareable` - Specifies if the tile is shareable or not
    pub fn set_tile_shareable(&mut self, shareable: bool) {
        let is_shareable = if shareable {0} else {1};
        agent_constr_set_tile_shareable(self.constraints, is_shareable);
    }

    /// # Return Value
    ///
    /// The inner constraints_t struct
    pub fn to_constraints_t(&self) -> constraints_t {
        return self.constraints;
    }
}