#![no_std]
#![no_main]

use stm32f4::stm32f469::{rcc, tim6, Peripherals, RCC, TIM6};

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[inline(never)]
fn delay(ms: u64) {
    for _ in 0..(72 * ms) {
        asm::nop();
    }
}

fn delay_tim_init(tim6: &'static tim6::RegisterBlock, rcc: &'static rcc::RegisterBlock) {
    // TIM6
    rcc.apb1enr.write(|w| w.tim6en().set_bit()); // enable TIM6 peripheral
    tim6.cr1.write(|w| {
        w.opm().set_bit(); // Counter stops counting at the next update event (clearing the CEN bit)
        w.cen().clear_bit() // Counter enabled; CEN is cleared automatically in one-pulse mode, when an update event occurs.
    });

    tim6.psc.write(|w| w.psc().bits(8000)); // prescler
}

fn delay_tim(ms: u16, tim6: &'static tim6::RegisterBlock) {
    tim6.arr.write(|w| w.arr().bits(ms));
    tim6.cr1.modify(|_, w| w.cen().set_bit()); // enable the counter

    while !tim6.sr.read().uif().bit_is_set() {
        asm::nop();
    }
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let peripherals = Peripherals::take().unwrap();

    // enable gpiod peripheral
    peripherals.RCC.ahb1enr.write(|w| w.gpioden().set_bit());

    delay_tim_init(unsafe { &*TIM6::ptr() }, unsafe { &*RCC::ptr() });

    let gpiod = &peripherals.GPIOD;

    gpiod.moder.write(|w| {
        w.moder4().output();
        w.moder5().output()
    });

    loop {
        delay_tim(1000, unsafe { &*TIM6::ptr() });

        gpiod.odr.modify(|_, w| w.odr4().set_bit());
        gpiod.odr.modify(|_, w| w.odr5().set_bit());

        delay_tim(1000, unsafe { &*TIM6::ptr() });

        gpiod.odr.modify(|_, w| w.odr4().clear_bit());
        gpiod.odr.modify(|_, w| w.odr5().clear_bit());
    }
}
