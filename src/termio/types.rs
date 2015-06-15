// Implement /usr/include/bits/termios.h in rust

pub const NCCS:u32   = 32;

/* c_iflags bits */
const _IGNBRK:u32    = 0o000001;
const _BRKINT:u32    = 0o000002;
const _IGNPAR:u32    = 0o000004;
const _PARMRK:u32    = 0o000010;
const _INPCK:u32     = 0o000020;
const _ISTRIP:u32    = 0o000040;
const _INLCR:u32     = 0o000100;
const _IGNCR:u32     = 0o000200;
const _ICRNL:u32     = 0o000400;
const _IUCLC:u32     = 0o001000;
const _IXON:u32      = 0o002000;
const _IXANY:u32     = 0o004000;
const _IXOFF:u32     = 0o010000;
const _IMAXBEL:u32   = 0o020000;
const _IUTF8:u32     = 0o040000;

/* c_oflags bits */
const _OPOST:u32     = 0o000001;
const _OLCUC:u32     = 0o000002;
const _ONLCR:u32     = 0o000004;
const _OCRNL:u32     = 0o000010;
const _ONOCR:u32     = 0o000020;
const _ONLRET:u32    = 0o000040;
const _OFILL:u32     = 0o000100;
const _OFDEL:u32     = 0o000200;

const _NL0:u32       = 0o000000;
const _NL1:u32       = 0o000400;
const _CR0:u32       = 0o000000;
const _CR1:u32       = 0o001000;
const _CR2:u32       = 0o002000;
const _CR3:u32       = 0o003000;
const _TAB0:u32      = 0o000000;
const _TAB1:u32      = 0o004000;
const _TAB2:u32      = 0o010000;
const _TAB3:u32      = 0o014000;
const _BS0:u32       = 0o000000;
const _BS1:u32       = 0o020000;
const _FF0:u32       = 0o000000;
const _FF1:u32       = 0o100000;

const _VT0:u32       = 0o000000;
const _VT1:u32       = 0o040000;

/* c_cflags bit meaning */
const _CBAUD:u32     = 0o010017;
const _CBAUDEX:u32   = 0o010000;

const _CSIZE:u32     = 0o000060;
const _CS5:u32       = 0o000000;
const _CS6:u32       = 0o000020;
const _CS7:u32       = 0o000040;
const _CS8:u32       = 0o000060;

const _CSTOPB:u32    = 0o000100;
const _CREAD:u32     = 0o000200;
const _PARENB:u32    = 0o000400;
const _PARODD:u32    = 0o001000;
const _HUPCL:u32     = 0o002000;
const _CLOCAL:u32    = 0o004000;

const _CIBAUD:u32    = 0o02003600000;     /* input baud rate  (not used) */
const _CMSPAR:u32    = 0o10000000000;     /* mark or space (stick) parity */
const _CRTSCTS:u32   = 0o20000000000;     /* flow control */

/* c_lflags bits */
const _ISIG:u32      = 0o000001;
const _ICANON:u32    = 0o000002;
const _XCASE:u32     = 0o000004;
const _ECHO:u32      = 0o000010;
const _ECHOE:u32     = 0o000020;
const _ECHOK:u32     = 0o000040;
const _ECHONL:u32    = 0o000100;
const _NOFLSH:u32    = 0o000200;
const _TOSTOP:u32    = 0o000400;
const _ECHOCTL:u32   = 0o001000;
const _ECHOPRT:u32   = 0o002000;
const _ECHOKE:u32    = 0o004000;
const _FLUSHO:u32    = 0o010000;
const _PENDIN:u32    = 0o040000;
const _IEXTEN:u32    = 0o100000;
const _EXTPROC:u32   = 0o200000;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ControlCharacter {
  VINTR     =  0,
  VQUIT     =  1,
  VERASE    =  2,
  VKILL     =  3,
  VEOF      =  4,
  VTIME     =  5,
  VMIN      =  6,
  VSWTC     =  7,
  VSTART    =  8,
  VSTOP     =  9,
  VSUSP     = 10,
  VEOL      = 11,
  VREPRINT  = 12,
  VDISCARD  = 13,
  VWERASE   = 14,
  VLNEXT    = 15,
  VEOL2     = 16,
}

