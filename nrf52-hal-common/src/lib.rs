#![no_std]

use embedded_hal as hal;

#[cfg(feature = "52810")]
pub use nrf52810_pac as target;

#[cfg(feature = "52832")]
pub use nrf52832_pac as target;

#[cfg(feature = "52840")]
pub use nrf52840_pac as target;

pub mod delay;
pub mod spim;
pub mod gpio;
pub mod clocks;
pub mod rng;
pub mod time;
pub mod timer;
pub mod twim;
pub mod uarte;

pub mod prelude {
    pub use crate::hal::prelude::*;

    pub use crate::clocks::ClocksExt;
    pub use crate::gpio::GpioExt;
    pub use crate::rng::RngExt;
    pub use crate::spim::SpimExt;
    pub use crate::time::U32Ext;
    pub use crate::timer::TimerExt;
    pub use crate::twim::TwimExt;
    pub use crate::uarte::UarteExt;
}

/// Length of Nordic EasyDMA differs for MCUs
#[cfg(any(feature = "52810", feature = "52832"))]
pub mod target_constants {
    // NRF52832 8 bits1..0xFF
    pub const EASY_DMA_SIZE: usize = 255;
    // Easy DMA can only read from data ram
    pub const SRAM_LOWER:usize = 0x2000_0000;
    pub const SRAM_UPPER:usize = 0x3000_0000;
    pub const FORCE_COPY_BUFFER_SIZE:usize = 255;
}
#[cfg(feature = "52840")]
pub mod target_constants {
    // NRF52840 16 bits 1..0xFFFF
    pub const EASY_DMA_SIZE: usize = 65535;
    // Limits for Easy DMA - it can only read from data ram
    pub const SRAM_LOWER:usize = 0x2000_0000;
    pub const SRAM_UPPER:usize = 0x3000_0000;
    pub const FORCE_COPY_BUFFER_SIZE:usize = 1024;
}

pub use crate::clocks::Clocks;
pub use crate::delay::Delay;
pub use crate::spim::Spim;
pub use crate::timer::Timer;
pub use crate::twim::Twim;
pub use crate::uarte::Uarte;
