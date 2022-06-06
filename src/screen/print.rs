static mut SCREEN: *mut u8 = 0xB8000 as _;

pub fn kprintchar(c: char, col: u8)
{
	unsafe
	{
		*SCREEN.offset(0) = c as u8;
		*SCREEN.offset(1) = col;
		SCREEN = SCREEN.offset(2);
	}
}

pub fn kprint(s: &str, col: u8)
{
	for c in s.chars()
	{
		kprintchar(c, col);
	}
}