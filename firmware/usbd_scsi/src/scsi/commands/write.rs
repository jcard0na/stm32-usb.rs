use packing::Packed;
use crate::scsi::{
    packing::ParsePackedStruct,
    commands::Control,
};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct WriteXCommand {
    pub lba: u32,
    pub transfer_length: u32,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Packed)]
#[packed(big_endian, lsb0)]
pub struct Write6Command {
    #[pkd(7, 0, 0, 0)]
    pub op_code: u8,

    #[pkd(4, 0, 1, 3)]
    pub lba: u32,

    #[pkd(7, 0, 4, 4)]
    pub transfer_length: u8,

    #[pkd(7, 0, 5, 5)]
    pub control: Control,
}
impl ParsePackedStruct for Write6Command {}
impl From<Write6Command> for WriteXCommand {
    fn from(w: Write6Command) -> Self {
        Self {
            lba: w.lba.into(),
            transfer_length: w.transfer_length.into(),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Packed)]
#[packed(big_endian, lsb0)]
pub struct WriteSame10Command {
    #[pkd(7, 0, 0, 0)]
    pub op_code: u8,

    #[pkd(7, 5, 1, 1)]
    pub wr_protect: u8,

    #[pkd(4, 4, 1, 1)]
    pub anchor: bool,

    #[pkd(3, 3, 1, 1)]
    pub unmap: bool,

    #[pkd(7, 0, 2, 5)]
    pub lba: u32,

    #[pkd(4, 0, 6, 6)]
    pub group_number: u8,

    #[pkd(7, 0, 7, 8)]
    pub number_of_blocks: u16,

    #[pkd(7, 0, 9, 9)]
    pub control: Control,
}
impl ParsePackedStruct for WriteSame10Command {}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Packed)]
#[packed(big_endian, lsb0)]
pub struct Write10Command {
    #[pkd(7, 0, 0, 0)]
    pub op_code: u8,
    
    #[pkd(7, 5, 1, 1)]
    pub wr_protect: u8,

    #[pkd(4, 4, 1, 1)]
    pub dpo: bool,
    
    #[pkd(3, 3, 1, 1)]
    pub fua: bool,

    #[pkd(1, 1, 1, 1)]
    pub fua_nv: bool,

    #[pkd(7, 0, 2, 5)]
    pub lba: u32,

    #[pkd(4, 0, 6, 6)]
    pub group_number: u8,

    #[pkd(7, 0, 7, 8)]
    pub transfer_length: u16,

    #[pkd(7, 0, 9, 9)]
    pub control: Control,
}
impl ParsePackedStruct for Write10Command {}
impl From<Write10Command> for WriteXCommand {
    fn from(w: Write10Command) -> Self {
        Self {
            lba: w.lba.into(),
            transfer_length: w.transfer_length.into(),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Packed)]
#[packed(big_endian, lsb0)]
pub struct Write12Command {
    #[pkd(7, 0, 0, 0)]
    pub op_code: u8,
    
    #[pkd(7, 5, 1, 1)]
    pub wr_protect: u8,

    #[pkd(4, 4, 1, 1)]
    pub dpo: bool,
    
    #[pkd(3, 3, 1, 1)]
    pub fua: bool,

    #[pkd(1, 1, 1, 1)]
    pub fua_nv: bool,

    #[pkd(7, 0, 2, 5)]
    pub lba: u32,

    #[pkd(7, 0, 6, 9)]
    pub transfer_length: u32,

    #[pkd(4, 0, 10, 10)]
    pub group_number: u8,

    #[pkd(7, 0, 11, 11)]
    pub control: Control,
}
impl ParsePackedStruct for Write12Command {}
impl From<Write12Command> for WriteXCommand {
    fn from(w: Write12Command) -> Self {
        Self {
            lba: w.lba.into(),
            transfer_length: w.transfer_length.into(),
        }
    }
}