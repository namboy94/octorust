/**
 * \file octo_agent.h
 *
 * \brief Functions to create and interact with Agents, Constraints and "high level" Claims.
 */

/**
 * \brief Creates a new Agent.
 *
 * This creates a new Agent on the "Agent Tile". Please note that new Agents are automatically
 * created when using agent_claim_invade without specifying a parentclaim. (Consequently, you
 * should never have to use this function for now.)
 *
 * \return AgentInstance handle which is required for all subsequent communications with that agent.
 */
agent_t agent_agent_create(void);

/**
 * \brief Deletes a previously created agent. Automatically done when using agent_claim_retreat!
 *
 * If force is non-zero, all active claims will be retreated. Otherwise, it is expected that
 * the agent to be deleted has no active claims.
 *
 * \param ag AgentInstance handle
 * \param force Expects an empty agent when set to 0, forces deletion otherwise.
 */
void agent_agent_delete(agent_t ag, uint8_t force);

/**
 * \brief Returns the AgentInstance handle of a claim.
 *
 * Use the AgentInstance handle to perform invades using the same Agent. Using one Agent per 'logical'
 * application.
 *
 * \note When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 */
agent_t agent_claim_get_agent(agentclaim_t claim);

/**
 * \brief Returns the constraints_t of a Claim.
 *
 * This function returns the constraints_t that have been specified during the invade
 * phase for an agentclaim_t.
 *
 * \note constraints_t is only valid on the tile it has been created on! (It must be a local pointer.)
 * \note When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 */
constraints_t agent_claim_get_constr(agentclaim_t claim);


/**
 * \brief Invades new resources.
 *
 * This creates a new agent if no parentagent is specified. Otherwise, the specified agent
 * is used to perform the invade and will be responsible for managing the invaded resources.
 *
 * A failed invade will lead to a NULL pointer being returned and the agent to be deleted,
 * if no other claims are active. Applications must check the return value for validity.
 *
 * \param parentagent Agent handle of the parentclaim. NULL if a new Agent should be created.
 *                    Use agent_claim_get_agent to get the Agent handle of an AgentOctoClaim.
 * \param constr Constraints handle, previously created with agent_constr_create();
 *
 * \return AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC()) if the invade operation was successful, NULL otherwise.
 */
agentclaim_t agent_claim_invade(agent_t parentagent, constraints_t constr);

/**
 * \brief Invades new resources.
 *
 * This creates a new agent if no parentagent is specified. Otherwise, the specified agent
 * is used to perform the invade and will be responsible for managing the invaded resources.
 *
 * A failed invade will lead to a NULL pointer being returned and the agent to be deleted,
 * if no other claims are active. Applications must check the return value for validity.
 *
 * \param parentagent Agent handle of the parentclaim. NULL if a new Agent should be created.
 *                    Use agent_claim_get_agent to get the Agent handle of an AgentOctoClaim.
 * \param constr Constraints handle, previously created with agent_constr_create();
 * \param agent_name If parentagent is NULL, newly created Agent will be assigned this name.
 * \return AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC()) if the invade operation was successful, NULL otherwise.
 */
agentclaim_t agent_claim_invade_with_name(agent_t parentagent, constraints_t constr, const char* agent_name);

/**
 * \brief sets a name to a claim's Owning Agent
 *
 * This assigns the given name to the Agent owning the claim.
 * If Claim is NULL or owning Agent is not valid it will cause panic to be thrown.
 *
 * \param claim that belongs to the agent
 * \param agent_name If parentagent is NULL, newly created Agent will be assigned this name.
 * \return
 */
void set_agent_name(agentclaim_t claim, const char* agent_name);

/**
 * \brief gets Name from a claim's Owning Agent
 *
 * \param claim that belongs to the agent
 * \param buffer where to name is copied into
 * \param size of given buffer
 * \return C-string of the name.
 */
int get_agent_name(agentclaim_t claim, char buffer[], size_t size);

/**
 * \brief Returns the sigma parameter of Downey's speedup curve model.
 *
 * This is the getter for the variance of parallelism parameter in Downey's speedup curve
 * model, multiplied by 100.
 * For more information, look up the setter function
 * agent_constr_set_downey_speedup_curve(constraints_t constr, int A, int sigma);
 *
 * \param constr Constraints handle
 * \return Constraints-internal sigma parameter of Downey's speedup curve model.
 */
int agent_get_downey_sigma(constraints_t constr);

