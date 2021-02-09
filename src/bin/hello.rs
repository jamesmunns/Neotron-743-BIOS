#![no_main]
#![no_std]

use neotron_743_bios as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    neotron_743_bios::exit()
}