/* tcsetattr uses these */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum When {
  TCSANOW   = 0,
  TCSADRAIN = 1,
  TCSAFLUSH = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Speed {
  B0        = 0o000000,     /* hang up */
  B50       = 0o000001,
  B75       = 0o000002,
  B110      = 0o000003,
  B134      = 0o000004,
  B150      = 0o000005,
  B200      = 0o000006,
  B300      = 0o000007,
  B600      = 0o000010,
  B1200     = 0o000011,
  B1800     = 0o000012,
  B2400     = 0o000013,
  B4800     = 0o000014,
  B9600     = 0o000015,
  B19200    = 0o000016,
  B38400    = 0o000017,
  B57600    = 0o010001,
  B115200   = 0o010002,
  B230400   = 0o010003,
  B460800   = 0o010004,
  B500000   = 0o010005,
  B576000   = 0o010006,
  B921600   = 0o010007,
  B1000000  = 0o010010,
  B1152000  = 0o010011,
  B1500000  = 0o010012,
  B2000000  = 0o010013,
  B2500000  = 0o010014,
  B3000000  = 0o010015,
  B3500000  = 0o010016,
  B4000000  = 0o010017
}

bitflags! {
  flags InputFlags: u32 {
    // Ignore BREAK condition on input.
    const IGNBRK  = _IGNBRK,
    // If  IGNBRK is set, a BREAK is ignored. If it is not set but BRKINT is set, then a BREAK
    // causes the input and output queues to be flushed, and if the terminal is the controlling
    // terminal of a foreground process group, it will cause a SIGINT to be sent to this foreground
    // process group. When neither IGNBRK nor BRKINT are set, a BREAK reads as a null byte ('\0'),
    // except when PARMRK is set, in which case it reads as the sequence \377 \0 \0.
    const BRKINT  = _BRKINT,
    // Ignore framing errors and parity errors.
    const IGNPAR  = _IGNPAR,
    // If IGNPAR is not set, prefix a character with a parity error or framing error with \377 \0.
    // If neither IGNPAR nor PARMRK is set, read a character with a parity error or framing error
    // as \0.
    const PARMRK  = _PARMRK,
    // Enable input parity checking.
    const INPCK   = _INPCK,
    // Strip off eighth bit.
    const ISTRIP  = _ISTRIP,
    // Translate NL to CR on input.
    const INLCR   = _INLCR,
    // Ignore carriage return on input.
    const IGNCR   = _IGNCR,
    // Translate carriage return to newline on input (unless IGNCR is set).
    const ICRNL   = _ICRNL,
    // (not in POSIX) Map uppercase characters to lowercase on input.
    const IUCLC   = _IUCLC,
    // Enable XON/XOFF flow control on output.
    const IXON    = _IXON,
    // (XSI) Typing any character will restart stopped output.  (The default is to allow just the
    // START character to restart output.)
    const IXANY   = _IXANY,
    // Enable XON/XOFF flow control on input.
    const IXOFF   = _IXOFF,
    // (not in POSIX) Ring bell when input queue is full.  Linux does not implement this bit, and
    // acts as if it is always set.
    const IMAXBEL = _IMAXBEL,
    // (since Linux 2.6.4) (not in POSIX) Input is UTF8; this allows character-erase to be
    // correctly performed in cooked mode.
    const IUTF8   = _IUTF8
  }
}

bitflags! {
  flags OutputFlags: u32 {
    // Enable implementation-defined output processing.
    const OPOST  = _OPOST,
    // (not in POSIX) Map lowercase characters to uppercase on output.
    const OLCUC  = _OLCUC,
    // (XSI) Map NL to CR-NL on output.
    const ONLCR  = _ONLCR,
    // Map CR to NL on output.
    const OCRNL  = _OCRNL,
    // Don't output CR at column 0.
    const ONOCR  = _ONOCR,
    // Don't output CR.
    const ONLRET = _ONLRET,

    // Send fill characters for a delay, rather than using a timed delay.
    const OFILL  = _OFILL,
    // Fill character is ASCII DEL (0177).  If unset, fill character is ASCII NUL ('\0').  (Not implemented on Linux.)
    const OFDEL  = _OFDEL,

    // newline delay
    const NL0    = _NL0,
    const NL1    = _NL1,

    // carriage return delay
    const CR0    = _CR0,
    const CR1    = _CR1,
    const CR2    = _CR2,
    const CR3    = _CR3,

    // tab delay
    const TAB0   = _TAB0,
    const TAB1   = _TAB1,
    const TAB2   = _TAB2,
    const TAB3   = _TAB3,

    // backspace delay
    const BS0    = _BS0,
    const BS1    = _BS1,

    // form feed delay
    const FF0    = _FF0,
    const FF1    = _FF1,

    // vertical tab delay
    const VT0    = _VT0,
    const VT1    = _VT1
  }
}

bitflags! {
  flags ControlFlags: u32 {
    // character size
    const CS5 = _CS5,
    const CS6 = _CS6,
    const CS7 = _CS7,
    const CS8 = _CS8,

    // Set two stop bits, rather than one.
    const CSTOPB = _CSTOPB,
    // Enable receiver.
    const CREAD = _CREAD,
    // Enable parity generation on output and parity checking for input.
    const PARENB = _PARENB,
    // If set, then parity for input and output is odd; otherwise even parity is used.
    const PARODD = _PARODD,
    // Lower modem control lines after last process closes the device (hang up).
    const HUPCL = _HUPCL,
    // Ignore modem control lines.
    const CLOCAL = _CLOCAL,

    // baud
    const CBAUD = _CBAUD,
    // extended baud
    const CBAUDEX = _CBAUDEX,

    // (not  in  POSIX) Mask for input speeds.  The values for the CIBAUD bits are the same as the
    // values for the CBAUD bits, shifted left IBSHIFT bits. (Not implemented on Linux.)
    const CIBAUD = _CIBAUD,

    // (not in POSIX) Use "stick" (mark/space) parity (supported on certain serial devices): if
    // PARODD is set, the parity bit is always 1; if PARODD is not set, then the parity  bit  is
    // always  0.
    const CMSPAR = _CMSPAR,
    // (not in POSIX) Enable RTS/CTS (hardware) flow control.
    const CRTSCTS = _CRTSCTS
  }
}

bitflags! {
  flags LocalFlags: u32 {
    // When any of the characters INTR, QUIT, SUSP, or DSUSP are received, generate the
    // corresponding signal.
    const ISIG = _ISIG,
    // Enable canonical mode
    const ICANON = _ICANON,
    // (not  in  POSIX; not supported under Linux) If ICANON is also set, terminal is uppercase
    // only.  Input is converted to lowercase, except for characters preceded by \.  On output,
    // uppercase characters are preceded by \ and lowercase characters are converted to uppercase
    const XCASE = _XCASE,
    // Echo input characters.
    const ECHO = _ECHO,
    // If ICANON is also set, the ERASE character erases the preceding input character, and WERASE
    // erases the preceding word.
    const ECHOE = _ECHOE,
    // If ICANON is also set, the KILL character erases the current line.
    const ECHOK = _ECHOK,
    // If ICANON is also set, echo the NL character even if ECHO is not set.
    const ECHONL = _ECHONL,
    // Disable flushing the input and output queues when generating signals for the INT, QUIT, and
    // SUSP characters.
    const NOFLSH = _NOFLSH,
    // Send the SIGTTOU signal to the process group of a background process which tries to write to
    // its controlling terminal.
    const TOSTOP = _TOSTOP,
    // (not in POSIX) If ECHO is also set, terminal special characters other than TAB, NL, START,
    // and STOP are echoed as ^X, where X is the character with ASCII code 0x40 greater than the
    // special character. For example, character 0x08 (BS) is echoed as ^H.
    const ECHOCTL = _ECHOCTL,
    // (not in POSIX) If ICANON and ECHO are also set, characters are printed as they are being
    // erased.
    const ECHOPRT = _ECHOPRT,
    // (not in POSIX) If ICANON is also set, KILL is echoed by erasing each character on the line,
    // as specified by ECHOE and ECHOPRT.
    const ECHOKE = _ECHOKE,
    // (not in POSIX; not supported under Linux) Output is being flushed.  This flag is toggled by
    // typing the DISCARD character.
    const FLUSHO = _FLUSHO,
    // (not in POSIX; not supported under Linux) All characters in the input queue are reprinted
    // when the next character is read.  (bash(1) handles typeahead this way.)
    const PENDIN = _PENDIN,
    // Enable  implementation-defined  input  processing.   This  flag, as well as ICANON must be
    // enabled for the special characters EOL2, LNEXT, REPRINT, WERASE to be interpreted, and for
    // the IUCLC flag to be effective.
    const IEXTEN = _IEXTEN

    /* const EXTPROC = _EXTPROC, */
  }
}
