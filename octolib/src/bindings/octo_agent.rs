/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_agent.h file

use octo_types::*;

extern {
    #[link_name="agent_agent_create"]
    fn __agent_agent_create() -> agent_t;

    #[link_name="agent_agent_delete"]
    fn __agent_agent_delete(ag: agent_t, force: u8);

    #[link_name="agent_claim_get_agent"]
    fn __agent_claim_get_agent(claim: agentclaim_t) -> agent_t;

    #[link_name="agent_claim_get_constr"]
    fn __agent_claim_get_constr(claim: agentclaim_t) -> constraints_t;

    #[link_name="agent_claim_invade"]
    fn __agent_claim_invade(parentagent: agent_t, constr: constraints_t) -> agentclaim_t;

    #[link_name="agent_claim_invade_with_name"]
    fn __agent_claim_invade_with_name(parentagent: agent_t, constr: constraints_t, agent_name: *const c_char) -> agentclaim_t;

    #[link_name="set_agent_name"]
    fn __set_agent_name(claim: agentclaim_t, agent_name: *const c_char);

    #[link_name="get_agent_name"]
    fn __get_agent_name(claim: agentclaim_t, buffer: *mut c_char, size: usize) -> i32;

    #[link_name="agent_get_downey_sigma"]
    fn __agent_get_downey_sigma(constr: constraints_t) -> i32;

    #[link_name="agent_claim_invade_or_constraints"]
    fn __agent_claim_invade_or_constraints(parentagent: agent_t, constr_count: u8, constr: *mut constraints_t) -> agentclaim_t;

    #[link_name="agent_claim_invade_parentclaim"]
    fn __agent_claim_invade_parentclaim(parentclaim: agentclaim_t, constr: constraints_t) -> agentclaim_t;

    #[link_name="agent_claim_get_initial"]
    fn __agent_claim_get_initial(octoclaim: claim_t) -> agentclaim_t;

    #[link_name="agent_claim_get_initial_with_name"]
    fn __agent_claim_get_initial_with_name(octoclaim: claim_t, agent_name: *const c_char) -> agentclaim_t;

    #[link_name="agent_claim_retreat"]
    fn __agent_claim_retreat(claim: agentclaim_t);

    #[link_name="agent_claim_reinvade"]
    fn __agent_claim_reinvade(claim: agentclaim_t) -> i32;

    #[link_name="agent_claim_reinvade_constraints"]
    fn __agent_claim_reinvade_constraints(claim: agentclaim_t, constr: constraints_t) -> i32;

    #[link_name="agent_claim_print"]
    fn __agent_claim_print(claim: agentclaim_t);

    #[link_name="agent_claim_get_pecount"]
    fn __agent_claim_get_pecount(claim: agentclaim_t) -> i32;

    #[link_name="agent_claim_get_pecount_type"]
    fn __agent_claim_get_pecount_type(claim: agentclaim_t, _type: res_type_t) -> i32;

    #[link_name="agent_claim_get_pecount_tile"]
    fn __agent_claim_get_pecount_tile(claim: agentclaim_t, tileID: tile_id_t) -> i32;

    #[link_name="agent_claim_get_pecount_tile_type"]
    fn __agent_claim_get_pecount_tile_type(claim: agentclaim_t, tileID: tile_id_t, _type: res_type_t) -> i32;

    #[link_name="agent_claim_get_proxyclaim_tile_type"]
    fn __agent_claim_get_proxyclaim_tile_type(claim: agentclaim_t, tileID: i32, _type: res_type_t) -> proxy_claim_t;

    #[link_name="agent_claim_get_tilecount"]
    fn __agent_claim_get_tilecount(claim: agentclaim_t) -> i32;

    #[link_name="agent_claim_get_tileid_iterative"]
    fn __agent_claim_get_tileid_iterative(claim: agentclaim_t, iterator: i32) -> i32;

    #[link_name="agent_constr_create"]
    fn __agent_constr_create() -> constraints_t;

    #[link_name="agent_constr_delete"]
    fn __agent_constr_delete(constr: constraints_t);

    #[link_name="agent_constr_overwrite"]
    fn __agent_constr_overwrite(constrTarget: constraints_t, additionalConstraints: constraints_t);

    #[link_name="agent_constr_set_quantity"]
    fn __agent_constr_set_quantity(constr: constraints_t, minPEs: i32, maxPEs: i32, _type: res_type_t);

    #[link_name="agent_constr_set_downey_speedup_curve"]
    fn __agent_constr_set_downey_speedup_curve(constr: constraints_t, A: i32, sigma: i32);

    #[link_name="agent_constr_set_tile"]
    fn __agent_constr_set_tile(constr: constraints_t, TileID: tile_id_t);

    #[link_name="agent_constr_set_notontile"]
    fn __agent_constr_set_notontile(constr: constraints_t, TileID: tile_id_t);

    #[link_name="agent_constr_set_tile_bitmap"]
    fn __agent_constr_set_tile_bitmap(constr: constraints_t, bitmap: u16);

    #[link_name="agent_constr_set_stickyclaim"]
    fn __agent_constr_set_stickyclaim(constr: constraints_t, sticky_boolean: u8);

    #[link_name="agent_constr_set_vipg"]
    fn __agent_constr_set_vipg(constr: constraints_t, vipgEnable: u8);

    #[link_name="agent_constr_set_appclass"]
    fn __agent_constr_set_appclass(constr: constraints_t, AppClass: i32);

    #[link_name="agent_constr_set_appnumber"]
    fn __agent_constr_set_appnumber(constr: constraints_t, AppNr: i32);

    #[link_name="agent_constr_set_tile_shareable"]
    fn __agent_constr_set_tile_shareable(constr: constraints_t, is_tile_shareable: u8);

    #[link_name="agent_constr_set_malleable"]
    fn __agent_constr_set_malleable(constr: constraints_t, malleable_boolean: u8, resize_handler: resize_handler_t, resize_env: resize_env_t);

    #[link_name="agent_stresstest_agentclaim"]
    fn __agent_stresstest_agentclaim();

    #[link_name="agent_stresstest_agentoctoclaim"]
    fn __agent_stresstest_agentoctoclaim();

    #[link_name="agent_constr_set_reinvade_handler"]
    fn __agent_constr_set_reinvade_handler(constr: constraints_t, reinvade_handler: reinvade_handler_t);

    #[link_name="agent_constr_get_reinvade_handler"]
    fn __agent_constr_get_reinvade_handler(constr: constraints_t) -> reinvade_handler_t;

    #[link_name="agent_proxy_get_proxyagentoctoclaim"]
    fn __agent_proxy_get_proxyagentoctoclaim(objects_tile: i32, octo_ucid: u32) -> agentclaim_t;

    #[link_name="agent_proxy_delete_proxyagentoctoclaim"]
    fn __agent_proxy_delete_proxyagentoctoclaim(proxy_agentoctoclaim: agentclaim_t);

    #[link_name="agent_proxy_get_objectstile"]
    fn __agent_proxy_get_objectstile(proxy_agentoctoclaim: agentclaim_t) -> i32;

    #[link_name="agent_claim_get_ucid"]
    fn __agent_claim_get_ucid(claim: agentclaim_t) -> u32;

    #[link_name="agent_claim_isempty"]
    fn __agent_claim_isempty(claim: agentclaim_t) -> bool;

}

