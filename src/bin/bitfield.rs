#![no_main]
#![no_std]

use neotron_743_bios as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    // value of the FREQUENCY register (nRF52840 device; RADIO peripheral)
    let frequency: u32 = 276;
    defmt::debug!("FREQUENCY: {0:0..7}, MAP: {0:8..9}", frequency);

    neotron_743_bios::exit()
}
