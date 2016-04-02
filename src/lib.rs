mod arm;
mod x86;
pub mod dwarf;

#[cfg(test)]
mod test;

enum Cursor {
    ARM(arm::Cursor),
    X86(x86::Cursor),
}

macro_rules! arch_forward {
    ($name: ident, $return_type: ident, ($( $args: ident ), *)) => {
        fn $name(&self, $(, $args)*) -> $return_type {
            match *self {
                Cursor::ARM(ref arm) => Cursor::ARM(arm.$ident()),
                Cursor::X86(ref x86) => Cursor::X86(x86.$ident())
            }
        }
    }
}

impl Cursor {
    fn up(&self) -> Cursor {
        match *self {
            Cursor::ARM(ref arm) => Cursor::ARM(arm.up()),
            Cursor::X86(ref x86) => Cursor::X86(x86.up()),
        }
    }

    fn pc(&self) -> u64 {
        match *self {
            Cursor::ARM(ref arm) => arm.pc(),
            Cursor::X86(ref x86) => x86.pc(),
        }
    }
}
