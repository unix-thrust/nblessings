use std::str;
use unicode_width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation as Uce;
use super::Nboon;
use super::raw::{nb_clear_screen};

/* Move buf_idx in the begining of the previous utf8 character
 * Return the old value of buf_idx
 */
fn move_prev_char(line: &mut Nboon) -> usize {
    let save_idx = line.buf_idx;
    while line.buf_idx != 0 {
        line.buf_idx -= 1;
        if (line.buf[line.buf_idx] & 0xC0) != 0x80 {
            break;
        }
    }
    // println!("idx {} - save_idx {} - diff {}", line.buf_idx,
    //                                      save_idx, save_idx - line.buf_idx);
    // println!("len {}", line.buf.len());
    save_idx
}

/* Move buf_idx in the begining of the next utf8 character
 * Return the old value of buf_idx
 */
fn move_next_char(line: &mut Nboon) -> usize {
    let save_idx = line.buf_idx;
    if line.buf_idx < line.buf.len() {
        line.buf_idx += 1;
    }
    while line.buf_idx < line.buf.len()
        && (line.buf[line.buf_idx] & 0xC0) == 0x80 {
        line.buf_idx += 1;
    }
    // println!("idx {} - save_idx {} - diff {}", line.buf_idx,
    //                                      save_idx, save_idx - line.buf_idx);
    // println!("len {}", line.buf.len());
    save_idx
}

pub fn move_left(line: &mut Nboon) {
    let save_idx = move_prev_char(line);
    let slice = &line.buf[line.buf_idx..save_idx];
    let c = match str::from_utf8(slice) {
        Ok(v) => v,
        Err(e) => panic!("Fatal error: {}", e)
    };
    line.buf_cursor -= Uce::graphemes(c, true).next().unwrap_or("").width_cjk();
}

pub fn move_right(line: &mut Nboon) {
    let save_idx = move_next_char(line);
    let slice = &line.buf[save_idx..line.buf_idx];
    let c = match str::from_utf8(slice) {
        Ok(v) => v,
        Err(e) => panic!("Fatal error: {}", e)
    };
    line.buf_cursor += Uce::graphemes(c, true).next().unwrap_or("").width_cjk();
}

pub fn move_home(line: &mut Nboon) {
    line.buf_idx = 0;
    line.buf_cursor = 0;
}

// TODO
pub fn move_end(line: &mut Nboon) {
    let slice = &line.buf[line.buf_idx..line.buf.len()];
    let s = match str::from_utf8(slice) {
        Ok(v) => v,
        Err(e) => panic!("Fatal error: {}", e)
    };
    line.buf_idx = line.buf.len();
    line.buf_cursor += s.width_cjk();
}

pub fn clear_screen(line: &mut Nboon) {
    nb_clear_screen();
}

pub fn clear_line(line: &mut Nboon) {
    line.buf.clear();
    line.buf_idx = 0;
    line.buf_cursor = 0;
}

pub fn clear_end_line(line: &mut Nboon) {
    line.buf.truncate(line.buf_idx);
}

// Todo, don't insert invalid char
pub fn paste_line(line: &mut Nboon) {
    // print!("{}", line.paste_line);
    for c in line.paste_line.iter() {
        line.buf.insert(line.buf_idx, *c);
        line.buf_idx += 1;
    }
    let s = match str::from_utf8(&line.paste_line[..]) {
        Ok(v) => v,
        Err(e) => panic!("Fatal error: {}", e)
    };
    // line.buf_idx = paste_line.len();
    line.buf_cursor += s.width_cjk();
}

pub fn delete(line: &mut Nboon) {
    let save_idx = move_next_char(line);
    // println!("old {} - new {}", save_idx, line.buf_idx);
    for i in (save_idx..line.buf_idx).rev() {
        line.buf.remove(i);
    }
    line.buf_idx = save_idx;
}
pub fn backspace(line: &mut Nboon) {
    let save_idx = line.buf_idx;
    move_left(line);
    // println!("old {} - new {}", save_idx, line.buf_idx);
    for i in (line.buf_idx..save_idx).rev() {
        line.buf.remove(i);
    }
}

pub fn del_word(line: &mut Nboon) {
    let mut c:char = '\0';
    let save_idx = line.buf_idx;
    line.paste_line.clear();
    while line.buf_idx > 0 && (c != ' ' && c != '\t') {
        move_left(line);
        c = line.buf[line.buf_idx] as char;
    }
    for i in line.buf_idx..save_idx {
        line.paste_line.push(line.buf[line.buf_idx]);
        line.buf.remove(line.buf_idx);
    }
}

pub fn del_path(line: &mut Nboon) {
    let mut c:char = '\0';
    let save_idx = line.buf_idx;
    line.paste_line.clear();
    while line.buf_idx > 0 && (c != ' ' && c != '\t' && c != '/') {
        move_left(line);
        c = line.buf[line.buf_idx] as char;
    }
    for i in line.buf_idx..save_idx {
        line.paste_line.push(line.buf[line.buf_idx]);
        line.buf.remove(line.buf_idx);
    }
}

/*
 * Example:  "RR"
 *            |
 *          prev, idx, line.buf_idx
 */
pub fn swap_letter(line: &mut Nboon) {
    let mut swap: [u8; 5] = [0; 5];
    let prev;
    let save = move_prev_char(line);
    prev = line.buf_idx;
    line.buf_idx = prev;
    let idx = move_next_char(line);
}

pub fn tab_evt(line: &mut Nboon) {

}
