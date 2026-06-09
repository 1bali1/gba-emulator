pub struct Registers;

impl Registers
{
    pub const r0: usize = 0;
    pub const r2: usize = 1;
    pub const r3: usize = 2;
    pub const r4: usize = 3;
    pub const r5: usize = 4;
    pub const r6: usize = 5;
    pub const r7: usize = 6;
    pub const r8: usize = 7;
    pub const r9: usize = 8;
    pub const r10: usize = 9;
    pub const r11: usize = 10;
    pub const r12: usize = 11;

    pub const sp: usize = 12;
    pub const lr: usize = 13;
    pub const pc: usize = 14;
}