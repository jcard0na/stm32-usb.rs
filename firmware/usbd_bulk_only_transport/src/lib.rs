#![no_std]

mod bulk_only_transport;

pub use bulk_only_transport::{
    BulkOnlyTransport,
    CommandBlockWrapper,
    TransferState,
    Error,
};

mod logging {
    use defmt_rtt as _;

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


    #[cfg(feature = "trace-bot-headers")]
    pub use defmt::trace as trace_bot_headers;

    #[cfg(feature = "trace-bot-states")]
    pub use defmt::trace as trace_bot_states;

    #[cfg(feature = "trace-bot-bytes")]
    pub use defmt::trace as trace_bot_bytes;

    #[cfg(feature = "trace-bot-zlp")]
    pub use defmt::trace as trace_bot_zlp;

    #[cfg(feature = "trace-bot-buffer")]
    pub use defmt::trace as trace_bot_buffer;
    
    #[cfg(feature = "trace-usb-control")]
    pub use defmt::trace as trace_usb_control;

    #[cfg(not(feature = "trace-bot-headers"))]
    pub use stub as trace_bot_headers;

    #[cfg(not(feature = "trace-bot-states"))]
    pub use stub as trace_bot_states;

    #[cfg(not(feature = "trace-bot-bytes"))]
    pub use stub as trace_bot_bytes;

    #[cfg(not(feature = "trace-bot-zlp"))]
    pub use stub as trace_bot_zlp;

    #[cfg(not(feature = "trace-bot-buffer"))]
    pub use stub as trace_bot_buffer;
    
    #[cfg(not(feature = "trace-usb-control"))]
    pub use stub as trace_usb_control;
}
