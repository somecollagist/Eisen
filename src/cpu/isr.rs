#[allow(dead_code)]

use crate::drivers::screen::{print, colours};

#[no_mangle]
pub extern "C" fn isr_handler(){
	print::kprint("idr handler", colours::ERR);
}