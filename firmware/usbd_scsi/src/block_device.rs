#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockDeviceError {
    /// Hardware didn't behave as expected, unrecoverable
    HardwareError,

    /// Error during writing; most likely value read back after write was wrong
    WriteError,

    /// Error during erase; most likely value read back after erase was wrong
    /// 
    /// STM32 flash programming app note implies this is possible but doesn't say under what 
    /// circumstances. Is the flash knackered if this happens?
    EraseError,

    /// Address is invalid or out of range
    InvalidAddress,
}

pub trait BlockDevice {
    /// The number of bytes per block. This determines the size of the buffer passed
    /// to read/write functions
    const BLOCK_BYTES: usize;

    /// Read the block indicated by `lba` into the provided buffer
    fn read_block(&mut self, lba: u32, block: &mut [u8]) -> Result<(), BlockDeviceError>;

    /// Write the `block` buffer to the block indicated by `lba`
    fn write_block(&mut self, lba: u32, block: &[u8]) -> Result<(), BlockDeviceError>;

    /// Erase entire device
    fn erase_device(&mut self) -> Result<(), BlockDeviceError>;

    /// Read the u32 value indicated by buffer id
    fn read_buffer(&mut self, id: u8) -> Result<u32, BlockDeviceError>;

    /// Write to the the u32 value indicated by buffer id
    fn write_buffer(&mut self, id: u8, data: u32) -> Result<(), BlockDeviceError>;
    
    /// Get the maxium valid lba (logical block address)
    fn max_lba(&self) -> u32;
}