/**
 * \brief Invades resources using multiple sets of constraints ORed together and returns the
 *        agentclaim_t these constraints_t gave the best overall rating for.
 *
 * This creates a new agent if no parentagent is specified. Otherwise, the specified agent
 * is used to perform the invade and will be responsible for managing the invaded resources.
 *
 * In contrast to agent_claim_invade(agentclaim_t parentclaim, constraints_t constr)
 * function, this function ORs multiple constraints together resulting in a potentially
 * highly complex invade call - it is not recommended to execute these invades within inner
 * loops.
 *
 * A failed invade call with all constraints OR'd together will lead to a NULL pointer being
 * returned and the agent to be deleted, if no other claims are active. Applications must
 * check the return value for validity.
 *
 * \param parentagent Agent handle of the parentclaim. NULL if a new Agent should be created.
 *                    Use agent_claim_get_agent to get the Agent handle of an (Abstract)AgentOctoClaim.
 * \param constr_count Amount of Constraints that should be considered.
 * \param constr[] Constraints handles, previously created with agent_constr_create();
 *
 * \return AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC()) if the invade operation was successful, NULL otherwise.
 */
agentclaim_t agent_claim_invade_or_constraints(agent_t parentagent, uint8_t constr_count, constraints_t constr[]);

/**
 * \brief Invades new resources and returns the AgentOctoClaim handle.
 *
 * This creates a new agent if no parentclaim is specified. Otherwise, the agent responsible
 * for parentclaim is used to perform the invade and will be responsible for managing the
 * invaded resources.
 *
 * A failed invade call will lead to a NULL pointer being returned and the agent to be
 * deleted, if no other claims are active. Applications must check the return value for
 * validity.
 *
 * \note When called with a AgentOctoClaim (via parentclaim->asAOC()), the AgentOctoClaim handle is only valid on the tile it has been created (It is a local pointer!).
 * \note When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
 *
 * \param parentclaim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC()) of the parentclaim.
 *                    NULL if a new Agent should be created.
 *                    The parentclaim is used to eventually allow recursive invasions more efficiently.
 * \param constr Constraints handle, previously created with agent_constr_create();
 *
 * \return AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC()) if the invade operation was successful, NULL otherwise.
 */
agentclaim_t agent_claim_invade_parentclaim(agentclaim_t parentclaim, constraints_t constr);

/**
 * \brief Wraps the initial OctoPOS-claim in an AgentOctoClaim.
 *
 * This function wraps the initial OctoPOS-claim in an AgentOctoClaim and binds it to the initial Agent.
 * It is required to achieve a consistent system state where all resources are managed by Agents without
 * breaking existing OctoPOS applications or 'pure' OctoPOS applications that do not want to use the
 * Agent System.
 * See file octo_claim.h for more information about a claim_t.
 *
 * \param octoclaim The OctoPOS-claim handle your application received in the main-ilet.
 *
 * \return AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC())
 */
agentclaim_t agent_claim_get_initial(claim_t octoclaim);

/**
 * \ brief See agent_claim_get_initial(claim_t octoclaim)
 *
 *  In Addition to creating an agent, this function gives a name to the Initial Agent.
 *
 * \param octoclaim The OctoPOS-claim handle your application received in the main-ilet.
 * \param agent_name the name to assign to the Agent
 *
 * \return AbstractAgentOctoClaim handle (which is castable to AgentOctoClaim* via asAOC())
 */
agentclaim_t agent_claim_get_initial_with_name(claim_t octoclaim, const char* agent_name);

/**
 * \brief Retreats all Resources in a claim.
 *
 * \note This will also delete the agent if this is the last claim the agent is responsible for!
 * \note When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 */
void agent_claim_retreat(agentclaim_t claim);

/**
 * \brief Reinvades a claim with the current constraints.
 *
 * The previously used constraints must not have been deleted. Changing the current constraints
 * also results in a different reinvade (i.e., constraints are not copied into the claim object!
 * To make the agent aware of the new constraints, however, reinvade needs to be called.)
 *
 * A failed reinvade will lead to:
 *   - the passed AgentOctoClaim object to be:
 *     - retreated
 *     - deleted
 *   - internally:
 *     - the original AgentClaim object "paired" with the passed AgentOctoClaim object
 *       to be deleted
 *     - the new AgentClaim object to be deleted
 *   - the owning agent to be deleted, if no other claims are active.
 *
 * \note If this functions fails, the passed object will be invalid and <b>must not</b>
 * be used any further!
 * \note When called with a ProxyAgentOctoClaim (via claim->asPAOC()), be aware that the reinvade changes the ucid, so the next time calling a function on the ProxyAgentOctoClaim, the application exits with an error message. See os::agent::ProxyAgentOctoClaim::reinvadeSameConstr() for more information about that.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 *
 * \returns Absolute sum of changes (i.e., gained and lost resources).
 *          Returns 0 if reinvade resulted in no change to the claim.
 *          Returns -1 if the reinvade was unsuccessful.
 */
