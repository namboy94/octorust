use core::ptr;
use improvements::constraints::Constraint;
use octo_agent::{agent_claim_invade};

pub struct Claim {
    constraint: Constraint
}

impl Claim {

    pub fn new(constraint: Constraint) -> Claim {
        Claim {constraint: constraint}
    }

    pub fn invade(&self) {
        agent_claim_invade(ptr::null_mut(), self.constraint.to_contraints_t());
    }

    pub fn reinvade(constraint: Option<Constraint>) {

    }

    pub fn infect(ilet: fn ()) {

    }
}


/* Returns a claim fulfilling the constraint or throws a NotEnoughResources exception */
/*
    public static def invade(c:Constraint):Claim {
        val ac = create_constr();
        c.toAgentConstr(ac);
        val clm = invade(Pointer.NULL, ac);
        if (clm == Pointer.NULL)
          throw new NotEnoughResources("iRTSS returned NULL");
        val ret = new AgentClaim(clm);
        ret.setAgentConstr(ac);
        return ret;
    }
*/