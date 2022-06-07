#[derive(Clone, Copy)]
struct VGAPTR {
    chr: *mut u8,
    col: *mut u8,
}
unsafe impl Sync for VGAPTR {}

const VGAHEIGHT: usize = 25;
const VGAWIDTH: usize = 80;
static mut ROW: usize = 0;
static mut COL: usize = 0;

static mut SCREEN: [[VGAPTR; VGAWIDTH]; VGAHEIGHT] = [[VGAPTR {
    chr: (0xB8000 as *mut u8),
    col: (0xB8001 as *mut u8),
}; VGAWIDTH]; VGAHEIGHT];

pub unsafe fn init() {
    for row in 0..VGAHEIGHT - 1 {
        for col in 0..VGAWIDTH - 1 {
            SCREEN[row][col] = VGAPTR {
                chr: (0xB8000 + (row * VGAWIDTH * 2) + (col * 2)) as *mut u8,
                col: (0xB8000 + (row * VGAWIDTH * 2) + (col * 2) + 1) as *mut u8,
            }
        }
    }
}

pub unsafe fn kprintchar(c: char, col: u8) {
    *(SCREEN[ROW][COL].chr) = c as u8;
    *(SCREEN[ROW][COL].col) = col;

    COL += 1;
    if COL == VGAWIDTH {
        COL = 0;
        ROW += 1;
    }
    if ROW == VGAHEIGHT {
        COL = 0;
        ROW = 0;
    }
}

pub fn kprint(s: &str, col: u8) {
    for c in s.chars() {
        unsafe {
            match c {
                '\n' => {
                    COL = 0;
                    ROW += 1;
                }
                _ => kprintchar(c, col),
            }
        }
    }
}
