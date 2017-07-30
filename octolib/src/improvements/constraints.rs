/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

// Imports
use octo_types::{constraints_t, tile_id_t, reinvade_handler_t, resize_handler_t, resize_env_t};
use octo_agent::{agent_constr_create, agent_constr_set_quantity, agent_constr_set_tile_shareable,
                 agent_constr_delete, agent_constr_overwrite, agent_constr_set_downey_speedup_curve,
                 agent_constr_set_tile, agent_constr_set_notontile, agent_constr_set_tile_bitmap,
                 agent_constr_set_stickyclaim, agent_constr_set_vipg, agent_constr_set_appclass,
                 agent_constr_set_appnumber, agent_constr_set_malleable,
                 agent_constr_get_reinvade_handler, agent_constr_set_reinvade_handler};

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

    /// Merges another Constraints struct with this one
    ///
    /// # Arguments
    ///
    /// * `to_merge` - The Constraints struct to merge with this Constraints struct
    pub fn merge_constraints(&mut self, to_merge: Constraints) {
        agent_constr_overwrite(self.constraints, to_merge.to_constraints_t());
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

    // Thin wrappers around agent_constr_set... functions

    pub fn set_downey_speedup_curve(&mut self, a: i32, sigma: i32) {
        agent_constr_set_downey_speedup_curve(self.constraints, a, sigma);
    }
    pub fn set_tile(&mut self, tile_id: tile_id_t) {
        agent_constr_set_tile(self.constraints, tile_id);
    }
    pub fn set_notontile(&mut self, tile_id: tile_id_t) {
        agent_constr_set_notontile(self.constraints, tile_id);
    }
    pub fn set_tile_bitmap(&mut self, bitmap: u16) {
        agent_constr_set_tile_bitmap(self.constraints, bitmap);
    }
    pub fn set_stickyclaim(&mut self, sticky: bool) {
        agent_constr_set_stickyclaim(self.constraints, sticky as u8);
    }
    pub fn set_vipg(&mut self, enable_vipg: bool) {
        agent_constr_set_vipg(self.constraints, enable_vipg as u8);
    }
    pub fn set_appclass(&mut self, app_class: i32) {
        agent_constr_set_appclass(self.constraints, app_class);
    }
    pub fn agent_constr_set_appnumber(&mut self, app_nr: i32) {
        agent_constr_set_appnumber(self.constraints, app_nr);
    }
    pub fn agent_constr_set_malleable(&mut self, malleable: bool,
                                      resize_handler: resize_handler_t, resize_env: resize_env_t) {
        agent_constr_set_malleable(self.constraints, malleable as u8, resize_handler, resize_env);
    }
    pub fn agent_constr_set_reinvade_handler(&mut self, reinvade_handler: reinvade_handler_t) {
        agent_constr_set_reinvade_handler(self.constraints, reinvade_handler);
    }
    pub fn agent_constr_get_reinvade_handler(&mut self) -> reinvade_handler_t {
        return agent_constr_get_reinvade_handler(self.constraints);
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

/// Implements the Drop trait for the constraints. Defines a destructor that
/// automatically deletes the constraints
impl Drop for Constraints {

    /// Destructor for the Constraints struct. Uses agent_constr_create to delete the
    /// internat constraints container to avoid memory leaks
    fn drop(&mut self) {
        agent_constr_delete(self.constraints);
    }

}