#[macro_use]
pub mod vga;

#[cfg(target_arch="x86_64")]
mod x86_64;
type CPU = x86_64::CPU;

// Initialize the architecture-specific features.
pub fn init(boot_info_struct: usize) -> CPU {
	let cpu = CPU::init(boot_info_struct);
	
	cpu
}

#[inline(always)]
pub fn halt() {
	unsafe { CPU::halt(); }
}