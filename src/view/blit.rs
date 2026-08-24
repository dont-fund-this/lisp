use slint::{Rgb8Pixel, SharedPixelBuffer};
use vt100::{Color as VColor, Screen};

use crate::view::font::FONT_8X16;

const CW: usize = 8;
const CH: usize = 16;

fn to_rgb(c: VColor, is_bg: bool) -> (u8, u8, u8) {
    match c {
        VColor::Rgb(r, g, b) => (r, g, b),
        VColor::Idx(i) => match i {
            0 => (15, 23, 42),
            1 => (239, 68, 68),
            2 => (34, 197, 94),
            3 => (234, 179, 8),
            4 => (59, 130, 246),
            5 => (168, 85, 247),
            6 => (6, 182, 212),
            7 => (241, 245, 249),
            8 => (71, 85, 105),
            9 => (248, 113, 113),
            10 => (74, 222, 128),
            11 => (250, 204, 21),
            12 => (96, 165, 250),
            13 => (192, 132, 252),
            14 => (34, 211, 238),
            15 => (255, 255, 255),
            _ => (i, i, i),
        },
        VColor::Default => {
            if is_bg {
                (15, 23, 42)
            } else {
                (241, 245, 249)
            }
        }
    }
}

pub fn blit(screen: &Screen, cols: u16, rows: u16, blink: bool) -> SharedPixelBuffer<Rgb8Pixel> {
    let img_w = cols as usize * CW;
    let img_h = rows as usize * CH;
    let mut pixels = SharedPixelBuffer::<Rgb8Pixel>::new(img_w as u32, img_h as u32);
    let slice = pixels.make_mut_slice();

    for cy in 0..rows {
        for cx in 0..cols {
            let cell = screen.cell(cy, cx);
            let sym_owned = cell.map(|c| c.contents()).unwrap_or_default();
            let sym = if sym_owned.is_empty() {
                " "
            } else {
                sym_owned.as_str()
            };

            let mut frgb = to_rgb(cell.map(|c| c.fgcolor()).unwrap_or(VColor::Default), false);
            let mut brgb = to_rgb(cell.map(|c| c.bgcolor()).unwrap_or(VColor::Default), true);

            if let Some(c) = cell {
                if c.inverse() {
                    std::mem::swap(&mut frgb, &mut brgb);
                }
            }

            let px_x = cx as usize * CW;
            let px_y = cy as usize * CH;

            for dy in 0..CH {
                let row_idx = (px_y + dy) * img_w + px_x;
                for dx in 0..CW {
                    slice[row_idx + dx] = Rgb8Pixel {
                        r: brgb.0,
                        g: brgb.1,
                        b: brgb.2,
                    };
                }
            }

            draw_glyph(slice, img_w, px_x, px_y, sym, frgb);
        }
    }

    if blink && !screen.hide_cursor() {
        let (cur_y, cur_x) = screen.cursor_position();
        if cur_x < cols && cur_y < rows {
            let px_x = cur_x as usize * CW;
            let px_y = cur_y as usize * CH;
            let cell = screen.cell(cur_y, cur_x);
            let sym_owned = cell.map(|c| c.contents()).unwrap_or_default();
            let sym = if sym_owned.is_empty() {
                " "
            } else {
                sym_owned.as_str()
            };

            for dy in 0..CH {
                let row_idx = (px_y + dy) * img_w + px_x;
                for dx in 0..CW {
                    slice[row_idx + dx] = Rgb8Pixel {
                        r: 212,
                        g: 163,
                        b: 115,
                    };
                }
            }

            draw_glyph(slice, img_w, px_x, px_y, sym, (15, 23, 42));
        }
    }

    pixels
}

fn set_px(slice: &mut [Rgb8Pixel], img_w: usize, x: usize, y: usize, color: (u8, u8, u8)) {
    let idx = y * img_w + x;
    if idx < slice.len() {
        slice[idx] = Rgb8Pixel {
            r: color.0,
            g: color.1,
            b: color.2,
        };
    }
}

fn draw_glyph(
    slice: &mut [Rgb8Pixel],
    img_w: usize,
    x: usize,
    y: usize,
    sym: &str,
    fg: (u8, u8, u8),
) {
    match sym {
        "─" => {
            for dx in 0..CW {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
        }
        "│" => {
            for dy in 0..CH {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
        }
        "┌" => {
            for dx in 3..CW {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
            for dy in 7..CH {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
        }
        "┐" => {
            for dx in 0..=4 {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
            for dy in 7..CH {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
        }
        "└" => {
            for dx in 3..CW {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
            for dy in 0..=8 {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
        }
        "┘" => {
            for dx in 0..=4 {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
            for dy in 0..=8 {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
        }
        "├" => {
            for dy in 0..CH {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
            for dx in 3..CW {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
        }
        "┤" => {
            for dy in 0..CH {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
            for dx in 0..=4 {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
        }
        "┬" => {
            for dx in 0..CW {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
            for dy in 7..CH {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
        }
        "┴" => {
            for dx in 0..CW {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
            for dy in 0..=8 {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
        }
        "┼" => {
            for dx in 0..CW {
                set_px(slice, img_w, x + dx, y + 7, fg);
                set_px(slice, img_w, x + dx, y + 8, fg);
            }
            for dy in 0..CH {
                set_px(slice, img_w, x + 3, y + dy, fg);
                set_px(slice, img_w, x + 4, y + dy, fg);
            }
        }
        "▶" | "⯈" => {
            for dy in 3..=12 {
                let span = if dy <= 7 { dy - 2 } else { 13 - dy };
                for dx in 1..=span + 1 {
                    set_px(slice, img_w, x + dx + 1, y + dy, fg);
                }
            }
        }
        "⯇" => {
            for dy in 3..=12 {
                let span = if dy <= 7 { dy - 2 } else { 13 - dy };
                for dx in 0..span {
                    set_px(slice, img_w, x + 6 - dx, y + dy, fg);
                }
            }
        }
        "●" => {
            for dy in 5..=10 {
                for dx in 2..=5 {
                    set_px(slice, img_w, x + dx, y + dy, fg);
                }
            }
        }
        _ => {
            if let Some(ch) = sym.chars().next() {
                let code = ch as usize;
                if (32..=126).contains(&code) {
                    let offset = (code - 32) * 16;
                    for dy in 0..16 {
                        let bits = FONT_8X16[offset + dy];
                        for dx in 0..8 {
                            if (bits & (0x80 >> dx)) != 0 {
                                set_px(slice, img_w, x + dx, y + dy, fg);
                            }
                        }
                    }
                }
            }
        }
    }
}