int agent_claim_reinvade(agentclaim_t claim);

/**
 * \brief Reinvades 'claim' with new constraints.
 *
 * A failed reinvade will lead to:
 *   - the passed AgentOctoClaim object to be:
 *     - retreated
 *     - deleted
 *   - internally:
 *     - the original AgentClaim object "paired" with the passed AgentOctoClaim object
 *       to be deleted
 *     - the new AgentClaim object to be deleted
 *   - the owning agent to be deleted, if no other claims are active.
 *
 * \note If this functions fails, the passed object will be invalid and <b>must not</b>
 * be used any further!
 * \note When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 * \param constr Constraints handle
 *
 * \returns Absolute sum of changes (i.e., gained and lost resources).
 *          Returns 0 if reinvade resulted in no change to the claim.
 *          Returns -1 if the reinvade was unsuccessful.
 */
int agent_claim_reinvade_constraints(agentclaim_t claim, constraints_t constr);

/**
 * \brief Prints the resources of the claim to stdout.
 *
 * \note If we print the resources of a ProxyAgentOctoClaim (via claim->asPAOC()) on a distant tile, in the "tiletail" window it will be printed on the tile where the ProxyAgentOctoClaim object is located, not where the ProxyAgentOctoClaim object is located.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 */
void agent_claim_print(agentclaim_t claim);

/**
 * \brief Gets the total number of resources in a claim.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 *
 * \returns Total number of resources in given claim.
 */
int agent_claim_get_pecount(agentclaim_t claim);

/**
 * \brief Gets the total number of resources of a specific type in a claim.
 *
 * \note The resource type can either be 0 => RISC, 1 => iCORE, 2 => TCPA or 3 => NONE
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 * \param type Resource type (see note)
 *
 * \returns Total number of resources of specific type in given claim.
 */
int agent_claim_get_pecount_type(agentclaim_t claim, res_type_t type);

/**
 * \brief Gets the total number of resources on a specific tile in a claim.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 * \param tileID TileID
 *
 * \returns total number of resources on specific tile in given claim
 */
int agent_claim_get_pecount_tile(agentclaim_t claim, tile_id_t tileID);

/**
 * \brief Gets the total number of resources on a specific tile of a specific type in a claim.
 *
 * \note The resource type can either be 0 => RISC, 1 => iCORE, 2 => TCPA or 3 => NONE.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 * \param tileID TileID
 * \param type Resource type (see note)
 *
 * \returns Total number of resources on a specific tile of specific type in given claim.
 */
int agent_claim_get_pecount_tile_type(agentclaim_t claim, tile_id_t tileID, res_type_t type);

/**
 * \brief Gets an OctoPOS ProxyClaim handle for resources on a specific tile of a specific type in a claim.
 *
 * \note This function needs to be used to get access to the OctoPOS ProxyClaim handles
 *       to perform actual infects on the previously invaded resources!
 *
 * \note The resource type can either be 0 => RISC, 1 => iCORE, 2 => TCPA or 3 => NONE
 *       Currently only RISC is usable.
 *
 * \note When called with a ProxyAgentOctoClaim (via claim->asPAOC()) from a different tile than where the AgentOctoClaim is located, exits application with error message.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 * \param tileID TileID
 * \param type Resource type (see note)
 *
 * \return OctoPOS ProxyClaim handle
 */
proxy_claim_t agent_claim_get_proxyclaim_tile_type(agentclaim_t claim, int tileID, res_type_t type);

/**
 * \brief Gets the total number of tiles in a claim.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 *
 * \returns Total number of different tiles in given claim.
 */
int agent_claim_get_tilecount(agentclaim_t claim);