/// octo_agent.h
/// Functions to create and interact with Agents, Constraints and "high level" Claims.

/// Creates a new Agent.
/// This creates a new Agent on the "Agent Tile". Please note that new Agents are automatically
/// created when using agent_claim_invade without specifying a parentclaim. (Consequently, you
/// should never have to use this function for now.)
///
/// # Return Value
///
/// AgentInstance handle which is required for all subsequent communications with that agent.
///
pub fn agent_agent_create() -> agent_t {
    unsafe {
        __agent_agent_create()
    }
}

/// Deletes a previously created agent. Automatically done when using agent_claim_retreat!
/// If force is non-zero, all active claims will be retreated. Otherwise, it is expected that
/// the agent to be deleted has no active claims.
///
/// # Arguments
///
/// * `ag` - AgentInstance handle
/// * `force` - Expects an empty agent when set to 0, forces deletion otherwise.
///
pub fn agent_agent_delete(ag: agent_t, force: u8) {
    unsafe {
        __agent_agent_delete(ag, force)
    }
}

/// Returns the AgentInstance handle of a claim.
/// Use the AgentInstance handle to perform invades using the same Agent. Using one Agent per 'logical'
/// application.
///
/// # Note
///
///  When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
///
pub fn agent_claim_get_agent(claim: agentclaim_t) -> agent_t {
    unsafe {
        __agent_claim_get_agent(claim)
    }
}

/// Returns the constraints_t of a Claim.
/// This function returns the constraints_t that have been specified during the invade
/// phase for an agentclaim_t.
///
/// # Note
///
///  constraints_t is only valid on the tile it has been created on! (It must be a local pointer.)
///  When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
///
pub fn agent_claim_get_constr(claim: agentclaim_t) -> constraints_t {
    unsafe {
        __agent_claim_get_constr(claim)
    }
}

