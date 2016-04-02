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


struct CFIHeader {
    length: u64,
    entry_id: u64,
    is_64bit: bool,
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
    header: CFIHeader,
    version: u8,
    augmentation: Vec<u8>,
    address_size: u8,
    segment_size: u8,
    code_alignment_factor: ULEB128,
    data_alignment_factor: SLEB128,
    return_address_register: ULEB128,
    initial_instructions: Vec<u8>,
}

impl fmt::Display for CommonInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let augmentation_str = match str::from_utf8(self.augmentation.as_slice()) {
            Ok(s) => s,
            Err(e) => "<invalid string>",
        };

        write!(f,
               "CommonInfo {{\n\t{},\n\tversion: {},\n\taugmentation: \"{}\",\n\taddress_size: \
                {}\n\tsegment_size: {}\n\tcode_alignment_factor: {}\n\tdata_alignment_factor: \
                {}\n\treturn_address_register: {}\n\tinitial_instructions: {:?}\n}}",
               self.header,
               self.version,
               augmentation_str,
               self.address_size,
               self.segment_size,
               self.code_alignment_factor,
               self.data_alignment_factor,
               self.return_address_register,
               self.initial_instructions)
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


impl CFIEntry {
    pub fn from_bytes(reader: &mut io::BufRead) -> io::Result<CFIEntry> {
        let mut reader = DwarfReader(reader);

        let mut is_64bit = false;
        let length: u64 = {
            let initial = try!(reader.read_u32());
            if initial == 0xffffffff {
                is_64bit = true;
                try!(reader.read_u64())
            } else {
                initial as u64
            }
        };

        // Constrain the reader.
        let mut reader = reader.take(length);
        let mut entry_id: u64;
        if is_64bit {
            entry_id = try!(reader.read_u64());
        } else {
            entry_id = try!(reader.read_u32()) as u64;
        }

        let header = CFIHeader {
            length: length,
            entry_id: entry_id,
            is_64bit: is_64bit,
        };

        if entry_id == 0 {
            match CommonInfo::from_bytes(&mut reader, header) {
                Ok(x) => Ok(CFIEntry::CommonInfo(x)),
                Err(e) => Err(e),
            }
        } else {
            match FrameDescription::from_bytes(&mut reader, header) {
                Ok(x) => Ok(CFIEntry::FrameDescription(x)),
                Err(e) => Err(e),
            }
        }
    }
}

impl CommonInfo {
    fn from_bytes<R: io::BufRead>(reader: &mut DwarfReader<R>,
                                  header: CFIHeader)
                                  -> io::Result<CommonInfo> {
        let version = try!(reader.read_u8());
        let augmentation = try!(reader.read_utf8());
        let address_size = try!(reader.read_u8());
        let segment_size = try!(reader.read_u8());
        let code_alignment_factor = try!(reader.read_uleb128());

        let data_alignment_factor = try!(reader.read_sleb128());
        let return_address_register = try!(reader.read_uleb128());
        let mut initial_instructions = Vec::new();
        try!(reader.read_to_end(&mut initial_instructions));

        Ok(CommonInfo {
            header: header,
            version: version,
            augmentation: augmentation,
            address_size: address_size,
            segment_size: segment_size,
            code_alignment_factor: code_alignment_factor,
            data_alignment_factor: data_alignment_factor,
            return_address_register: return_address_register,
            initial_instructions: initial_instructions,
        })
    }
}

impl FrameDescription {
    fn from_bytes<R: io::BufRead>(reader: &mut DwarfReader<R>,
                                  header: CFIHeader)
                                  -> io::Result<FrameDescription> {
        unimplemented!();
    }
}
