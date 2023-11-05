use arduino_hal::{
    delay_us,
    pac::TC1,
    port::{
        mode::{Floating, Input, Output},
        Pin, PinOps,
    },
};

/// Wartet auf ein Signal des HC-SR04 Sensors und gibt dann die Entfernung in cm zurück.
pub fn get<P1: PinOps, P2: PinOps>(
    timer: &TC1,
    trig: &mut Pin<Output, P1>,
    echo: &Pin<Input<Floating>, P2>,
) -> Option<u16> {
    // Setzt den Timer zurück
    timer.tcnt1.write(|w| w.bits(0));

    // Setze den Trigger für 10µs auf high
    trig.set_high();
    delay_us(10);
    trig.set_low();

    // Warte für 200ms darauf, dass der Sensor antwortet
    while echo.is_low() {
        // Starte die Messung erneut, falls kein Hindernis erkannt wurde
        if timer.tcnt1.read().bits() >= 50_000 {
            return None;
        }
    }

    // Setze den Timer zurück
    timer.tcnt1.write(|w| w.bits(0));

    // Warte darauf, dass echo wieder low wird
    while echo.is_high() {}

    // 1 count == 4 µs, so the value is multiplied by 4.
    // 1/58 ≈ (34000 cm/s) * 1µs / 2
    // when no object is detected, instead of keeping the echo pin completely low,
    // some HC-SR04 labeled sensor holds the echo pin in high state for very long time,
    // thus overflowing the u16 value when multiplying the timer1 value with 4.
    // overflow during runtime causes panic! so it must be handled
    let temp_timer = timer.tcnt1.read().bits().saturating_mul(4); // Time in µs
    let value = match temp_timer {
        u16::MAX => return None,
        _ => temp_timer / 58,
    };
    while timer.tcnt1.read().bits() < 25_000 {}
    Some(value)
}

// /// TODO:
// /// - Add variables that store the expected distance
// /// - Add constant that expresses how big the difference (non-absolute) should be at minimum to
// ///   trigger
// /// - Create actual return logic
// pub fn get_two(
//     timer: &TC1,
//     trig1: &mut Pin<Output>,
//     echo1: &Pin<Input<Floating>>,
//     trig2: &mut Pin<Output>,
//     echo2: &Pin<Input<Floating>>,
// ) -> Option<(u16, u16)> {
//     // Setzt den Timer zurück
//     timer.tcnt1.write(|w| w.bits(0));
//
//     // Setze den Trigger für 10µs auf high
//     trig1.set_high();
//     trig2.set_high();
//     delay_us(10);
//     trig1.set_low();
//     trig2.set_low();
//
//     // Warte für 200ms darauf, dass der Sensor antwortet
//     //
//     // Goal:
//     // - Check if the person walked in or out
//     let mut res1 = false;
//     let mut res2 = false;
//     while res1.is_none() || res2.is_none() {
//         // Erster Sensor wurde zuerst geschaltet
//         if res1.is_none() && echo1.is_high() {
//             todo!()
//         }
//         // Zweiter Sensor wurde zuerst geschaltet
//         if res2.is_none() && echo2.is_high() {
//             todo!()
//         }
//         if timer.tcnt1.read().bits() >= 50_000 {
//             return None;
//         }
//     }
//
//     // Setze den Timer zurück
//     timer.tcnt1.write(|w| w.bits(0));
//
//     // Warte darauf, dass echo wieder low wird
//     while echo1.is_high() || echo2.is_high() {}
//
//     // 1 count == 4 µs, so the value is multiplied by 4.
//     // 1/58 ≈ (34000 cm/s) * 1µs / 2
//     // when no object is detected, instead of keeping the echo pin completely low,
//     // some HC-SR04 labeled sensor holds the echo pin in high state for very long time,
//     // thus overflowing the u16 value when multiplying the timer1 value with 4.
//     // overflow during runtime causes panic! so it must be handled
//     let temp_timer = timer.tcnt1.read().bits().saturating_mul(4);
//     let value = match temp_timer {
//         u16::MAX => return None,
//         _ => temp_timer / 58,
//     };
//     while timer.tcnt1.read().bits() < 25_000 {}
//     Some((res1?, res2?))
// }
