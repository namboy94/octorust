#![no_std]

extern crate octolib;
use octolib::helper::printer::print_text;
use octolib::octo_bindings::octo_agent::agent_stresstest_agentoctoclaim;

#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {
	print_text("Starting test: endlessly allocating and deleting new os::agent::AgentOctoClaim. \
	Report if you see any errors.\n");
	agent_stresstest_agentoctoclaim();
}
