#[allow(improper_ctypes)]

use self::types::*;
pub mod types;

/* extern C library function */
extern {
    pub fn tcgetattr(fd:i32, termios: *mut Termios) -> i32;
    pub fn tcsetattr(fd:i32, action:i32, termios: *mut Termios) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Termios {
    pub input_flags: InputFlags,
    pub output_flags: OutputFlags,
    pub control_flags: ControlFlags,
    pub local_flags: LocalFlags,
    pub _line: u8, // line discipline (unused on POSIX)
    pub control_chars: [u8; NCCS as usize],
    pub input_speed: Speed,
    pub output_speed: Speed,
}
