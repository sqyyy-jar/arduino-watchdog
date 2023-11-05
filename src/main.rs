//! Echo: 13, 43
//! Trig: 12, 42

#![no_std]
#![no_main]

pub mod hc_sr04;

use arduino_hal::{pins, Peripherals};
use panic_halt as _;

use crate::hc_sr04::get;

const DETECT_DIFF: u16 = 10;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let timer = dp.TC1;
    let mut trig1 = pins.d12.into_output();
    let echo1 = pins.d13;
    let mut trig2 = pins.d42.into_output();
    let echo2 = pins.d43;

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
    // Amount of people that entered the room
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
    }
}
