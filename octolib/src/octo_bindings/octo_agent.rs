extern {
    #[link_name="agent_stresstest_agentclaim"]
    fn __agent_stresstest_agentclaim();

    #[link_name="agent_stresstest_agentoctoclaim"]
    fn __agent_stresstest_agentoctoclaim();
}

/// Testing function only! Do not use! Undocumented for a good reason.
pub fn agent_stresstest_agentclaim() { unsafe { __agent_stresstest_agentclaim() } }

/// Testing function only! Do not use! Undocumented for a good reason.
pub fn agent_stresstest_agentoctoclaim() { unsafe { __agent_stresstest_agentoctoclaim() } }
