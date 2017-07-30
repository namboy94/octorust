// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;
use octolib::improvements::constraints::Constraints;
use octolib::improvements::claim::AgentClaim;
use octolib::octo_types::*;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {


}

fn do_test() {

	let mut constraints = Constraints::new();
	constraints.set_pe_quantity(8, 8);

	let mut claim = Claim::new(constraints);

}

pub extern fn ilet(param: *mut c_void) {

}

pub extern fn reduce(x: i32, y: i32) -> i32 {
	return x + y;
}

/*

ORIGINAL

public class MapReduce {
	private static def do_test() {
		val constraints = new PEQuantity(8,8) && new PEQuantity(8,8);
		val claim = Claim.invade(constraints);

		val ilet = (id:IncarnationID):int => id.ordinal;
		val reduce = (x:int, y:int) => x+y;

		val result = claim.infect[int](ilet);
		val sum    = claim.infect[int](ilet,reduce);
		claim.retreat();

		Console.OUT.println("values: "+result);
		Console.OUT.println("sum:    "+sum);
	}

	public static def main(args: Array[String]) {
		for (i in 0..20) {
			do_test();
		}
	}
}



*/
