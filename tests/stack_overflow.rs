#![no_std]
#![feature(abi_x86_interrupt)]
#![no_main]

extern crate lazy_static;
extern crate os;
extern crate x86_64;
use core::panic::PanicInfo;
use os::serial_print;

use self::lazy_static::lazy_static;
use self::x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
  static ref TEST_IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    unsafe {
      idt
        .double_fault
        .set_handler_fn(test_double_fault_handler)
        .set_stack_index(os::gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt
  };
}

pub fn init_test_idt() {
  TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  serial_print!("stack_overflow::stack_overflow...\t");

  os::gdt::init();
  init_test_idt();

  // trigger a stack overflow
  stack_overflow();

  panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
  stack_overflow(); // recurse in order to overflow
  volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  os::test_panic_handler(info)
}

use self::x86_64::structures::idt::InterruptStackFrame;
use os::{exit_qemu, serial_println, QemuExitCode};

extern "x86-interrupt" fn test_double_fault_handler(
  _stack_frame: &mut InterruptStackFrame,
  _error_code: u64,
) -> ! {
  serial_print!("[ok]");
  exit_qemu(QemuExitCode::Success);
  loop {}
}
