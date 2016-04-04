use std::fmt;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::str;
use dwarf::cfi::*;
use dwarf::reader::DwarfReader;

pub fn read_cfi_entry(reader: &mut io::BufRead) -> io::Result<CFIEntry> {
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
        match read_common_info(&mut reader, header) {
            Ok(x) => Ok(CFIEntry::CommonInfo(x)),
            Err(e) => Err(e),
        }
    } else {
        match read_frame_description(&mut reader, header) {
            Ok(x) => Ok(CFIEntry::FrameDescription(x)),
            Err(e) => Err(e),
        }
    }
}

fn read_common_info<R: io::BufRead>(reader: &mut DwarfReader<R>,
                                    header: CFIHeader)
                                    -> io::Result<CommonInfo> {
    let version = try!(reader.read_u8());
    let augmentation = try!(reader.read_utf8());
    if augmentation.len() >= 2 {
        if augmentation[0] == 'e' as u8 && augmentation[1] == 'h' as u8 {
            panic!("unimplemented .eh_frame augmentation 'eh'");
        }
    }

    let code_alignment_factor = try!(reader.read_uleb128());
    let data_alignment_factor = try!(reader.read_sleb128());
    let return_address_register = try!(reader.read_uleb128());

    let augmentation_data = if augmentation.len() > 0 {
        let augmentation_str = try!(str::from_utf8(augmentation.as_slice()).map_err(|err| {
            io::Error::new(ErrorKind::InvalidInput, err)
        }));

        if augmentation[0] != 'z' as u8 {
            panic!("unexpected .eh_frame augmentation string: {}",
                   augmentation_str)
        }

        let augmentation_data_length = try!(reader.read_uleb128());
        let mut data = Vec::with_capacity(augmentation_data_length as usize);
        try!(reader.read_exact(data.as_mut_slice()));
        println!("read {} bytes", augmentation_data_length);
        data
    } else {
        Vec::new()
    };

    let mut initial_instructions = Vec::new();
    try!(reader.read_to_end(&mut initial_instructions));

    Ok(CommonInfo {
        header: header,
        version: version,
        augmentation: augmentation,
        augmentation_data: augmentation_data,
        code_alignment_factor: code_alignment_factor,
        data_alignment_factor: data_alignment_factor,
        return_address_register: return_address_register,
        initial_instructions: initial_instructions,
    })
}

fn read_frame_description<R: io::BufRead>(reader: &mut DwarfReader<R>,
                                          header: CFIHeader)
                                          -> io::Result<FrameDescription> {
    unimplemented!();
}
