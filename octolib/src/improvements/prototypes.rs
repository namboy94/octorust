use libc;
use core::ptr;
use core::mem;

// Dependencies

const SIMPLE_ILET_SIZE: usize = 16usize;
const SIMPLE_SIGNAL_SIZE: usize = 64usize;
type agentclaim_t = *mut libc::c_void;
type constraints_t = *mut libc::c_void;
type agent_t = *mut libc::c_void;
type proxy_claim_t = *mut libc::c_void;
type res_type_t = u8;
type tile_id_t = u8;
type team_size_t = u32;
type ilet_func = extern fn(arg1: *mut libc::c_void);


#[repr(C)]
struct simple_signal {
    pub padding: [libc::c_char; SIMPLE_SIGNAL_SIZE],
}

#[repr(C)]
struct simple_ilet {
    pub padding: [libc::c_char; SIMPLE_ILET_SIZE],
}

extern {
    fn printf(s: *const u8, ...);
    fn agent_constr_create() -> constraints_t;
    fn agent_constr_set_tile_shareable(constr: constraints_t, is_tile_shareable: u8);
    fn agent_constr_set_quantity(constr: constraints_t, min_pes: usize, max_pes: usize, res_type: res_type_t);
    fn agent_claim_invade(parentagent: agent_t, constr: constraints_t) -> agentclaim_t;
    fn agent_claim_reinvade(claim: agentclaim_t) -> i32;
    fn agent_claim_get_pecount(claim: agentclaim_t) -> i32;
    fn agent_claim_get_pecount_tile_type(claim: agentclaim_t, tile_id: tile_id_t, res_type: res_type_t) -> i32;
    fn agent_claim_get_proxyclaim_tile_type(claim: agentclaim_t, tile_id: i32, res_type: res_type_t) -> proxy_claim_t;
    fn get_tile_count() -> u32;
    fn simple_signal_init(instance: *mut simple_signal, wait_for_ilet_count: usize) -> i32;
    fn simple_signal_wait(instance: *mut simple_signal);
    fn simple_signal_signal_and_exit(instance: *mut simple_signal);
    fn simple_ilet_init(ilet: *mut simple_ilet, code: ilet_func, param: *mut libc::c_void);
    fn proxy_infect_with_ilet(claim: agentclaim_t, ilet: ilet_func, pes: i32, param: *mut libc::c_void);
    fn dispatch_claim_send_reply(ilet: *mut simple_ilet);
    fn proxy_infect(claim: proxy_claim_t, team: *mut simple_ilet, count: team_size_t);
}


// START HERE

pub struct ConstraintsPrototype {
    constraints: constraints_t
}
impl ConstraintsPrototype {

    pub fn new(min_pes: usize, max_pes: usize) -> ConstraintsPrototype {
        unsafe {
            let constr = agent_constr_create();
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
            let constraints = proto_constraints.get_constraints();
            let agent = agent_claim_invade(ptr::null_mut(), constraints);
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

    pub fn infect(&self, ilet: ilet_func) {

        unsafe {

            let mut sync = simple_signal { padding: [0; 64] };
            simple_signal_init(&mut sync, agent_claim_get_pecount(self.claim) as usize);

            for tile in 0..get_tile_count() {
                let pes = agent_claim_get_pecount_tile_type(self.claim, tile as u8, 0);
                if pes != 0 { // Type = 0 ^= RISC

                    let proxy_claim = agent_claim_get_proxyclaim_tile_type(self.claim, tile as i32, 0);
                    printf("* Got Proxy Claim %p\n\0".as_ptr(), proxy_claim);

                    printf("* Infecting\n\0".as_ptr());

                    // malloc/free because we can't initialize arrays like in C
                    let arraysize = pes as usize * mem::size_of::<simple_ilet>();
                    let mut ilets: *mut simple_ilet = libc::malloc(arraysize) as *mut simple_ilet;
                    for i in 0..pes as isize {
                        simple_ilet_init(ilets.offset(i), ilet, &mut sync as *mut _ as *mut libc::c_void)
                    }
                    proxy_infect(proxy_claim, ilets.offset(0), pes as u32);
                    libc::free(ilets as *mut _ as *mut libc::c_void);
                }
            }

            printf("Waiting on Signal %p...\n\0".as_ptr(), &mut sync);
            simple_signal_wait(&mut sync);
            printf("All Signals received!\n\0".as_ptr());
        }
    }
}

pub fn reply_signal(signal: *mut libc::c_void) {

    extern "C" fn inner(sig: *mut libc::c_void) {
        unsafe {
            simple_signal_signal_and_exit(sig as *mut simple_signal);
        }
    }

    let mut answer: simple_ilet = simple_ilet { padding: [0; SIMPLE_ILET_SIZE] };
    unsafe {
        printf("Answering Signal %p\n\0".as_ptr(), signal);
	    simple_ilet_init(&mut answer, inner, signal);
        dispatch_claim_send_reply(&mut answer);
    }
}
