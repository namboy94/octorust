// Author: Hermann Krumrey 2017 <hermann@krumreyh.com>
#![no_std]

extern crate octolib;

extern {
	fn printf(s: *const u8, ...);
	fn handle_closure(func: i32, params: *mut c_void);
}
use core::ptr;
use core::mem;
use octolib::octo_types::c_void;
use octolib::octo_types::c_int;
use octolib::octo_guest::shutdown;
use octolib::improvements::functions::reply_signal;
use octolib::improvements::claim::AgentClaim;
use octolib::improvements::constraints::Constraints;


#[no_mangle]
pub extern "C" fn rust_main_ilet(claim: u8) {

	fn ilet(params: *mut c_void) {
		unsafe { printf("Hello World\n\0".as_ptr()) }
		// reply_signal(params);
	}

	let mut x = 1;

	let func = |params: *mut c_void| {
		unsafe { printf("Hello World\n\0".as_ptr()) }
		//reply_signal(params);
	};



	unsafe {
		let mut closure = |param: i32| -> bool { param == 1 };
		do_with_callback(0, closure);
		//handle_closure(closure, ptr::null_mut());

		//let mut clos = |param: i32| -> bool { param == 1 };
		//run(clos);

		// fn x(i: i32, n: *mut c_void) -> c_int { return 1; }
		// do_something(x, ptr::null_mut());



	}


	/*
	let constr = Constraints::new(3, 4);
	let mut claim = AgentClaim::new(constr);
	claim.set_verbose(true);

	claim.infect(func);
	claim.reinvade();

	claim.infect(ilet);
	claim.reinvade_with_constraints(Constraints::new(6, 7));

	claim.infect(ilet);
	// Implicit retreat
	// Shutdown handled by octorust
	*/
}

/*
extern "C" {
    fn do_something(f: Option<extern "C" fn(x: *mut c_void, arg: *mut c_void)>,
					param: *mut c_void);
}

extern "C" fn do_something_handler(x: *mut c_void, arg: *mut c_void) {
    let closure: &mut &mut FnMut(*mut c_void) = unsafe { mem::transmute(arg) };
    closure(x)
}

fn do_with_callback<F>(mut closure: F, x: *mut c_void) where F: FnMut(*mut c_void) {
    let callback: &mut FnMut(*mut c_void) = &mut closure;
    unsafe {
		do_something(Some(do_something_handler), callback as *mut _ as *mut c_void)
	}
}
*/


extern "C" {
    fn do_something(f: Option<extern "C" fn(x: c_int, arg: *mut c_void) -> c_int>, arg: *mut c_void) -> c_int;
}

extern "C" fn do_something_handler(x: c_int, arg: *mut c_void) -> c_int {
    let closure: &mut &mut FnMut(i32) -> bool = unsafe { mem::transmute(arg) };
    closure(x as i32) as c_int
}

pub fn do_with_callback<F>(x: i32, mut callback: F) -> bool
    where F: FnMut(i32) -> bool
{
    let mut cb: &mut FnMut(i32) -> bool = &mut callback;
	let ctx = &mut cb as *mut &mut FnMut(i32) -> bool as *mut c_void;
    let cb2: *mut *mut FnMut(i32) -> bool = unsafe { mem::transmute(ctx) };
    let clos: &mut &mut FnMut(i32) -> bool = unsafe { mem::transmute(ctx) };
    unsafe { do_something(Some(do_something_handler), clos as *mut _ as *mut c_void) > 0 }
}


pub fn run<F>(mut callback: F) -> bool
    where F: FnMut(i32) -> bool
{
    let mut cb: &mut FnMut(i32) -> bool = &mut callback;
    let ctx = &mut cb as *mut &mut FnMut(i32) -> bool as *mut c_void;
    let cb2: *mut *mut FnMut(i32) -> bool = unsafe { mem::transmute(ctx) };
    let closure: &mut &mut FnMut(i32) -> bool = unsafe { mem::transmute(ctx) };
    closure(0xDEAD)
}