/// Invades new resources.
/// This creates a new agent if no parentagent is specified. Otherwise, the specified agent
/// is used to perform the invade and will be responsible for managing the invaded resources.
/// A failed invade will lead to a NULL pointer being returned and the agent to be deleted,
/// if no other claims are active. Applications must check the return value for validity.
///
/// # Arguments
///
/// * `parentagent` - Agent handle of the parentclaim. NULL if a new Agent should be created.
///
///                    Use agent_claim_get_agent to get the Agent handle of an AgentOctoClaim.
///
/// # Arguments
///
/// * `constr` - Constraints handle, previously created with agent_constr_create();
///
/// # Return Value
///
/// AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC()) if the invade operation was successful, NULL otherwise.
///
pub fn agent_claim_invade(parentagent: agent_t, constr: constraints_t) -> agentclaim_t {
    unsafe {
        __agent_claim_invade(parentagent, constr)
    }
}

/// Invades new resources.
/// This creates a new agent if no parentagent is specified. Otherwise, the specified agent
/// is used to perform the invade and will be responsible for managing the invaded resources.
/// A failed invade will lead to a NULL pointer being returned and the agent to be deleted,
/// if no other claims are active. Applications must check the return value for validity.
///
/// # Arguments
///
/// * `parentagent` - Agent handle of the parentclaim. NULL if a new Agent should be created.
///
///                    Use agent_claim_get_agent to get the Agent handle of an AgentOctoClaim.
///
/// # Arguments
///
/// * `constr` - Constraints handle, previously created with agent_constr_create();
/// * `agent_name` - If parentagent is NULL, newly created Agent will be assigned this name.
///
/// # Return Value
///
/// AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC()) if the invade operation was successful, NULL otherwise.
///
pub fn agent_claim_invade_with_name(parentagent: agent_t, constr: constraints_t, agent_name: *const c_char) -> agentclaim_t {
    unsafe {
        __agent_claim_invade_with_name(parentagent, constr, agent_name)
    }
}

/// sets a name to a claim's Owning Agent
/// This assigns the given name to the Agent owning the claim.
/// If Claim is NULL or owning Agent is not valid it will cause panic to be thrown.
///
/// # Arguments
///
/// * `claim` - that belongs to the agent
/// * `agent_name` - If parentagent is NULL, newly created Agent will be assigned this name.
///
/// \return
pub fn set_agent_name(claim: agentclaim_t, agent_name: *const c_char) {
    unsafe {
        __set_agent_name(claim, agent_name)
    }
}

/// gets Name from a claim's Owning Agent
///
/// # Arguments
///
/// * `claim` - that belongs to the agent
/// * `buffer` - where to name is copied into
/// * `size` - of given buffer
///
/// # Return Value
///
/// C-string of the name.
///
pub fn get_agent_name(claim: agentclaim_t, buffer: *mut c_char, size: usize) -> i32 {
    unsafe {
        __get_agent_name(claim, buffer, size)
    }
}

/// Returns the sigma parameter of Downey's speedup curve model.
/// This is the getter for the variance of parallelism parameter in Downey's speedup curve
/// model, multiplied by 100.
/// For more information, look up the setter function
/// agent_constr_set_downey_speedup_curve(constraints_t constr, int A, int sigma);
///
/// # Arguments
///
/// * `constr` - Constraints handle
///
/// # Return Value
///
/// Constraints-internal sigma parameter of Downey's speedup curve model.
///
pub fn agent_get_downey_sigma(constr: constraints_t) -> i32 {
    unsafe {
        __agent_get_downey_sigma(constr)
    }
}

/// Invades resources using multiple sets of constraints ORed together and returns the
///        agentclaim_t these constraints_t gave the best overall rating for.
/// This creates a new agent if no parentagent is specified. Otherwise, the specified agent
/// is used to perform the invade and will be responsible for managing the invaded resources.
/// In contrast to agent_claim_invade(agentclaim_t parentclaim, constraints_t constr)
/// function, this function ORs multiple constraints together resulting in a potentially
/// highly complex invade call - it is not recommended to execute these invades within inner
/// loops.
/// A failed invade call with all constraints OR'd together will lead to a NULL pointer being
/// returned and the agent to be deleted, if no other claims are active. Applications must
/// check the return value for validity.
///
/// # Arguments
///
/// * `parentagent` - Agent handle of the parentclaim. NULL if a new Agent should be created.
///
///                    Use agent_claim_get_agent to get the Agent handle of an (Abstract)AgentOctoClaim.
///
/// # Arguments
///
/// * `constr_count` - Amount of Constraints that should be considered.
/// * `constr[]` - Constraints handles, previously created with agent_constr_create();
///
/// # Return Value
///
/// AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC()) if the invade operation was successful, NULL otherwise.
///
pub fn agent_claim_invade_or_constraints(parentagent: agent_t, constr_count: u8, constr: *mut constraints_t) -> agentclaim_t {
    unsafe {
        __agent_claim_invade_or_constraints(parentagent, constr_count, constr)
    }
}

