#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate unicode_width;
extern crate unicode_segmentation;

use std::str;
use std::io;
use std::io::Read;
use std::io::Write;
use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;
use self::raw::{nb_enable_raw,nb_disable_raw};

pub mod termio;
pub mod raw;
pub mod history;
mod events;

const CTRL_A:u8 = 1;
const CTRL_B:u8 = 2;
const CTRL_C:u8 = 3;
const CTRL_D:u8 = 4;
const CTRL_E:u8 = 5;
const CTRL_F:u8 = 6;
const CTRL_G:u8 = 7;
const CTRL_H:u8 = 8;
const HT_TAB:u8 = 9;
const CTRL_I:u8 = 9;
const CTRL_J:u8 = 10;
const CTRL_K:u8 = 11;
const CTRL_L:u8 = 12;
const CTRL_M:u8 = 13;
const ENTER :u8= 13;
const CTRL_N:u8 = 14;
const CTRL_O:u8 = 15;
const CTRL_P:u8 = 16;
const CTRL_Q:u8 = 17;
const CTRL_R:u8 = 18;
const CTRL_S:u8 = 19;
const CTRL_T:u8 = 20;
const CTRL_U:u8 = 21;
const CTRL_V:u8 = 22;
const CTRL_W:u8 = 23;
const CTRL_X:u8 = 24;
const CTRL_Y:u8 = 25;
const CTRL_Z:u8 = 26;
const ESCAPE:u8 = 27;
const BACKSPACE:u8 = 127;

const SHIFT_KEY:u8 = '2' as u8;
const CTRL_KEY:u8 = '5' as u8;
const UP_KEY:u8 = 'A' as u8;
const DOWN_KEY:u8 = 'B' as u8;
const RIGHT_KEY:u8 = 'C' as u8;
const LEFT_KEY:u8 = 'D' as u8;
const HOME_KEY:u8 = 'H' as u8;
const END_KEY:u8 = 'F' as u8;
const DELETE_KEY:u8 = '~' as u8;

#[derive(PartialEq, Eq)]
pub enum NbStatus {
    ERROR    = -1,
    SUCCESS  = 0,
    EXIT     = 1,
    INTERRUPT= 2
}

pub struct Nboon {
    pub prompt:String,
    pub prompt_cursor:usize,
    pub buf:Vec<u8>,
    pub buf_idx:usize,
    pub buf_cursor:usize,
    pub paste_line:Vec<u8>,
    pub nbr_rows:usize,
}

fn line_refresh_single(line: &Nboon) {
    // let s = str::from_utf8(&line.buf[..]).unwrap_err();
    let s = match str::from_utf8(&line.buf[..]) {
        Ok(v) => v,
        Err(e) => panic!("Fatal error: {}", e)
    };
    print!("\x1b[0G");
    print!("{}", line.prompt);
    print!("{}", s);
    print!("\x1b[0K\x1b[0G");
    print!("\x1b[{}G", line.prompt_cursor + line.buf_cursor);
    match io::stdout().flush() { // Write immediately
        Ok(v) => v,
        Err(e) => panic!("Write error: {}", e)
    };
}

fn line_insert_utf8(line: &mut Nboon, buf:&[u8], len:usize) {
    // println!("size {}", line.buf_idx);
    for i in 0..len {
        line.buf.insert(line.buf_idx, buf[i]);
        line.buf_idx += 1;
    }
    let s = match str::from_utf8(&buf[..]) {
        Ok(v) => v,
        Err(e) => panic!("Fatal error: {}", e)
    };
    line.buf_cursor += s.width_cjk();
}

