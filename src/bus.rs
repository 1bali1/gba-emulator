pub struct Bus
{
    bios: [u32; 0x00003fff],
    eWram: [u16; 0x0203ffff-0x02000000],
    iWram: [u32; 0x03007fff-0x03000000]
}

impl Bus
{
    pub fn read8(&self, address: u32) -> u8
    {
        return 0;
    }
}