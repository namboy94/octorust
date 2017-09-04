use libc::c_void;
use core::ptr;
pub type agentclaim_t = *mut c_void;
pub type constraints_t = *mut c_void;
pub type agent_t = *mut c_void;
pub type res_type_t = u8;

extern {
    fn printf(s: *const u8, ...);
    fn agent_constr_create() -> constraints_t;
    fn agent_constr_set_quantity(constr: constraints_t, min_pes: usize, max_pes: usize, res_type: res_type_t);
    fn agent_claim_invade(parentagent: agent_t, constr: constraints_t) -> agentclaim_t;
    fn agent_claim_reinvade(claim: agentclaim_t) -> i32;
    fn agent_constr_set_tile_shareable(constr: constraints_t, is_tile_shareable: u8);
}

pub struct ConstraintsPrototype {
    constraints: constraints_t
}
impl ConstraintsPrototype {

    pub fn new(min_pes: usize, max_pes: usize) -> ConstraintsPrototype {
        unsafe {
            let mut constr = agent_constr_create();
            agent_constr_set_quantity(constr, min_pes, max_pes, 0);
            agent_constr_set_tile_shareable(constr, 1); // Default to shareable
            return ConstraintsPrototype { constraints: constr }
        }
    }

    pub fn get_constraints(&self) -> constraints_t {
        self.constraints
    }

}

pub struct ClaimPrototype {
    claim: agentclaim_t,
    constraints: constraints_t
}
impl ClaimPrototype {

    pub fn new(proto_constraints: ConstraintsPrototype) -> ClaimPrototype {

        unsafe {
            let mut constraints = proto_constraints.get_constraints();
            let mut agent = agent_claim_invade(ptr::null_mut(), constraints);
            return ClaimPrototype { claim: agent, constraints: constraints }
        }

    }

    pub fn reinvade(&self) {

        unsafe {
            let status = agent_claim_reinvade(self.claim);
            if status == -1 {
                printf("* Reinvade Failed\n\0".as_ptr());
            } else {
                printf("* Reinvade Successful\n\0".as_ptr());
            }
        }
    }

    pub fn infect(&self) {

    }
}