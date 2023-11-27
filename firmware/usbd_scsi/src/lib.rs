#![no_std]

mod scsi;
pub use scsi::*;

mod block_device;
pub use block_device::*;

#[macro_export]
    macro_rules! stub {
        (target: $target:expr, $( $arg:expr$(,)?)+ ) => (
            {
                let _ = $target;
                $(let _ = $arg;)+
                ()
            }
        );
        ( $( $arg:expr$(,)?)+ ) => (
            {
                $(let _ = $arg;)+
                ()
            }
        )
    }
mod logging {

    pub use defmt_rtt as _;

    #[cfg(feature = "trace-scsi-command")]
    pub use defmt::trace as trace_scsi_command;
    #[cfg(not(feature = "trace-scsi-command"))]
    pub use stub as trace_scsi_command;
    
    #[cfg(feature = "trace-scsi-fs")]
    pub use defmt::trace as trace_scsi_fs;
    #[cfg(not(feature = "trace-scsi-fs"))]
    pub use stub as trace_scsi_fs;
}
