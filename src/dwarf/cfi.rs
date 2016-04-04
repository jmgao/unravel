use std::fmt;
use std::io;
use std::io::Read;
use std::str;
use dwarf::reader::DwarfReader;

struct Register(u64);
struct DwarfExpression;

enum RegisterRules {
    Undefined,
    SameValue,
    Offset(i64),
    ValOffset(i64),
    Register(Register),
    Expression(DwarfExpression),
    ValExpression(DwarfExpression),
}

type ULEB128 = u64;
type SLEB128 = i64;


pub enum CFIEntry {
    CommonInfo(CommonInfo),
    FrameDescription(FrameDescription),
}

impl fmt::Display for CFIEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CFIEntry::CommonInfo(ref x) => write!(f, "{}", x),
            &CFIEntry::FrameDescription(ref x) => write!(f, "{}", x),
        }
    }
}


pub struct CFIHeader {
    pub length: u64,
    pub entry_id: u64,
    pub is_64bit: bool,
}

impl fmt::Display for CFIHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "CFIHeader {{ length: {}, entry_id: {}, 64-bit: {} }}",
               self.length,
               self.entry_id,
               self.is_64bit)
    }
}


pub struct CommonInfo {
    pub header: CFIHeader,
    pub version: u8,
    pub augmentation: Vec<u8>,
    pub augmentation_data: Vec<u8>,
    pub code_alignment_factor: u64,
    pub data_alignment_factor: i64,
    pub return_address_register: u64,
    pub initial_instructions: Vec<u8>,
}

impl fmt::Display for CommonInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let augmentation_str = match str::from_utf8(self.augmentation.as_slice()) {
            Ok(s) => s,
            Err(e) => "<invalid string>",
        };

        write!(f, "CommonInfo {{");
        write!(f, "\n\tversion: {}", self.version);
        write!(f, "\n\taugmentation: {}", augmentation_str);
        write!(f, "\n\taugmentation data: {:?}", self.augmentation_data);
        write!(f, "\n\tcode_alignment_factor: {}", self.code_alignment_factor);
        write!(f, "\n\tdata_alignment_factor: {}", self.data_alignment_factor);
        write!(f, "\n\treturn_address_register: {}", self.return_address_register);
        write!(f, "\n\tinitial instructions: {:?}", self.initial_instructions);
        write!(f, "\n}}")
    }
}


pub struct FrameDescription {
    header: CFIHeader,
}

impl fmt::Display for FrameDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FrameDescription {{\n\t{}\n}}", self.header)
    }
}
