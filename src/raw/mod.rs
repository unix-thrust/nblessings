use std::mem::transmute;
use std::io;
use std::io::Write;
use libc::funcs::posix88::unistd::isatty;
use libc::consts::os::posix88::STDIN_FILENO;
use termio::{tcgetattr, tcsetattr, Termios};
use termio::types::*;

/* Initialize to random value, not important */
static mut ORIG_TERM:Termios = Termios {
    input_flags: IXON,
    output_flags: OPOST,
    control_flags: CS8,
    local_flags: ECHO,
    _line: 0,
    control_chars: [0; NCCS as usize],
    input_speed: Speed::B0,
    output_speed: Speed::B0
};
static mut g_is_raw:bool = false;

/* cast TCSAFLUSH to i32 */
const _TCSAFLUSH:i32 = When::TCSAFLUSH as i32;

/*
 * Input modes: no break, no CR to NL, no parity check, no strip char,
 * no start/stop output control.
 * Output modes - disable post processing
 * Control modes - set 8 bit chars
 * Local modes - echoing off, canonical off, no extended functions,
 * no signal chars (^Z, ^C)
 * errno = ENOTTY 
*/
pub fn nb_enable_raw() -> bool {
    unsafe {
        let mut raw:Termios;
        if isatty(STDIN_FILENO) == 0
            || tcgetattr(STDIN_FILENO, transmute(&mut ORIG_TERM)) == -1 {
            return false;
        }
        raw = ORIG_TERM;
        raw.input_flags.remove(BRKINT | ICRNL | INPCK | IXON | IUTF8);
        raw.output_flags.remove(OPOST);
        raw.control_flags.insert(CS8);
        raw.local_flags.remove(ECHO | ICANON | IEXTEN);
        raw.control_chars[ControlCharacter::VMIN as usize] = 1;
        raw.control_chars[ControlCharacter::VTIME as usize] = 0;
        if tcsetattr(STDIN_FILENO, _TCSAFLUSH, transmute(&raw)) < 0 {
            return false;
        }
        g_is_raw = true;
    }
    return true;
}

pub fn nb_clear_screen() {
    print!("\x1b[H\x1b[2J");
    match io::stdout().flush() {
        Ok(v) => v,
        Err(e) => panic!("Write error: {}", e)
    };
}

pub fn nb_disable_raw() {
    unsafe {
        if g_is_raw == true
            && tcsetattr(STDIN_FILENO, _TCSAFLUSH, transmute(&ORIG_TERM)) < 0 {
            g_is_raw = false;
        }
    }
}
