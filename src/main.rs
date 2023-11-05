//! Echo: 13, 43
//! Trig: 12, 42

#![no_std]
#![no_main]

pub mod hc_sr04;

use arduino_hal::{
    delay_ms, pins,
    port::{mode::Output, Pin, PinOps},
    Peripherals,
};
use panic_halt as _;

use crate::hc_sr04::get;

/// Minimum difference from initial distance to trigger
const DETECT_DIFF: u16 = 15;
const TABLE: [u8; 10] = [0xc0, 0xf9, 0xa4, 0xb0, 0x99, 0x92, 0x82, 0xf8, 0x80, 0x90];

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let timer = dp.TC1;
    let mut trig1 = pins.d12.into_output();
    let echo1 = pins.d13;
    let mut trig2 = pins.d42.into_output();
    let echo2 = pins.d43;

    let mut latch = pins.d4.into_output();
    let mut cs = pins.d5.into_output();
    let mut data = pins.d3.into_output();
    let mut d1 = pins.d11.into_output();
    let mut d2 = pins.d10.into_output();
    let mut d3 = pins.d9.into_output();
    let mut d4 = pins.d8.into_output();

    let base1 = loop {
        if let Some(base) = get(&timer, &mut trig1, &echo1) {
            break base;
        }
    };
    let base2 = loop {
        if let Some(base) = get(&timer, &mut trig2, &echo2) {
            break base;
        }
    };
    // Amount of people inside the room
    let mut inside_now = 0u16;
    // Amount of people that entered the room since startup
    let mut inside_total = 0u16;

    let mut prev1 = false;
    let mut prev2 = false;
    loop {
        let Some(cur1) = get(&timer, &mut trig1, &echo1) else {
            continue;
        };
        let Some(cur2) = get(&timer, &mut trig2, &echo2) else {
            continue;
        };
        let diff2 = base2.abs_diff(cur2);
        let active2 = diff2 >= DETECT_DIFF;
        // Person entered the room
        if prev1 && active2 {
            inside_now = inside_now.saturating_add(1);
            inside_total = inside_total.saturating_add(1);
            prev1 = false;
            prev2 = false;
            continue;
        }
        let diff1 = base1.abs_diff(cur1);
        let active1 = diff1 >= DETECT_DIFF;
        // Person left the room
        if prev2 && active1 {
            inside_now = inside_now.saturating_sub(1);
            prev1 = false;
            prev2 = false;
            continue;
        }
        prev1 |= active1;
        prev2 |= active2;
        display(
            &mut latch, &mut cs, &mut data, &mut d1, &mut d2, &mut d3, &mut d4, inside_now,
        )
    }
}

fn shift_out<P1, P2>(data_pin: &mut Pin<Output, P1>, clock_pin: &mut Pin<Output, P2>, data: u8)
where
    P1: PinOps,
    P2: PinOps,
{
    for i in 0..8 {
        // MSBFIRST
        let n = data & (1 << (7 - i));
        // LSBFIRST
        // let n = data & (1 << i);
        if n == 0 {
            data_pin.set_low();
        } else {
            data_pin.set_high();
        }
        clock_pin.set_high();
        clock_pin.set_low();
    }
}

fn update_shift_register<P1, P2, P3>(
    data_pin: &mut Pin<Output, P1>,
    latch_pin: &mut Pin<Output, P2>,
    clock_pin: &mut Pin<Output, P3>,
    data: u8,
) where
    P1: PinOps,
    P2: PinOps,
    P3: PinOps,
{
    latch_pin.set_low();

    shift_out(data_pin, clock_pin, data);

    latch_pin.set_high();
}

#[allow(clippy::too_many_arguments)]
fn display<P1, P2, P3, P4, P5, P6, P7>(
    latch: &mut Pin<Output, P1>,
    cs: &mut Pin<Output, P2>,
    data: &mut Pin<Output, P3>,
    d1: &mut Pin<Output, P4>,
    d2: &mut Pin<Output, P5>,
    d3: &mut Pin<Output, P6>,
    d4: &mut Pin<Output, P7>,
    value: u16,
) where
    P1: PinOps,
    P2: PinOps,
    P3: PinOps,
    P4: PinOps,
    P5: PinOps,
    P6: PinOps,
    P7: PinOps,
{
    let digit1 = value % 10;
    let digit2 = value / 10 % 10;
    let digit3 = value / 100 % 10;
    let digit4 = value / 1000 % 10;
    {
        update_shift_register(data, latch, cs, TABLE[digit1 as usize]);
        d2.set_low();
        d3.set_low();
        d4.set_low();
        d1.set_high();
    }
    delay_ms(5);
    {
        update_shift_register(data, latch, cs, TABLE[digit2 as usize]);
        d1.set_low();
        d2.set_high();
    }
    delay_ms(5);
    {
        update_shift_register(data, latch, cs, TABLE[digit3 as usize]);
        d2.set_low();
        d3.set_high();
    }
    delay_ms(5);
    {
        update_shift_register(data, latch, cs, TABLE[digit4 as usize]);
        d3.set_low();
        d4.set_high();
    }
}