/// Invades new resources and returns the AgentOctoClaim handle.
/// This creates a new agent if no parentclaim is specified. Otherwise, the agent responsible
/// for parentclaim is used to perform the invade and will be responsible for managing the
/// invaded resources.
/// A failed invade call will lead to a NULL pointer being returned and the agent to be
/// deleted, if no other claims are active. Applications must check the return value for
/// validity.
///
/// # Note
///
///  When called with a AgentOctoClaim (via parentclaim->asAOC()), the AgentOctoClaim handle is only valid on the tile it has been created (It is a local pointer!).
///  When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
///
/// # Arguments
///
/// * `parentclaim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC()) of the parentclaim.
///
///                    NULL if a new Agent should be created.
///                    The parentclaim is used to eventually allow recursive invasions more efficiently.
///
/// # Arguments
///
/// * `constr` - Constraints handle, previously created with agent_constr_create();
///
/// # Return Value
///
/// AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC()) if the invade operation was successful, NULL otherwise.
///
pub fn agent_claim_invade_parentclaim(parentclaim: agentclaim_t, constr: constraints_t) -> agentclaim_t {
    unsafe {
        __agent_claim_invade_parentclaim(parentclaim, constr)
    }
}

/// Wraps the initial OctoPOS-claim in an AgentOctoClaim.
/// This function wraps the initial OctoPOS-claim in an AgentOctoClaim and binds it to the initial Agent.
/// It is required to achieve a consistent system state where all resources are managed by Agents without
/// breaking existing OctoPOS applications or 'pure' OctoPOS applications that do not want to use the
/// Agent System.
/// See file octo_claim.h for more information about a claim_t.
///
/// # Arguments
///
/// * `octoclaim` - The OctoPOS-claim handle your application received in the main-ilet.
///
/// # Return Value
///
/// AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC())
///
pub fn agent_claim_get_initial(octoclaim: claim_t) -> agentclaim_t {
    unsafe {
        __agent_claim_get_initial(octoclaim)
    }
}

/// \ brief See agent_claim_get_initial(claim_t octoclaim)
///  In Addition to creating an agent, this function gives a name to the Initial Agent.
///
/// # Arguments
///
/// * `octoclaim` - The OctoPOS-claim handle your application received in the main-ilet.
/// * `agent_name` - the name to assign to the Agent
///
/// # Return Value
///
/// AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC())
///
pub fn agent_claim_get_initial_with_name(octoclaim: claim_t, agent_name: *const c_char) -> agentclaim_t {
    unsafe {
        __agent_claim_get_initial_with_name(octoclaim, agent_name)
    }
}

/// Retreats all Resources in a claim.
///
/// # Note
///
///  This will also delete the agent if this is the last claim the agent is responsible for!
///  When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
///
pub fn agent_claim_retreat(claim: agentclaim_t) {
    unsafe {
        __agent_claim_retreat(claim)
    }
}

/// Reinvades a claim with the current constraints.
/// The previously used constraints must not have been deleted. Changing the current constraints
/// also results in a different reinvade (i.e., constraints are not copied into the claim object!
/// To make the agent aware of the new constraints, however, reinvade needs to be called.)
/// A failed reinvade will lead to:
///   - the passed AgentOctoClaim object to be:
///     - retreated
///     - deleted
///   - internally:
///     - the original AgentClaim object "paired" with the passed AgentOctoClaim object
///       to be deleted
///     - the new AgentClaim object to be deleted
///   - the owning agent to be deleted, if no other claims are active.
///
/// # Note
///
///  If this functions fails, the passed object will be invalid and <b>must not</b>
/// be used any further!
///
/// # Note
///
///  When called with a ProxyAgentOctoClaim (via claim->asPAOC()), be aware that the reinvade changes the ucid, so the next time calling a function on the ProxyAgentOctoClaim, the application exits with an error message. See os::agent::ProxyAgentOctoClaim::reinvadeSameConstr() for more information about that.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
///
/// \returns Absolute sum of changes (i.e., gained and lost resources).
///          Returns 0 if reinvade resulted in no change to the claim.
///          Returns -1 if the reinvade was unsuccessful.
pub fn agent_claim_reinvade(claim: agentclaim_t) -> i32 {
    unsafe {
        __agent_claim_reinvade(claim)
    }
}

/// Reinvades 'claim' with new constraints.
/// A failed reinvade will lead to:
///   - the passed AgentOctoClaim object to be:
///     - retreated
///     - deleted
///   - internally:
///     - the original AgentClaim object "paired" with the passed AgentOctoClaim object
///       to be deleted
///     - the new AgentClaim object to be deleted
///   - the owning agent to be deleted, if no other claims are active.
///
/// # Note
///
///  If this functions fails, the passed object will be invalid and <b>must not</b>
/// be used any further!
///
/// # Note
///
///  When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
/// * `constr` - Constraints handle
///
/// \returns Absolute sum of changes (i.e., gained and lost resources).
///          Returns 0 if reinvade resulted in no change to the claim.
///          Returns -1 if the reinvade was unsuccessful.
pub fn agent_claim_reinvade_constraints(claim: agentclaim_t, constr: constraints_t) -> i32 {
    unsafe {
        __agent_claim_reinvade_constraints(claim, constr)
    }
}

