#[derive(Clone, Copy)]
pub struct Core {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    esp: u32,
    eip: u32,
}

#[derive(Clone, Copy)]
pub struct Cursor {
    registers: Core,
}

impl Cursor {
    pub fn up(&self) -> Cursor {
        *self
    }

    pub fn pc(&self) -> u64 {
        self.registers.eip as u64
    }
}
