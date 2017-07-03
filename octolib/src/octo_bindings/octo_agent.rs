use octo_bindings::octo_types::*;

extern {
    #[link_name="agent_stresstest_agentclaim"]
    fn __agent_stresstest_agentclaim();

    #[link_name="agent_stresstest_agentoctoclaim"]
    fn __agent_stresstest_agentoctoclaim();

    #[link_name="agent_constr_create"]
    fn __agent_constr_create() -> constraints_t;
}

/// Testing function only! Do not use! Undocumented for a good reason.
pub fn agent_stresstest_agentclaim() { unsafe { __agent_stresstest_agentclaim() } }

/// Testing function only! Do not use! Undocumented for a good reason.
pub fn agent_stresstest_agentoctoclaim() { unsafe { __agent_stresstest_agentoctoclaim() } }

/// Creates a new set of constraints.
///
/// All constraints are always ANDed together. To support ORed constraints, multiple sets of
/// constraints have to be created! This function initializes to default values.
///
/// # Note
///
/// The constraints handle is only valid on the tile it has been created. (It is local pointer!)
///
/// # Return Value
///
/// Constraints handle which is required for invading resources
/// and actually changing the constraints.
pub fn agent_constr_create() -> constraints_t { unsafe { __agent_constr_create() } }