/// Prints the resources of the claim to stdout.
///
/// # Note
///
///  If we print the resources of a ProxyAgentOctoClaim (via claim->asPAOC()) on a distant tile, in the "tiletail" window it will be printed on the tile where the ProxyAgentOctoClaim object is located, not where the ProxyAgentOctoClaim object is located.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
///
pub fn agent_claim_print(claim: agentclaim_t) {
    unsafe {
        __agent_claim_print(claim)
    }
}

/// Gets the total number of resources in a claim.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
///
/// \returns Total number of resources in given claim.
pub fn agent_claim_get_pecount(claim: agentclaim_t) -> i32 {
    unsafe {
        __agent_claim_get_pecount(claim)
    }
}

/// Gets the total number of resources of a specific type in a claim.
///
/// # Note
///
///  The resource type can either be 0 => RISC, 1 => iCORE, 2 => TCPA or 3 => NONE
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
/// * `type` - Resource type (see note)
///
/// \returns Total number of resources of specific type in given claim.
pub fn agent_claim_get_pecount_type(claim: agentclaim_t, _type: res_type_t) -> i32 {
    unsafe {
        __agent_claim_get_pecount_type(claim, _type)
    }
}

/// Gets the total number of resources on a specific tile in a claim.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
/// * `tileID` - TileID
///
/// \returns total number of resources on specific tile in given claim
pub fn agent_claim_get_pecount_tile(claim: agentclaim_t, tileID: tile_id_t) -> i32 {
    unsafe {
        __agent_claim_get_pecount_tile(claim, tileID)
    }
}

/// Gets the total number of resources on a specific tile of a specific type in a claim.
///
/// # Note
///
///  The resource type can either be 0 => RISC, 1 => iCORE, 2 => TCPA or 3 => NONE.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
/// * `tileID` - TileID
/// * `type` - Resource type (see note)
///
/// \returns Total number of resources on a specific tile of specific type in given claim.
pub fn agent_claim_get_pecount_tile_type(claim: agentclaim_t, tileID: tile_id_t, _type: res_type_t) -> i32 {
    unsafe {
        __agent_claim_get_pecount_tile_type(claim, tileID, _type)
    }
}

/// Gets an OctoPOS ProxyClaim handle for resources on a specific tile of a specific type in a claim.
///
/// # Note
///
///  This function needs to be used to get access to the OctoPOS ProxyClaim handles
///       to perform actual infects on the previously invaded resources!
///
/// # Note
///
///  The resource type can either be 0 => RISC, 1 => iCORE, 2 => TCPA or 3 => NONE
///       Currently only RISC is usable.
///
/// # Note
///
///  When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
/// * `tileID` - TileID
/// * `type` - Resource type (see note)
///
/// # Return Value
///
/// OctoPOS ProxyClaim handle
///
pub fn agent_claim_get_proxyclaim_tile_type(claim: agentclaim_t, tileID: i32, _type: res_type_t) -> proxy_claim_t {
    unsafe {
        __agent_claim_get_proxyclaim_tile_type(claim, tileID, _type)
    }
}

/// Gets the total number of tiles in a claim.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
///
/// \returns Total number of different tiles in given claim.
pub fn agent_claim_get_tilecount(claim: agentclaim_t) -> i32 {
    unsafe {
        __agent_claim_get_tilecount(claim)
    }
}

/// Gets the TileID at the specified position.
///
/// # Note
///
///  It is not very efficient to iterate over the tiles using this function as it does not maintain any internal state!
///  What is this function good for? Its implementation in file "AgentClaim.h" says "this function doesn't make too much sense as it is.." about it.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
/// * `iterator` - Integer iterating from 0 to the total number of tiles in claim.
///
/// \returns The tileID at the iterator position. Returns 0xFF if out of bounds.
pub fn agent_claim_get_tileid_iterative(claim: agentclaim_t, iterator: i32) -> i32 {
    unsafe {
        __agent_claim_get_tileid_iterative(claim, iterator)
    }
}

// Functions to interact with Constraints

/// Creates a new set of constraints.
/// All constraints are always ANDed together. To support ORed constraints, multiple sets of constraints have to
/// be created! This function initializes to default values.
///
/// # Note
///
///  The constraints handle is only valid on the tile it has been created. (It is local pointer!)
/// # Return Value
///
/// Constraints handle which is required for invading resources and actually changing the constraints.
///
pub fn agent_constr_create() -> constraints_t {
    unsafe {
        __agent_constr_create()
    }
}

