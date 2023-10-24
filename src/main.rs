mod model;

use model::*;
use nannou::{prelude::*, text::font};

const FRAME_DURATION_MS: u128 = 75;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .run();
}

fn model(_app: &App) -> Model {
    Model::new()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let is_ready = model.status == Status::Playing
        && model.last_updated.elapsed().as_millis() >= FRAME_DURATION_MS;

    if !is_ready {
        return;
    }

    model.move_forward();
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(window_event),
            ..
        } => match window_event {
            KeyPressed(Key::Space) => {
                if model.status == Status::GameOver {
                    model.reset();
                } else {
                    model.toggle_pause();
                }
            }
            KeyPressed(keycode) => {
                Direction::from_keycode(keycode).map(|dir| model.change_direction(dir));
            }
            _ => (),
        },
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for &point in &model.snake {
        draw.rect()
            .xy(point * pt2(SQUARE_SIZE, SQUARE_SIZE))
            .wh(vec2(SQUARE_SIZE, SQUARE_SIZE))
            .color(GREENYELLOW);
    }

    draw.rect()
        .xy(model.food * pt2(SQUARE_SIZE, SQUARE_SIZE))
        .wh(vec2(SQUARE_SIZE, SQUARE_SIZE))
        .color(RED);

    // draw the score in the top right corner
    draw.text(&model.score.to_string())
        .xy(pt2(HALF_WINDOW_SIZE - 10.0, HALF_WINDOW_SIZE - 10.0))
        .color(GREENYELLOW);

    // if the game is paused, draw a pause message
    if model.status != Status::Playing {
        draw.text(model.status.to_string())
            .font(font::default_notosans())
            .xy(pt2(HALF_WINDOW_SIZE - 30.0, HALF_WINDOW_SIZE - 30.0))
            .color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
}
