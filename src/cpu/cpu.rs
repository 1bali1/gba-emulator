const PC: u8 = 14;
const LR: u8 = 15;
const SP: u8 = 12;

pub struct CPU
{
    mode: u8,
    registers: [u32; 15],
    pipeDecode: u32,
    pipeFetch: u32
}

impl CPU
{
    pub fn new() -> Self
    {
        let cpu = Self
        {
            mode: 0,
            registers: [0; 15],
            pipeDecode: 0,
            pipeFetch: 0
        };

        return cpu;
    }

    fn step()
    {
        
    }
}