/// Deletes a set of constraints.
///
/// # Note
///
///  Make sure to delete all the constraint-containers you created to avoid memory leaks!
///
/// # Arguments
///
/// * `constr` - Constraints handle that should be deleted.
///
pub fn agent_constr_delete(constr: constraints_t) {
    unsafe {
        __agent_constr_delete(constr)
    }
}

/// Merges two sets of constraints.
///
/// # Note
///
///  It is currently not decided on how to proceed with default values. I.e.: functionality not available!
///
/// # Arguments
///
/// * `constrTarget` - Constraints handle which should be the target of the merge.
/// * `additionalConstraints` - Constraints handle whose Constraints should be merged into the existing constraints.
///
pub fn agent_constr_overwrite(constrTarget: constraints_t, additionalConstraints: constraints_t) {
    unsafe {
        __agent_constr_overwrite(constrTarget, additionalConstraints)
    }
}

/// Sets the PE-Quantity Constraints.
///
/// # Note
///
///  The resource type can either be 0 => RISC, 1 => iCORE, 2 => TCPA or 3 => NONE.
///       Currently only RISC is usable.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which the PE-Quantity constraints should be set.
/// * `minPEs` - Minimum number of required PEs.
/// * `maxPEs` - Maximum number of useful PEs.
/// * `type` - PE type of which to set the PE-Quantity constraints. (See note.)
///
pub fn agent_constr_set_quantity(constr: constraints_t, minPEs: i32, maxPEs: i32, _type: res_type_t) {
    unsafe {
        __agent_constr_set_quantity(constr, minPEs, maxPEs, _type)
    }
}

/// Sets the A and sigma parameters of Downey's speedup curve model.
/// For a detailed description of the parameter semantics, read Downey's 1997 paper
/// "A model for speedup of parallel programs".
/// Intuitively, A is the upper bound of parallelism the application can exploit.
/// Sigma is 0, for applications that scale linearly and goes to infinity for the rest.
/// While sigma is a real number around 1.0 in the paper, we use 100 times the value
/// as an integer. Thus, a value of 1.0 in the paper corresponds to a sigma value of 100 here.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `A` -      average parallelism
/// * `sigma` -  variance of parallelism 100
///
pub fn agent_constr_set_downey_speedup_curve(constr: constraints_t, A: i32, sigma: i32) {
    unsafe {
        __agent_constr_set_downey_speedup_curve(constr, A, sigma)
    }
}

/// Sets the 'this tile' constraint.
/// Disallows all tiles except of TileID.
///
/// # Note
///
///  Invalidates the 'not on tile' constraint.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `TileID` - Resources MUST be located on TileID
///
pub fn agent_constr_set_tile(constr: constraints_t, TileID: tile_id_t) {
    unsafe {
        __agent_constr_set_tile(constr, TileID)
    }
}

/// Sets the 'not on tile' constraint.
/// Allows all tiles except of TileID.
///
/// # Note
///
///  Invalidates the 'this tile' constraint.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `TileID` - TileID to disallow.
///
pub fn agent_constr_set_notontile(constr: constraints_t, TileID: tile_id_t) {
    unsafe {
        __agent_constr_set_notontile(constr, TileID)
    }
}

/// Sets a bitmap which defines which tiles are allowed for invasion.
/// Bit 0 = tile 0, bit 1 = tile 1, etc...
/// A set bit represents an allowed TileID.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `bitmap` - Tile selection bitmap.
///
pub fn agent_constr_set_tile_bitmap(constr: constraints_t, bitmap: u16) {
    unsafe {
        __agent_constr_set_tile_bitmap(constr, bitmap)
    }
}

/// Sets the 'Sticky Claim' constraint.
/// When set to true (default case), the Agents never retreat whole tiles when calling reinvade.
/// When set to false, only one core in the tile calling reinvade is "immune", all other tiles might lose///all* cores.
///
/// # Note
///
///  On by default.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `sticky_boolean` - Turns the constraint off if 0, on otherwise.
///
pub fn agent_constr_set_stickyclaim(constr: constraints_t, sticky_boolean: u8) {
    unsafe {
        __agent_constr_set_stickyclaim(constr, sticky_boolean)
    }
}

/// Sets the 'ViPG Claim' constraint.
/// When set to true, the ViPG blocking of cores at re-invade is enabled (B3).
/// When set to false (default case), ViPG is disabled.
///
/// # Note
///
///  Off by default.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `vipgEnable` - Turns the constraint off if 0, on otherwise.
///
pub fn agent_constr_set_vipg(constr: constraints_t, vipgEnable: u8) {
    unsafe {
        __agent_constr_set_vipg(constr, vipgEnable)
    }
}

/// Sets the 'application class' hint.
///
/// # Note
///
///  Actually does nothing until monitoring subsystem is available.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `AppClass` - Application Class identifier
///
pub fn agent_constr_set_appclass(constr: constraints_t, AppClass: i32) {
    unsafe {
        __agent_constr_set_appclass(constr, AppClass)
    }
}

