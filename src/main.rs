// use macroquad::audio::{load_sound, play_sound_once};
// use macroquad::prelude::*;

use macroquad::{audio, prelude::*};

// fn conf() -> Conf {
//     Conf {
//         window_title: String::from("Clicker Game"), //this field is not optional!
//         fullscreen: false,
//         //you can add other options too, or just use the default ones:
//         ..Default::default()
//     }
// }
// //then pass the function to the attribute
// #[macroquad::main(conf)]

#[macroquad::main("Clicker Game")]
async fn main() {
    let (x, y) = (screen_width() / 2., screen_height() / 2.);
    let r = 70.;
    let circle = Circle::new(x, y, r);
    let mut score = 0;

    let click_sound = audio::load_sound("res/click.wav").await.unwrap();

    loop {
        clear_background(GRAY);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mouse_circ = Circle::new(mouse_x, mouse_y, 1.);

            if mouse_circ.overlaps(&circle) {
                audio::play_sound_once(&click_sound);
                score += 1;
            }
        }

        draw_text("Clicker Game", screen_width() / 2. - 100., 100., 50., WHITE);
        draw_text(
            format!("Clicks: {}", score).as_str(),
            screen_width() / 2. - 100.,
            500.,
            50.,
            WHITE,
        );

        draw_circle(x, y, r, RED);
        next_frame().await;
    }
}