/**
 * \brief Gets the TileID at the specified position.
 *
 * \note It is not very efficient to iterate over the tiles using this function as it does not maintain any internal state!
 * \note What is this function good for? Its implementation in file "AgentClaim.h" says "this function doesn't make too much sense as it is.." about it.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 * \param iterator Integer iterating from 0 to the total number of tiles in claim.
 *
 * \returns The tileID at the iterator position. Returns 0xFF if out of bounds.
 */
int agent_claim_get_tileid_iterative(agentclaim_t claim, int iterator);


// Functions to interact with Constraints

/**
 * \brief Creates a new set of constraints.
 *
 * All constraints are always ANDed together. To support ORed constraints, multiple sets of constraints have to
 * be created! This function initializes to default values.
 *
 * \note The constraints handle is only valid on the tile it has been created. (It is local pointer!)
 *
 * \return Constraints handle which is required for invading resources and actually changing the constraints.
 */
constraints_t agent_constr_create(void);

/**
 * \brief Deletes a set of constraints.
 *
 * \note Make sure to delete all the constraint-containers you created to avoid memory leaks!
 *
 * \param constr Constraints handle that should be deleted.
 *
 */
void agent_constr_delete(constraints_t constr);

/**
 * \brief Merges two sets of constraints.
 *
 * \note It is currently not decided on how to proceed with default values. I.e.: functionality not available!
 *
 * \param constrTarget Constraints handle which should be the target of the merge.
 * \param additionalConstraints Constraints handle whose Constraints should be merged into the existing constraints.
 */
void agent_constr_overwrite(constraints_t constrTarget, constraints_t additionalConstraints); // TODO

/**
 * \brief Sets the PE-Quantity Constraints.
 *
 * \note The resource type can either be 0 => RISC, 1 => iCORE, 2 => TCPA or 3 => NONE.
 *       Currently only RISC is usable.
 *
 * \param constr Constraints handle for which the PE-Quantity constraints should be set.
 * \param minPEs Minimum number of required PEs.
 * \param maxPEs Maximum number of useful PEs.
 * \param type PE type of which to set the PE-Quantity constraints. (See note.)
 */
void agent_constr_set_quantity(constraints_t constr, int minPEs, int maxPEs, res_type_t type); // sets the PE-Quantity Constraint (min, max)

/**
 * \brief Sets the A and sigma parameters of Downey's speedup curve model.
 *
 * For a detailed description of the parameter semantics, read Downey's 1997 paper
 * "A model for speedup of parallel programs".
 *
 * Intuitively, A is the upper bound of parallelism the application can exploit.
 * Sigma is 0, for applications that scale linearly and goes to infinity for the rest.
 * While sigma is a real number around 1.0 in the paper, we use 100 times the value
 * as an integer. Thus, a value of 1.0 in the paper corresponds to a sigma value of 100 here.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param A      average parallelism
 * \param sigma  variance of parallelism * 100
 */
void agent_constr_set_downey_speedup_curve(constraints_t constr, int A, int sigma);

/**
 * \brief Sets the 'this tile' constraint.
 *
 * Disallows all tiles except of TileID.
 *
 * \note Invalidates the 'not on tile' constraint.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param TileID Resources MUST be located on TileID
 */
void agent_constr_set_tile(constraints_t constr, tile_id_t TileID); // sets the 'this'-Place-Constraint (only cores on Tile TileID)

/**
 * \brief Sets the 'not on tile' constraint.
 *
 * Allows all tiles except of TileID.
 *
 * \note Invalidates the 'this tile' constraint.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param TileID TileID to disallow.
 */
void agent_constr_set_notontile(constraints_t constr, tile_id_t TileID);

/**
 * \brief Sets a bitmap which defines which tiles are allowed for invasion.
 *
 * Bit 0 = tile 0, bit 1 = tile 1, etc...
 * A set bit represents an allowed TileID.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param bitmap Tile selection bitmap.
 */
void agent_constr_set_tile_bitmap(constraints_t constr, uint16_t bitmap);

/**
 * \brief Sets the 'Sticky Claim' constraint.
 *
 * When set to true (default case), the Agents never retreat whole tiles when calling reinvade.
 * When set to false, only one core in the tile calling reinvade is "immune", all other tiles might lose *all* cores.
 *
 * \note On by default.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param sticky_boolean Turns the constraint off if 0, on otherwise.
 */
void agent_constr_set_stickyclaim(constraints_t constr, uint8_t sticky_boolean);

/**
 * \brief Sets the 'ViPG Claim' constraint.
 *
 * When set to true, the ViPG blocking of cores at re-invade is enabled (B3).
 * When set to false (default case), ViPG is disabled.
 *
 * \note Off by default.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param vipgEnable Turns the constraint off if 0, on otherwise.
 */