/// Sets the 'application number' constraint.
///
/// # Note
///
///  This is primarily used for the Ethernet-State-Dump interface.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `AppNr` - Application Identifier
///
pub fn agent_constr_set_appnumber(constr: constraints_t, AppNr: i32) {
    unsafe {
        __agent_constr_set_appnumber(constr, AppNr)
    }
}

/// Sets the 'tileSharing' constraint.
///
/// # Note
///
///  Tile sharing is enabled by default.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `is_tile_shareable` - Turns the constraint off if 0, on otherwise.
///
pub fn agent_constr_set_tile_shareable(constr: constraints_t, is_tile_shareable: u8) {
    unsafe {
        __agent_constr_set_tile_shareable(constr, is_tile_shareable)
    }
}

/// Sets the 'malleable' constraint and assigns a resize handler, if applicable.
///
/// # Note
///
///  Malleability is disabled by default.
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this constraint should be set.
/// * `malleable_boolean` - Turns the constraint off if 0, on otherwise.
/// * `resize_handler` - A pointer to the function that will be called on resizes.
///                      Must be non-NULL if malleable_boolean is not 0, otherwise
///                      it is ignored.
/// * `resize_env` - A pointer to random data, application-specific. The agent
///                  system will not access this data. Is ignored if
///                  malleable_boolean is 0.
pub fn agent_constr_set_malleable(constr: constraints_t, malleable_boolean: u8, resize_handler: resize_handler_t, resize_env: resize_env_t) {
    unsafe {
        __agent_constr_set_malleable(constr, malleable_boolean, resize_handler, resize_env)
    }
}

/* Testing functions only! Do not use! Undocumented for a good reason. */
pub fn agent_stresstest_agentclaim() {
    unsafe {
        __agent_stresstest_agentclaim()
    }
}

/* Testing functions only! Do not use! Undocumented for a good reason. */
pub fn agent_stresstest_agentoctoclaim() {
    unsafe {
        __agent_stresstest_agentoctoclaim()
    }
}

/// Assigns a reinvade handler which gets called on every reinvade of the claim which is constrained by the Constraints handle constr.
/// Assigns a reinvade handler which gets called on every reinvade of the claim which is constrained by the Constraints handle constr.
/// If this function is not used (e.g. before an invade), the reinvade_handler will be set to NULL and therefore will not be called on reinvades (see parameter notes).
///
/// # Arguments
///
/// * `constr` - Constraints handle for which this reinvade handler should be set.
/// * `reinvade_handler` - A pointer to the function that will be called on reinvades.
///
///			When reinvade_handler_t is NULL (default case), the handler will not be called on reinvades.
///			When reinvade_handler_t is non-NULL, the handler will be called on every reinvade (until it is set NULL again).
pub fn agent_constr_set_reinvade_handler(constr: constraints_t, reinvade_handler: reinvade_handler_t) {
    unsafe {
        __agent_constr_set_reinvade_handler(constr, reinvade_handler)
    }
}

/// Returns the current reinvade handler for the Constraints handle constr.
/// Returns the current reinvade handler for the Constraints handle constr.
///
/// # Arguments
///
/// * `constr` - Constraints handle whose reinvade handler should be returned.
///
/// # Return Value
///
/// The reinvade handler for the Constraints constr. Might be NULL if no reinvade handler or a NULL has been set as "reinvade handler".
///
pub fn agent_constr_get_reinvade_handler(constr: constraints_t) -> reinvade_handler_t {
    unsafe {
        __agent_constr_get_reinvade_handler(constr)
    }
}

/// Returns a new ProxyAgentOctoClaim handle which allows access to its corresponding AgentOctoClaim on tile id objects_tile. The agentclaim_t is castable to AbstractAgentOctoClaim* and finally to ProxyAgentOctoClaim* via asPAOC() method.
/// Returns a new ProxyAgentOctoClaim handle which allows access to its corresponding AgentOctoClaim on tile id objects_tile. The agentclaim_t is castable to AbstractAgentOctoClaim* and finally to ProxyAgentOctoClaim* via asPAOC() method.
///
/// # Note
///
///  The ProxyAgentOctoClaim can only be used within an iLet that needs access to ITS OWN AgentOctoClaim (the one whose ProxyClaim's were used to spawn this iLet).
///  It may result in undefined behavior if you create a ProxyAgentOctoClaim within the same iLet that has created the AgentOctoClaim object.
///  Retreating the AgentOctoClaim before an iLet which has a ProxyAgentOctoClaim ends, may result in undefined behavior. iRTSS allows retreating Claims without stopping the corresponding ilets.
///  Reinvasion via agent_claim_reinvade(agentclaim_t claim) leads to a new AgentOctoClaim with a different ucid. Therefore this leads to problems with the ProxyAgentOctoClaim. See os::agent::ProxyAgentOctoClaim::reinvadeSameConstr() for more information about that.
///
/// # Arguments
///
/// * `objects_tile` - The tile id where the AgentOctoClaim object is located. All corresponding objects (AgentClaim, Constraints etc.) must be located there, too.
/// * `octo_ucid` - The unique id (ucid) which belongs to the AgentOctoClaim that we want to access.
///
/// # Return Value
///
/// A new ProxyAgentOctoClaim handle (to be more precise, a AbstractAgentOctoClaim handle that is castable to a ProxyAgentOctoClaim via asPAOC() method) which allows access to its corresponding AgentOctoClaim on tile id objects_tile.
///
pub fn agent_proxy_get_proxyagentoctoclaim(objects_tile: i32, octo_ucid: u32) -> agentclaim_t {
    unsafe {
        __agent_proxy_get_proxyagentoctoclaim(objects_tile, octo_ucid)
    }
}

