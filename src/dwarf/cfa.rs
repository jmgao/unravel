use dwarf::reader::DwarfReader;
use std::io;
use std::io::ErrorKind;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum CFA {
    DW_CFA_advance_loc {
        delta: u32,
    },

    DW_CFA_offset {
        register: u64,
        offset: u64,
    },

    DW_CFA_restore {
        register: u64,
    },

    DW_CFA_nop,

    DW_CFA_set_loc {
        address: u64,
    },

    // DW_CFA_advance_loc1,2,4 folded into DW_CFA_advance_loc
    DW_CFA_offset_extended,
    DW_CFA_restore_extended,
    DW_CFA_undefined,
    DW_CFA_same_value,
    DW_CFA_register,
    DW_CFA_remember_state,
    DW_CFA_restore_state,

    DW_CFA_def_cfa {
        register: u64,
        offset: u64,
    },

    DW_CFA_def_cfa_register {
        register: u64,
    },

    DW_CFA_def_cfa_offset {
        offset: u64,
    },

    DW_CFA_def_cfa_expression,
    DW_CFA_expression,
    DW_CFA_offset_extended_sf,
    DW_CFA_def_cfa_sf,
    DW_CFA_def_cfa_offset_sf,
    DW_CFA_val_offset,
    DW_CFA_val_offset_sf,
    DW_CFA_val_expression,
    DW_CFA_lo_user,
    DW_CFA_hi_user,
}

impl CFA {
    pub fn read_instruction<R: io::BufRead>(reader: &mut DwarfReader<R>) -> io::Result<Option<CFA>> {
        let opcode = match reader.read_u8() {
            Ok(x) => x,
            Err(err) => {
                if err.kind() == ErrorKind::UnexpectedEof {
                    return Ok(None);
                } else {
                    return Err(err);
                }
            }
        };

        Ok(Some({
            let high = opcode >> 6;
            let low = opcode & 0b00111111;
            match high {
                1 => CFA::DW_CFA_advance_loc { delta: low as u32 },
                2 => {
                    CFA::DW_CFA_offset {
                        register: low as u64,
                        offset: try!(reader.read_uleb128()),
                    }
                }
                3 => CFA::DW_CFA_restore { register: low as u64 },
                _ => {
                    match low {
                        0 => CFA::DW_CFA_nop,
                        0xc => {
                            let register = try!(reader.read_uleb128());
                            let offset = try!(reader.read_uleb128());
                            CFA::DW_CFA_def_cfa {
                                register: register,
                                offset: offset,
                            }
                        }
                        _ => {
                            let error_msg = format!("Unexpected opcode {:#02x} ({}, {:#02x})",
                                                    opcode,
                                                    high,
                                                    low);

                            return Err(io::Error::new(ErrorKind::InvalidData, error_msg));
                        }
                    }
                }
            }
        }))
    }
}
