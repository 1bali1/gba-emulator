pub struct Bus
{
    bios: [u8; 0x00003fff],
    eWram: [u8; 0x0203ffff-0x02000000],
    iWram: [u8; 0x03007fff-0x03000000]
}

impl Bus
{
    pub fn read8(&self, address: u32) -> u8
    {
        let val = match address
        {
            0x0..=0x00003fff => self.bios[address as usize],
            0x02000000..=0x02ffffff => self.eWram[(address & 0x3ffff) as usize],
            0x03000000..=0x03ffffff => self.iWram[(address & 0x7fff) as usize],
            _ => 0x0
        };

        return val;
    }

    pub fn read16(&self, address: u32) -> u16
    {
        let twoByteAddr = address & !1;

        let byte1 = self.read8(twoByteAddr) as u16;
        let byte2 = self.read8(twoByteAddr.wrapping_add(1)) as u16;

        let val = (byte2 << 8) | byte1;

        return val;
    }

    pub fn read32(&self, address: u32) -> u32
    {
        let fourByteAddr = address & !3; // this should work i guess

        let half1 = self.read16(fourByteAddr) as u32;
        let half2 = self.read16(fourByteAddr.wrapping_add(2)) as u32;

        let val = (half2 << 16) | half1;

        return val;
    }
}