/// Deletes a ProxyAgentOctoClaim.
/// Deletes a ProxyAgentOctoClaim to free up some memory.
///
/// # Note
///
///  Make sure to delete all the objects you created to avoid memory leaks!
///  Has no effect on the referenced AgentOctoClaim. One may create and delete any number of ProxyAgentOctoClaims, it will have no effect on the AgentOctoClaim.
///  Exits application with error message when called with a AbstractAgentOctoClaim handle that is castable to a AgentOctoClaim* via asAOC().
///
/// # Arguments
///
/// * `proxy_agentoctoclaim` - claim AbstractAgentOctoClaim handle (which is ONLY castable to a ProxyAgentOctoClaim* via asPAOC()) that should be deleted.
///
pub fn agent_proxy_delete_proxyagentoctoclaim(proxy_agentoctoclaim: agentclaim_t) {
    unsafe {
        __agent_proxy_delete_proxyagentoctoclaim(proxy_agentoctoclaim)
    }
}

/// Returns the objects tile id where the AgentOctoClaim object is located that corresponds to the ProxyAgentOctoClaim handle proxy_agentoctoclaim.
/// Returns the objects tile id where the AgentOctoClaim object is located that corresponds to the ProxyAgentOctoClaim handle proxy_agentoctoclaim.
///
/// # Note
///
///  Exits application with error message when called with a AbstractAgentOctoClaim handle that is castable to a AgentOctoClaim* via asAOC().
///
/// # Arguments
///
/// * `proxy_agentoctoclaim` - The AbstractAgentOctoClaim whose objects' location we look for. The AgentOctoClaim object and all corresponding objects (AgentClaim, Constraints, ProxyClaim) must be located there (we don't want to scatter the objects corresponding to an iLet). proxy_agentoctoclaim must be castable to ProxyAgentOctoClaim via asPAOC() method.
///
/// # Return Value
///
/// The objects tile id where the AgentOctoClaim object is located that corresponds to the ProxyAgentOctoClaim handle proxy_agentoctoclaim.
///
pub fn agent_proxy_get_objectstile(proxy_agentoctoclaim: agentclaim_t) -> i32 {
    unsafe {
        __agent_proxy_get_objectstile(proxy_agentoctoclaim)
    }
}

/// Returns the Ucid of the AbstractAgentOctoClaim claim.
/// Returns the Ucid of the AbstractAgentOctoClaim claim. The ucid is the unique id of an os::agent::AgentClaim; the AgentClaim's referenced os::agent::AgentOctoClaim just returns its AgentClaim's ucid, so both have the same ucid. Furthermore, a os::agent::ProxyAgentOctoClaim has the same ucid as its referenced AgentOctoClaim; in the current implementation of os::agent::ProxyAgentOctoClaim, its ucid does not change during object's lifetime, so it the ProxyAgentOctoClaim class has a problem with reinvades, see os::agent::ProxyAgentOctoClaim::reinvadeSameConstr() for more information about that.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
///
/// # Return Value
///
/// The Ucid of the AbstractAgentOctoClaim claim.
///
pub fn agent_claim_get_ucid(claim: agentclaim_t) -> u32 {
    unsafe {
        __agent_claim_get_ucid(claim)
    }
}

/// Returns true iff AbstractAgentOctoClaim claim has no resources associated in its AgentClaim.
/// Returns true iff AbstractAgentOctoClaim claim has no resources associated in its AgentClaim. os::agent::AgentOctoClaim::isEmpty() just calls os::agent::AgentClaim::isEmpty() on its AgentClaim.
///
/// # Arguments
///
/// * `claim` - AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
///
/// # Return Value
///
/// True iff the claim has no resources associated in its AgentClaim.
///
pub fn agent_claim_isempty(claim: agentclaim_t) -> bool {
    unsafe {
        __agent_claim_isempty(claim)
    }
}

