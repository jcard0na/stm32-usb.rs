use packing::Packed;

use usbd_bulk_only_transport::CommandBlockWrapper as CommandBlockWrapper_NEW;
use crate::scsi::{
    commands::*,
    enums::*,
    Error,
    packing::ParsePackedStruct,
};

/// A fully parsed and validated SCSI command
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Command {
    None,
    Inquiry(InquiryCommand),
    TestUnitReady(TestUnitReadyCommand),
    ReadCapacity(ReadCapacity10Command),
    ModeSense(ModeSenseXCommand),
    PreventAllowMediumRemoval(PreventAllowMediumRemovalCommand),
    RequestSense(RequestSenseCommand),
    Read(ReadXCommand),
    Write(WriteXCommand),
    Format(FormatCommand),
    SendDiagnostic(SendDiagnosticCommand),
    ReportLuns(ReportLunsCommand),
    ModeSelect(ModeSelectXCommand),
    StartStopUnit(StartStopUnitCommand),
    ReadFormatCapacities(ReadFormatCapacitiesCommand),
    Verify(Verify10Command),
    SynchronizeCache(SynchronizeCache10Command),
}

impl Command {
    pub fn extract_from_cbw(cbw: &CommandBlockWrapper_NEW) -> Result<Command, Error> {
        let op_code = OpCode::from_primitive(cbw.data[0]).map_err(|_| Error::UnhandledOpCode)?;
        match op_code {
            OpCode::Read6 => Ok(Command::Read(checked_extract::<Read6Command>(cbw)?.into())),
            OpCode::Read10 => Ok(Command::Read(checked_extract::<Read10Command>(cbw)?.into())),
            OpCode::Read12 => Ok(Command::Read(checked_extract::<Read12Command>(cbw)?.into())),
            OpCode::ReadCapacity10 => Ok(Command::ReadCapacity(checked_extract(cbw)?)), 
            OpCode::ReadFormatCapacities => Ok(Command::ReadFormatCapacities(checked_extract(cbw)?)),
            OpCode::Inquiry => Ok(Command::Inquiry(checked_extract(cbw)?)),
            OpCode::TestUnitReady => Ok(Command::TestUnitReady(checked_extract(cbw)?)),
            OpCode::ModeSense6 => Ok(Command::ModeSense(checked_extract::<ModeSense6Command>(cbw)?.into())),
            OpCode::ModeSense10 => Ok(Command::ModeSense(checked_extract::<ModeSense10Command>(cbw)?.into())),
            OpCode::ModeSelect6 => Ok(Command::ModeSelect(checked_extract::<ModeSelect6Command>(cbw)?.into())),
            OpCode::ModeSelect10 => Ok(Command::ModeSelect(checked_extract::<ModeSelect10Command>(cbw)?.into())),
            OpCode::PreventAllowMediumRemoval => Ok(Command::PreventAllowMediumRemoval(checked_extract(cbw)?)),
            OpCode::RequestSense => Ok(Command::RequestSense(checked_extract(cbw)?)),
            OpCode::Write6 => Ok(Command::Write(checked_extract::<Write6Command>(cbw)?.into())),
            OpCode::Write10 => Ok(Command::Write(checked_extract::<Write10Command>(cbw)?.into())),
            OpCode::Write12 => Ok(Command::Write(checked_extract::<Write12Command>(cbw)?.into())),
            OpCode::Format => Ok(Command::Format(checked_extract(cbw)?)),
            OpCode::SendDiagnostic => Ok(Command::SendDiagnostic(checked_extract(cbw)?)),
            OpCode::ReportLuns => Ok(Command::ReportLuns(checked_extract(cbw)?)),
            OpCode::StartStopUnit => Ok(Command::StartStopUnit(checked_extract(cbw)?)),
            OpCode::Verify10 => Ok(Command::Verify(checked_extract(cbw)?)),
            OpCode::SynchronizeCache10 => Ok(Command::SynchronizeCache(checked_extract(cbw)?)),
            _ => Err(Error::UnhandledOpCode),
        }
    }
}


fn checked_extract<T>(cbw: &CommandBlockWrapper_NEW) -> Result<T, Error> 
where 
    T: ParsePackedStruct,
    Error: From<<T as Packed>::Error>,
{
    if cbw.data_length < T::BYTES as u8 {
        Err(Error::InsufficientDataForCommand)?;
    }
    Ok(T::parse(&cbw.data)?)
}

impl defmt::Format for Command {
    fn format(&self, f: defmt::Formatter) {
        match self {
            Command::None => defmt::write!(f,"None"),
            Command::Inquiry(_) => defmt::write!(f,"Inquiry"),
            Command::TestUnitReady(_) => defmt::write!(f,"TestUnitReady"),
            Command::ReadCapacity(_) => defmt::write!(f,"ReadCapacity"),
            Command::ModeSense(_) => defmt::write!(f,"ModeSense"),
            Command::PreventAllowMediumRemoval(_) => defmt::write!(f,"PreventAllowMediumRemoval"),
            Command::RequestSense(_) => defmt::write!(f,"RequestSense"),
            Command::Read(_) => defmt::write!(f,"Read"),
            Command::Write(_) => defmt::write!(f,"Write"),
            Command::Format(_) => defmt::write!(f,"Format"),
            Command::SendDiagnostic(_) => defmt::write!(f,"SenseDiagnostic"),
            Command::ReportLuns(_) => defmt::write!(f,"ReportLuns"),
            Command::ModeSelect(_) => defmt::write!(f,"ModeSelect"),
            Command::StartStopUnit(_) => defmt::write!(f,"StartStopUnit"),
            Command::ReadFormatCapacities(_) => defmt::write!(f,"ReadFormatCapacities"),
            Command::Verify(_) => defmt::write!(f,"Verify"),
            Command::SynchronizeCache(_) => defmt::write!(f,"SynchronizeCache"),
        }
    }
}