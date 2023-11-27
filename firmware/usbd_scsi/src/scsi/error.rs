use packing::Error as PackingError;
use usbd_bulk_only_transport::Error as BulkOnlyTransportError;
use usb_device::UsbError;
use crate::block_device::BlockDeviceError;

#[derive(Debug)]
pub enum Error {
    UnhandledOpCode,
    /// The identified opcode requires more data than was sent
    InsufficientDataForCommand,
    PackingError(PackingError),
    BlockDeviceError(BlockDeviceError),
    BulkOnlyTransportError(BulkOnlyTransportError),
}

impl From<PackingError> for Error {
    fn from(e: PackingError) -> Error {
        Error::PackingError(e)
    }
}

impl From<BlockDeviceError> for Error {
    fn from(e: BlockDeviceError) -> Error {
        Error::BlockDeviceError(e)
    }
}

impl From<BulkOnlyTransportError> for Error {
    fn from(e: BulkOnlyTransportError) -> Error {
        Error::BulkOnlyTransportError(e)
    }
}

impl From<UsbError> for Error {
    fn from(e: UsbError) -> Error {
        Error::BulkOnlyTransportError(e.into())
    }
}

impl defmt::Format for Error {
    fn format(&self, f: defmt::Formatter) {
        match self {
            Error::UnhandledOpCode => defmt::write!(f, "UnhandledOpCode"),
            Error::InsufficientDataForCommand => defmt::write!(f, "InsufficientDataForCommand"),
            Error::PackingError(_) => defmt::write!(f, "PackingError"),
            Error::BlockDeviceError(_) => defmt::write!(f, "BlockDeviceError"),
            Error::BulkOnlyTransportError(_) => defmt::write!(f, "BulkOnlyTransportError"),
        }
    }
}