#![no_main]
#![no_std]

use cortex_m::Peripherals as CorePeripherals;
use dwt_systick_monotonic::DwtSystick;
use fugit::Duration;
use stm32f3xx_hal::pac::{self, Peripherals as DevicePeripherals};
use stm32f3xx_hal::prelude::*;

use {{crate_name}} as _; // global logger + panicking behavior + memory layout

#[rtic::app(device = pac, peripherals = true, dispatchers = [EXTI0, EXTI1])]
mod app {
    use super::*;

    const CLOCK_FREQ: u32 = 8_000_000; // 8 MHz

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[monotonic(binds = SysTick, default = true)]
    type Mono = DwtSystick<CLOCK_FREQ>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        task1::spawn().ok();
        (
            Shared {},
            Local {},
            init::Monotonics(init_mono(ctx.core, ctx.device)),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task]
    fn task1(_: task1::Context) {
        defmt::info!("task1");
        task1::spawn_after(Duration::<u32, 1, CLOCK_FREQ>::millis(125)).unwrap();
    }

    fn init_mono(mut cp: CorePeripherals, dp: DevicePeripherals) -> Mono {
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        let mut flash = dp.FLASH.constrain();
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(CLOCK_FREQ.Hz().into()).freeze(&mut flash.acr);

        DwtSystick::new(
            &mut cp.DCB,
            cp.DWT,
            cp.SYST,
            clocks.hclk().0,
        )
    }
}
