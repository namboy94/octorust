use libc;

// C Types
pub type c_int = libc::c_int;
pub type c_void = libc::c_void;
pub type c_char = libc::wchar_t;

// Types
pub type claim_t = u8;
pub type tile_id_t = u8;
pub type tile_quantity_t = u8;
pub type pe_quantity_t = u32;
pub type pe_quantity_delta_t = i32;
pub type team_size_t = u32;

/// Represents a os::agent::AbstractAgentOctoClaim* in the C interface. So an agentclaim_t must be
/// castable to a os::agent::AbstractAgentOctoClaim* via static cast.
/// The os::agent::AbstractAgentOctoClaim* can usually be used directly, but sometimes needs to be
/// downcasted to one of its two subclasses. To enable downcasting, the AbstractAgentOctoClaim* must
/// be castable to a os::agent::AgentOctoClaim* XOR a os::agent::ProxyAgentOctoClaim* via methods
/// os::agent::AbstractAgentOctoClaim::asPAOC()
/// respectively os::agent::AbstractAgentOctoClaim::asAOC() depending on what it represents.
/// As a rule of thumb, as a function parameter, the agentclaim_t can be casted to either the one
/// or the other class; as a function return value, the agentclaim_t is mostly only castable to a
/// os::agent::AgentOctoClaim*, except for method agent_proxy_get_proxyagentoctoclaim
/// (only castable to a os::agent::ProxyAgentOctoClaim*).
/// Whether an agentclaim_t is castable to the one or the other subclass
/// of os::agent::AbstractAgentOctoClaim is noted in all methods of the C interface, so don't worry.
/// os::agent::ProxyAgentOctoClaims can be used in an iLet to access
/// an iLet's os::agent:AgentOctoClaim, even if the iLet does not run on the tile where
/// the os::agent:AgentOctoClaim object is located.
pub type agentclaim_t = *mut c_void;

/// Represents a os::agent::Constraints* in the C interface.
/// So a constraints_t must be castable to a os::agent::Constraints* via static cast.
pub type constraints_t = *mut c_void;

/// Represents aos::agent::AgentInstance* in the C interface.
/// So a agent_t must be castable to a os::agent::AgentInstance* via static cast.
pub type agent_t = *mut c_void;

/// struct proxy_claim
/// ProxyClaims are system data structures: Hence, only a Pointer is needed.
pub type proxy_claim_t = *mut c_void;

/*
 * Both gain_t and loss_t are actually two-dimensional uint8_t arrays,
 * bounded by the tile_count and res_per_tile parameters.
 */
pub type gain_t = *mut c_void;
pub type loss_t = *mut c_void;
pub type resize_env_t = *mut c_void;

/// Size type for DMA buffers.
pub type buf_size_t = u32;

/** Agent Types */
enum ResType {RISC=0, iCore=1, TCPA=2, none=3, TYPE_ALL=4}
pub type res_type_t = u8;

// Handlers
pub type resize_handler_t = extern "C" fn(arg1: agentclaim_t,
                                          arg2: usize,
                                          arg3: usize,
                                          arg4: gain_t,
                                          arg5: loss_t,
                                          arg6: resize_env_t);
pub type reinvade_handler_t = extern "C" fn();


// Types originally in other files
pub type clock_t = u64;
pub type eth_channel_t = *mut c_void;
pub type ilet_func = extern fn(arg1: *mut c_void);
pub type rust_ilet_func = fn(arg1: *mut c_void);
pub type dual_ilet_func = extern fn(arg1: *mut c_void, arg2: *mut c_void);
pub type rust_dual_ilet_func = fn(arg1: *mut c_void, arg2: *mut c_void);
pub type tcpa_proxy_claim_t = *mut c_void;
pub type tcpa_infect_response_t = *mut c_void;
pub type tcpa_output_transfer_confirm_t = *mut c_void;


// Enums
enum ETH_MODE {
	ETH_MODE_READ  = 0x1,
	ETH_MODE_WRITE = 0x2
}

enum ETH_TRANS {
	ETH_TRANS_CHUNKS  = 1,
	ETH_TRANS_CONTROL = 2
}

/// Memory types.
enum MEMTYPES {
	MEM_TLM_LOCAL = 0, // Tile-local memory from this tile's local address space.
	MEM_TLM_GLOBAL = 1, // Tile-local memory somewhere in the range of the tile's shared address space.
	MEM_SHM = 2,  // Shared (global) memory.
	MEM_ICM = 3, // iCore memory.
	MEM_TYPES_SIZE = 4,  // this is the number of valid MEM_types
	MEM_INVALID = -1,  // < Invalid memory region.
}