fn line_refresh(line: &mut Nboon, buf:&mut[u8], len:usize) {
    buf[len] = 0;
    // println!("key {},{},{},{},{}", buf[0] as char, buf[1] as char, buf[2] as char, buf[3] as char, buf[4] as char);
    // println!("key {},{},{},{},{}", buf[0], buf[1], buf[2], buf[3], buf[4]);
    match buf[0] {
        BACKSPACE | CTRL_H   => events::backspace(line),
        CTRL_B      => events::move_left(line),
        CTRL_F      => events::move_right(line),
        CTRL_A      => events::move_home(line),
        CTRL_E      => events::move_end(line),
        CTRL_L      => events::clear_screen(line),
        CTRL_U      => events::clear_line(line),
        CTRL_K      => events::clear_end_line(line),
        CTRL_P      => history::history_next(),
        CTRL_N      => history::history_prev(),
        CTRL_Y      => events::paste_line(line),
        CTRL_W      => events::del_word(line),
        CTRL_Q      => events::del_path(line),
        CTRL_T      => events::swap_letter(line),
        HT_TAB      => events::tab_evt(line),
        ESCAPE       => {
            match buf[1] {
                91 => { // '[' 
                    match buf[2] {
                        LEFT_KEY    => events::move_left(line),
                        RIGHT_KEY   => events::move_right(line),
                        UP_KEY      => history::history_prev(),
                        DOWN_KEY    => history::history_next(),
                        // HOME_KEY    => events::move_home(line),
                        // END_KEY     => events::move_end(line),
                        // },
                        _           => {
                            if buf[3] == DELETE_KEY {
                                match buf[2] {
                                    51  => events::delete(line), // HOME_KEY
                                    55  => events::move_home(line), // HOME_KEY
                                    56  => events::move_end(line), // END_KEY
                                    _   => () 
                                }
                            }
                        }
                    }
                },
                // 48...57     => { // '0' a '9'
                117 => { // 'O'
                    match buf[2] {
                        HOME_KEY    => events::move_home(line),
                        END_KEY     => events::move_end(line),
                        _           => ()
                    }
                }
                _ => ()
            }
        },
        _           => if buf[0] > 31 { line_insert_utf8(line, buf, len) }
    }
    line_refresh_single(line);
}

fn line_edit(line: &mut Nboon) -> NbStatus {
    let mut buf = [0; 1024];
    print!("{}", line.prompt);
    match io::stdout().flush() { // Write immediately
        Ok(v) => v,
        Err(e) => panic!("Write error: {}", e)
    };
    loop {
        match io::stdin().read(&mut buf) {
            Ok(len) => {
                // println!("working with version: {:?}", len);
                match buf[0] {
                    CTRL_D => if len == 1 { return NbStatus::EXIT; },
                    ENTER | CTRL_J => return NbStatus::SUCCESS,
                    CTRL_C => return NbStatus::INTERRUPT,
                    _ => line_refresh(line, &mut buf, len)
                }
                buf = [0; 1024];
            }
            Err(e) => {
                println!("Error parsing header: {:?}", e);
                return NbStatus::ERROR;
            }
        }
    }
}

pub fn nb_get_line(prompt: &'static str) -> NbStatus {
    if nb_enable_raw() == false {
        return NbStatus::ERROR;
    }
    let mut expand_prompt = String::with_capacity(prompt.len());
    let mut insert = true;
    let mut prompt_cursor = 1;
    for c in prompt.chars() {
        // Insert a char at the end of string
        if c as u8 == 1 {
            insert = false;
        }
        else if c as u8 == 2 {
            insert = true;
        }
        else {
            if insert == true {
                prompt_cursor += c.width_cjk().unwrap_or(0);
            }
            expand_prompt.push(c);
        }
    }
    let mut line:Nboon = Nboon {
        prompt: expand_prompt,
        prompt_cursor: prompt_cursor,
        paste_line: Vec::with_capacity(4096),
        buf: Vec::with_capacity(4096),
        buf_idx: 0,
        buf_cursor: 0,
        nbr_rows: 0
    };
    let ret = line_edit(&mut line);
    nb_disable_raw();
    println!("");
    ret
}


#[test]
fn it_works() {
}
