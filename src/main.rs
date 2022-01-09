#![no_std]
#![no_main]

use stm32f4::stm32f469;

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

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let mut peripherals = stm32f469::Peripherals::take().unwrap();

    // enable gpiod peripheral
    peripherals.RCC.ahb1enr.write(|w| w.gpioden().set_bit());

    // TIM6
    peripherals.RCC.apb1enr.write(|w| w.tim6en().set_bit()); // enable TIM6 peripheral
    peripherals.TIM6.cr1.write(|w| {
        w.opm().set_bit(); // Counter stops counting at the next update event (clearing the CEN bit)
        w.cen().clear_bit() // Counter enabled; CEN is cleared automatically in one-pulse mode, when an update event occurs.
    });

    peripherals.TIM6.psc.write(|w| w.psc().bits(8000)); // prescler
    peripherals.TIM6.arr.write(|w| w.arr().bits(1000)); // 1000 ms
    peripherals.TIM6.cr1.modify(|_, w| w.cen().set_bit()); // enable the counter

    let gpiod = &peripherals.GPIOD;

    gpiod.moder.write(|w| {
        w.moder4().output();
        w.moder5().output()
    });

    let mut led_toggle = false;
    loop {
        //gpiod.odr.modify(|_, w| w.odr6().set_bit());

        //delay(200);
        let reader = peripherals.TIM6.sr.read();
        let flag = reader.uif().bit_is_set();
        if flag {
            peripherals.TIM6.sr.modify(|_, w| w.uif().clear_bit());

            if led_toggle {
                gpiod.odr.modify(|_, w| w.odr4().set_bit());
                gpiod.odr.modify(|_, w| w.odr5().set_bit());
                led_toggle = false;
            } else {
                gpiod.odr.modify(|_, w| w.odr4().clear_bit());
                gpiod.odr.modify(|_, w| w.odr5().clear_bit());
                led_toggle = true;
            }

            peripherals.TIM6.cr1.modify(|_, w| w.cen().set_bit()); // enable the counter
        };

        //gpiod.odr.modify(|_, w| w.odr6().clear_bit());

        //delay(200);
    }
}
