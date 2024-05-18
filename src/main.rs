#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);
        #[allow(non_snake_case)]
        let letter_A = [
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_B = [
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_C = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_D = [
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_E = [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ];

        #[allow(non_snake_case)]
        let letter_F = [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_G = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 1, 1, 1],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_H = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_J = [
            [0, 0, 0, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_K = [
            [1, 0, 0, 1, 0],
            [1, 0, 1, 0, 0],
            [1, 1, 0, 0, 0],
            [1, 0, 1, 0, 0],
            [1, 0, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_L = [
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_M = [
            [1, 0, 0, 0, 1],
            [1, 1, 0, 1, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_N = [
            [1, 0, 0, 0, 1],
            [1, 1, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 1, 1],
            [1, 0, 0, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_O = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_P = [
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_Q = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_R = [
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 1, 0, 0],
            [1, 0, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_S = [
            [0, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_T = [
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];

        let heart = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_U = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_V = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_W = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 1, 0, 1, 1],
            [1, 0, 0, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_X = [
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_Y = [
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_Z = [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ];

        #[allow(non_snake_case)]
        let letter_a = [
            [0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 1, 1],
        ];

        #[allow(non_snake_case)]
        let letter_b = [
            [1, 0, 0, 0, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_c = [
            [0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_d = [
            [0, 0, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 1, 1],
        ];

        #[allow(non_snake_case)]
        let letter_e = [
            [0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_f = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [1, 1, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_g = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_h = [
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_i = [
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_j = [
            [0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_k = [
            [1, 0, 0, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 1, 0, 0],
            [1, 0, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_l = [
            [1, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 1, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_m = [
            [0, 0, 0, 0, 0],
            [1, 1, 0, 1, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_n = [
            [0, 0, 0, 0, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_o = [
            [0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_p = [
            [0, 0, 0, 0, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_q = [
            [0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 1, 1],
        ];

        #[allow(non_snake_case)]
        let letter_r = [
            [0, 0, 0, 0, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_s = [
            [0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [1, 1, 1, 0, 0],
            [0, 0, 0, 1, 1],
            [1, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_t = [
            [0, 1, 0, 0, 0],
            [1, 1, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_u = [
            [0, 0, 0, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 1, 1],
        ];

        #[allow(non_snake_case)]
        let letter_v = [
            [0, 0, 0, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_w = [
            [0, 0, 0, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 1, 1, 0],
            [0, 1, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_x = [
            [0, 0, 0, 0, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [0, 0, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_y = [
            [0, 0, 0, 0, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_z = [
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 1, 1, 1, 0],
        ];

        loop {
            display.show(&mut timer, letter_I, 1000);
            display.show(&mut timer, heart, 1000);
            display.show(&mut timer, letter_L, 1000);
            display.show(&mut timer, letter_I, 1000);
            display.show(&mut timer, letter_V, 1000);
            display.show(&mut timer, letter_E, 1000);
            display.show(&mut timer, letter_S, 1000);
            display.show(&mut timer, letter_E, 1000);
            display.show(&mut timer, letter_S, 1000);
            display.show(&mut timer, letter_S, 1000);
            display.show(&mut timer, letter_I, 1000);
            display.show(&mut timer, letter_O, 1000);
            display.show(&mut timer, letter_N, 1000);
            display.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
