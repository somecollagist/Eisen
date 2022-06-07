#![allow(dead_code)]

pub enum VGACOL {
	BLACK	= 0x0,
	NAVY 	= 0x1,
	GREEN 	= 0x2,
	CYAN 	= 0x3,
	CRIMSON = 0x4,
	MAGENTA = 0x5,
	BROWN 	= 0x6,
	LGREY 	= 0x7,
	DGREY 	= 0x8,
	BLUE 	= 0x9,
	LIME 	= 0xA,
	TEAL 	= 0xB,
	RED 	= 0xC,
	PINK 	= 0xD,
	YELLOW 	= 0xE,
	WHITE 	= 0xF,
}

pub fn getcol(fore: VGACOL, back: VGACOL) -> u8 {
	return ((back as u8) << 4) + (fore as u8);
}

pub const OUT	: u8 = 0x07; //Standard output:		LGREY	on 	BLACK
pub const IN	: u8 = 0x0F; //Standard input:		WHITE	on 	BLACK
pub const QUERY	: u8 = 0x09; //Standard question:	BLUE	on 	BLACK
pub const ERR	: u8 = 0x0C; //Standard error:		RED		on 	BLACK
pub const OK	: u8 = 0x02; //Standard ok:			LIME	on 	BLACK
pub const PROC	: u8 = 0x0E; //Standard process:	YELLOW	on 	BLACK
