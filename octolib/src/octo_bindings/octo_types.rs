use libc::c_void;

pub type claim_t = u8;
pub type tile_id_t = u8;
pub type agent_t = * c_void;
pub type agentclaim_t = * c_void;
pub type constraints_t = * c_void;
// TODO continue

res_type_t
proxy_claim_t