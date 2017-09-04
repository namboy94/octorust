/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

// Imports
use octo_types;
use octo_agent;

/// Struct modelling the constraints for a claim. Makes use of a constraints_t struct
/// to do so.
///
/// # Members
///
/// * `constraints` - The constraints_t struct that this struct is wrapped around
pub struct Constraints {
    constraints: octo_types::constraints_t
}


/// Implements the methods for the Constraints Struct
impl Constraints {

    /// Constructor for the Constraints struct. Creates a new constraints_t
    /// using the `agent_constr_create` function. Sets a default minimum/maximum amount
    /// of processing elements and sets the tile as shareable
    ///
    /// # Arguments
    ///
    /// * `min_pes` - The minimum amount of processing elements
    /// * `max_pes` - The maximum amount of processing elements
    pub fn new(min_pes: i32, max_pes: i32) -> Constraints {
        let constraints = octo_agent::agent_constr_create();

        // Set defaults
        octo_agent::agent_constr_set_quantity(constraints, min_pes, max_pes, 0);
        octo_agent::agent_constr_set_tile_shareable(constraints, 1);
        Constraints {constraints: constraints}
    }

    /// Merges another Constraints struct with this one
    ///
    /// # Arguments
    ///
    /// * `to_merge` - The Constraints struct to merge with this Constraints struct
    pub fn merge_constraints(&self, to_merge: Constraints) {
        octo_agent::agent_constr_overwrite(self.constraints, to_merge.to_constraints_t());
    }

    /// Sets the amount of processing elements of this constraint
    ///
    /// # Arguments
    ///
    /// * `min_pes` - The minimum amount of processing elements
    /// * `max_pes` - The maximum amount of processing elements
    pub fn set_pe_quantity(&self, min_pes: i32, max_pes: i32) {
        octo_agent::agent_constr_set_quantity(self.constraints, min_pes, max_pes, 0);
    }

    // Thin wrappers around agent_constr_set... functions
    pub fn set_downey_speedup_curve(&self, a: i32, sigma: i32) {
        octo_agent::agent_constr_set_downey_speedup_curve(self.constraints, a, sigma);
    }
    pub fn set_tile(&self, tile_id: octo_types::tile_id_t) {
        octo_agent::agent_constr_set_tile(self.constraints, tile_id);
    }
    pub fn set_notontile(&self, tile_id: octo_types::tile_id_t) {
        octo_agent::agent_constr_set_notontile(self.constraints, tile_id);
    }
    pub fn set_tile_bitmap(&self, bitmap: u16) {
        octo_agent::agent_constr_set_tile_bitmap(self.constraints, bitmap);
    }
    pub fn set_stickyclaim(&self, sticky: bool) {
        octo_agent::agent_constr_set_stickyclaim(self.constraints, sticky as u8);
    }
    pub fn set_vipg(&self, enable_vipg: bool) {
        octo_agent::agent_constr_set_vipg(self.constraints, enable_vipg as u8);
    }
    pub fn set_appclass(&self, app_class: i32) {
        octo_agent::agent_constr_set_appclass(self.constraints, app_class);
    }
    pub fn agent_constr_set_appnumber(&self, app_nr: i32) {
        octo_agent::agent_constr_set_appnumber(self.constraints, app_nr);
    }
    pub fn agent_constr_set_malleable(&self, malleable: bool,
                                      resize_handler: octo_types::resize_handler_t,
                                      resize_env: octo_types::resize_env_t) {
        octo_agent::agent_constr_set_malleable(self.constraints, malleable as u8, resize_handler, resize_env);
    }
    pub fn agent_constr_set_reinvade_handler(&self,
                                             reinvade_handler: octo_types::reinvade_handler_t) {
        octo_agent::agent_constr_set_reinvade_handler(self.constraints, reinvade_handler);
    }
    pub fn agent_constr_get_reinvade_handler(&self) -> octo_types::reinvade_handler_t {
        return octo_agent::agent_constr_get_reinvade_handler(self.constraints);
    }

    /// Sets the shareability of the tile
    ///
    /// # Arguments
    ///
    /// * `shareable` - Specifies if the tile is shareable or not
    pub fn set_tile_shareable(&self, shareable: bool) {
        let is_shareable = if shareable {0} else {1};
        octo_agent::agent_constr_set_tile_shareable(self.constraints, is_shareable);
    }

    /// # Return Value
    ///
    /// The inner constraints_t struct
    pub fn to_constraints_t(&self) -> octo_types::constraints_t {
        return self.constraints;
    }
}
