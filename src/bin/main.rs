#![no_main]
#![no_std]

use {{crate_name}} as _; // global logger + panicking behavior + memory layout

#[rtic::app(device = stm32f3xx_hal::pac, peripherals = true, dispatchers = [EXTI0, EXTI1])]
mod app {
    use fugit::Duration;
    use dwt_systick_monotonic::DwtSystick;
    use stm32f3xx_hal::prelude::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    const CLOCK_FREQ: u32 = 48_000_000; // 48 MHz

    #[monotonic(binds = SysTick, default = true)]
    type DwtMono = DwtSystick<CLOCK_FREQ>;

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        ctx.core.DCB.enable_trace();
        ctx.core.DWT.enable_cycle_counter();

        let mut flash = ctx.device.FLASH.constrain();
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(CLOCK_FREQ.Hz().into()).freeze(&mut flash.acr);

        let mono = DwtSystick::new(
            &mut ctx.core.DCB,
            ctx.core.DWT,
            ctx.core.SYST,
            clocks.hclk().0,
        );

        task1::spawn().ok();

        (
            Shared {},
            Local {},
            init::Monotonics(mono),
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
}
