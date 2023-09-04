extern crate pancurses;

use pancurses::{endwin, initscr, noecho, Input, Window};
use std::thread::sleep;
use std::time::Duration;

const LOGOHEIGHT: usize = 6;
const LOGOPATTERNS: usize = 6;
const LOGOLENGTH: usize = 84;

fn main() {
    let window = initscr();
    window.timeout(0);
    window.nodelay(true);

    pancurses::start_color();
    pancurses::init_pair(1, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    window.attron(pancurses::COLOR_PAIR(1));

    pancurses::curs_set(0);
    noecho();

    let patterns = [
        [
            "     ++      +------ ",
            "     ||      |+-+ |  ",
            "   /---------|| | |  ",
            "  + ========  +-+ |  ",
            " _|--O========O~\\-+  ",
            "//// \\_/      \\_/    ",
            "                     ",
        ],
        [
            "     ++      +------ ",
            "     ||      |+-+ |  ",
            "   /---------|| | |  ",
            "  + ========  +-+ |  ",
            " _|--/O========O\\-+  ",
            "//// \\_/      \\_/    ",
            "                     ",
        ],
        [
            "     ++      +------ ",
            "     ||      |+-+ |  ",
            "   /---------|| | |  ",
            "  + ========  +-+ |  ",
            " _|--/~O========O-+  ",
            "//// \\_/      \\_/    ",
            "                     ",
        ],
        [
            "     ++      +------ ",
            "     ||      |+-+ |  ",
            "   /---------|| | |  ",
            "  + ========  +-+ |  ",
            " _|--/~\\------/~\\-+  ",
            "//// \\_O========O    ",
            "                     ",
        ],
        [
            "     ++      +------ ",
            "     ||      |+-+ |  ",
            "   /---------|| | |  ",
            "  + ========  +-+ |  ",
            " _|--/~\\------/~\\-+  ",
            "//// \\O========O/    ",
            "                     ",
        ],
        [
            "     ++      +------ ",
            "     ||      |+-+ |  ",
            "   /---------|| | |  ",
            "  + ========  +-+ |  ",
            " _|--/~\\------/~\\-+  ",
            "//// O========O_/    ",
            "                     ",
        ],
    ];

    let mut x = window.get_max_x() - 1;

    loop {
        if add_sl(&window, x, &patterns) == pancurses::ERR {
            break;
        }

        if let Some(Input::KeyResize) = window.getch() {
            window.clear();
            x = window.get_max_x() - 1;
        }

        window.refresh();
        sleep(Duration::from_millis(40));
        x -= 1;
    }

    endwin();
}

fn add_sl(window: &Window, x: i32, patterns: &[[&str; LOGOHEIGHT + 1]]) -> i32 {
    if x < -(LOGOLENGTH as i32) {
        return pancurses::ERR;
    }

    let y = window.get_max_y() / 2 - 3;

    for i in 0..=LOGOHEIGHT {
        my_mvaddstr(
            window,
            y + i as i32,
            x,
            patterns[(LOGOLENGTH as i32 + x) as usize / 3 % LOGOPATTERNS][i],
        );
    }

    pancurses::OK
}

fn my_mvaddstr(window: &Window, y: i32, x: i32, s: &str) {
    let mut x = x;
    for c in s.chars() {
        if x >= 0 {
            window.mvaddch(y, x, c);
        }
        x += 1;
    }
}