void agent_constr_set_vipg(constraints_t constr, uint8_t vipgEnable);

/**
 * \brief Sets the 'application class' hint.
 *
 * \note Actually does nothing until monitoring subsystem is available.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param AppClass Application Class identifier
 */
void agent_constr_set_appclass(constraints_t constr, int AppClass);


/**
 * \brief Sets the 'application number' constraint.
 *
 * \note This is primarily used for the Ethernet-State-Dump interface.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param AppNr Application Identifier
 */
void agent_constr_set_appnumber(constraints_t constr, int AppNr);


/**
 * \brief Sets the 'tileSharing' constraint.
 *
 * \note Tile sharing is enabled by default.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param is_tile_shareable Turns the constraint off if 0, on otherwise.
 */
void agent_constr_set_tile_shareable(constraints_t constr, uint8_t is_tile_shareable);

/**
 * \brief Sets the 'malleable' constraint and assigns a resize handler, if applicable.
 *
 * \note Malleability is disabled by default.
 *
 * \param constr Constraints handle for which this constraint should be set.
 * \param malleable_boolean Turns the constraint off if 0, on otherwise.
 * \param resize_handler A pointer to the function that will be called on resizes.
 *                       Must be non-NULL if malleable_boolean is not 0, otherwise
 *                       it is ignored.
 * \param resize_env A pointer to random data, application-specific. The agent
 *                   system will not access this data. Is ignored if
 *                   malleable_boolean is 0.
 */
void agent_constr_set_malleable(constraints_t constr, uint8_t malleable_boolean, resize_handler_t resize_handler, resize_env_t resize_env);

/* Testing functions only! Do not use! Undocumented for a good reason. */
void agent_stresstest_agentclaim(void);

/* Testing functions only! Do not use! Undocumented for a good reason. */
void agent_stresstest_agentoctoclaim(void);

/**
 * \brief Assigns a reinvade handler which gets called on every reinvade of the claim which is constrained by the Constraints handle constr.
 *
 * Assigns a reinvade handler which gets called on every reinvade of the claim which is constrained by the Constraints handle constr.
 * If this function is not used (e.g. before an invade), the reinvade_handler will be set to NULL and therefore will not be called on reinvades (see parameter notes).
 *
 * \param constr Constraints handle for which this reinvade handler should be set.
 * \param reinvade_handler A pointer to the function that will be called on reinvades.
 *			When reinvade_handler_t is NULL (default case), the handler will not be called on reinvades.
 *			When reinvade_handler_t is non-NULL, the handler will be called on every reinvade (until it is set NULL again).
 */
void agent_constr_set_reinvade_handler(constraints_t constr, reinvade_handler_t reinvade_handler);

/**
 * \brief Returns the current reinvade handler for the Constraints handle constr.
 *
 * Returns the current reinvade handler for the Constraints handle constr.
 *
 * \param constr Constraints handle whose reinvade handler should be returned.
 *
 * \return The reinvade handler for the Constraints constr. Might be NULL if no reinvade handler or a NULL has been set as "reinvade handler".
*/
reinvade_handler_t agent_constr_get_reinvade_handler(constraints_t constr);

/**
 * \brief Returns a new ProxyAgentOctoClaim handle which allows access to its corresponding AgentOctoClaim on tile id objects_tile. The agentclaim_t is castable to AbstractAgentOctoClaim* and finally to ProxyAgentOctoClaim* via asPAOC() method.
 *
 * Returns a new ProxyAgentOctoClaim handle which allows access to its corresponding AgentOctoClaim on tile id objects_tile. The agentclaim_t is castable to AbstractAgentOctoClaim* and finally to ProxyAgentOctoClaim* via asPAOC() method.
 *
 * \note The ProxyAgentOctoClaim can only be used within an iLet that needs access to ITS OWN AgentOctoClaim (the one whose ProxyClaim's were used to spawn this iLet).
 * \note It may result in undefined behavior if you create a ProxyAgentOctoClaim within the same iLet that has created the AgentOctoClaim object.
 * \note Retreating the AgentOctoClaim before an iLet which has a ProxyAgentOctoClaim ends, may result in undefined behavior. iRTSS allows retreating Claims without stopping the corresponding ilets.
 * \note Reinvasion via agent_claim_reinvade(agentclaim_t claim) leads to a new AgentOctoClaim with a different ucid. Therefore this leads to problems with the ProxyAgentOctoClaim. See os::agent::ProxyAgentOctoClaim::reinvadeSameConstr() for more information about that.
 *
 * \param objects_tile The tile id where the AgentOctoClaim object is located. All corresponding objects (AgentClaim, Constraints etc.) must be located there, too.
 * \param octo_ucid The unique id (ucid) which belongs to the AgentOctoClaim that we want to access.
 *
 * \return A new ProxyAgentOctoClaim handle (to be more precise, a AbstractAgentOctoClaim handle that is castable to a ProxyAgentOctoClaim via asPAOC() method) which allows access to its corresponding AgentOctoClaim on tile id objects_tile.
 */
