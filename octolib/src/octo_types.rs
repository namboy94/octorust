use libc::c_void as __c_void;
use libc::wchar_t;

// C Types
pub type c_void = __c_void;
pub type c_char = wchar_t;

pub type claim_t = u8;
pub type tile_id_t = u8;
pub type tile_quantity_t = u8;
pub type pe_quantity_t = u32;
pub type pe_quantity_delta_t = i32;
pub type team_size_t = u32;

/** Agent Types */
enum ResType {RISC=0, iCore=1, TCPA=2, none=3, TYPE_ALL=4}
pub type res_type_t = u8;

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

pub type resize_handler_t = Option<unsafe extern "C" fn(arg1: agentclaim_t,
                                                        arg2: usize,
                                                        arg3: usize,
                                                        arg4: gain_t,
                                                        arg5: loss_t,
                                                        arg6: resize_env_t)>;
pub type reinvade_handler_t = Option<unsafe extern "C" fn()>;

/// Size type for DMA buffers.
pub type buf_size_t = u32;