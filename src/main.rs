#![no_std]
#![no_main]

use auxiliary::leds_init;
use auxiliary::leds_off;
use auxiliary::leds_on;
use stm32f4::stm32f469::{gpiok, Peripherals, GPIOD, RCC, TIM6};

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use auxiliary::{delay_tim, delay_tim_init};
use cortex_m::asm;
use cortex_m_rt::entry;

#[inline(never)]
fn _delay(ms: u64) {
    for _ in 0..(72 * ms) {
        asm::nop();
    }
}

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    //let peripherals = Peripherals::take().unwrap();

    // enable gpiod peripheral
    //peripherals.RCC.ahb1enr.write(|w| w.gpioden().set_bit());

    delay_tim_init(unsafe { &*TIM6::ptr() }, unsafe { &*RCC::ptr() });

    leds_init(unsafe { &*RCC::ptr() }, unsafe { &*GPIOD::ptr() });

    loop {
        delay_tim(1000, unsafe { &*TIM6::ptr() });

        leds_on(unsafe { &*GPIOD::ptr() });

        delay_tim(1000, unsafe { &*TIM6::ptr() });

        leds_off(unsafe { &*GPIOD::ptr() });
    }
}
