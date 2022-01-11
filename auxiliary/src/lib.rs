#![no_std]

use cortex_m::asm;
use stm32f4::stm32f469::{gpiok, rcc, tim6, Peripherals, GPIOD, RCC, TIM6};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn delay_tim_init(tim6: &'static tim6::RegisterBlock, rcc: &'static rcc::RegisterBlock) {
    // TIM6
    rcc.apb1enr.write(|w| w.tim6en().set_bit()); // enable TIM6 peripheral
    tim6.cr1.write(|w| {
        w.opm().set_bit(); // Counter stops counting at the next update event (clearing the CEN bit)
        w.cen().clear_bit() // Counter enabled; CEN is cleared automatically in one-pulse mode, when an update event occurs.
    });

    tim6.psc.write(|w| w.psc().bits(8000)); // prescler
}

pub fn delay_tim(ms: u16, tim6: &'static tim6::RegisterBlock) {
    tim6.arr.write(|w| w.arr().bits(ms));
    tim6.cr1.modify(|_, w| w.cen().set_bit()); // enable the counter

    while !tim6.sr.read().uif().bit_is_set() {
        asm::nop();
    }
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

// enable LD2 and LD3
pub fn leds_init(rcc: &'static rcc::RegisterBlock, gpiod: &'static gpiok::RegisterBlock) {
    rcc.ahb1enr.write(|w| w.gpioden().set_bit());
    gpiod.moder.write(|w| {
        w.moder4().output();
        w.moder5().output()
    });
}

pub fn leds_on(gpiod: &'static gpiok::RegisterBlock) {
    gpiod.odr.modify(|_, w| w.odr4().set_bit());
    gpiod.odr.modify(|_, w| w.odr5().set_bit());
}

pub fn leds_off(gpiod: &'static gpiok::RegisterBlock) {
    gpiod.odr.modify(|_, w| w.odr4().clear_bit());
    gpiod.odr.modify(|_, w| w.odr5().clear_bit());
}