agentclaim_t agent_proxy_get_proxyagentoctoclaim(int objects_tile, uint32_t octo_ucid);

/**
 * \brief Deletes a ProxyAgentOctoClaim.
 *
 * Deletes a ProxyAgentOctoClaim to free up some memory.
 *
 * \note Make sure to delete all the objects you created to avoid memory leaks!
 * \note Has no effect on the referenced AgentOctoClaim. One may create and delete any number of ProxyAgentOctoClaims, it will have no effect on the AgentOctoClaim.
 * \note Exits application with error message when called with a AbstractAgentOctoClaim handle that is castable to a AgentOctoClaim* via asAOC().
 *
 * \param proxy_agentoctoclaim claim AbstractAgentOctoClaim handle (which is ONLY castable to a ProxyAgentOctoClaim* via asPAOC()) that should be deleted.
 *
 */
void agent_proxy_delete_proxyagentoctoclaim(agentclaim_t proxy_agentoctoclaim);

/**
 * \brief Returns the objects tile id where the AgentOctoClaim object is located that corresponds to the ProxyAgentOctoClaim handle proxy_agentoctoclaim.
 *
 * Returns the objects tile id where the AgentOctoClaim object is located that corresponds to the ProxyAgentOctoClaim handle proxy_agentoctoclaim.
 *
 * \note Exits application with error message when called with a AbstractAgentOctoClaim handle that is castable to a AgentOctoClaim* via asAOC().
 *
 * \param proxy_agentoctoclaim The AbstractAgentOctoClaim whose objects' location we look for. The AgentOctoClaim object and all corresponding objects (AgentClaim, Constraints, ProxyClaim) must be located there (we don't want to scatter the objects corresponding to an iLet). proxy_agentoctoclaim must be castable to ProxyAgentOctoClaim via asPAOC() method.
 *
 * \return The objects tile id where the AgentOctoClaim object is located that corresponds to the ProxyAgentOctoClaim handle proxy_agentoctoclaim.
 */
int agent_proxy_get_objectstile(agentclaim_t proxy_agentoctoclaim);

/**
 * \brief Returns the Ucid of the AbstractAgentOctoClaim claim.
 *
 * Returns the Ucid of the AbstractAgentOctoClaim claim. The ucid is the unique id of an os::agent::AgentClaim; the AgentClaim's referenced os::agent::AgentOctoClaim just returns its AgentClaim's ucid, so both have the same ucid. Furthermore, a os::agent::ProxyAgentOctoClaim has the same ucid as its referenced AgentOctoClaim; in the current implementation of os::agent::ProxyAgentOctoClaim, its ucid does not change during object's lifetime, so it the ProxyAgentOctoClaim class has a problem with reinvades, see os::agent::ProxyAgentOctoClaim::reinvadeSameConstr() for more information about that.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 *
 * \return The Ucid of the AbstractAgentOctoClaim claim.
*/
uint32_t agent_claim_get_ucid(agentclaim_t claim);

/**
 * \brief Returns true iff AbstractAgentOctoClaim claim has no resources associated in its AgentClaim.
 *
 * Returns true iff AbstractAgentOctoClaim claim has no resources associated in its AgentClaim. os::agent::AgentOctoClaim::isEmpty() just calls os::agent::AgentClaim::isEmpty() on its AgentClaim.
 *
 * \param claim AbstractAgentOctoClaim handle (which is either castable to a AgentOctoClaim* via asAOC() XOR to a ProxyAgentOctoClaim* via asPAOC())
 *
 * \return True iff the claim has no resources associated in its AgentClaim.
*/
bool agent_claim_isempty(agentclaim_t claim);

/** @} */ //End agent_grp