mod model;

use model::*;
use nannou::prelude::*;

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
    if model.status != Status::Playing || model.last_updated.elapsed().as_millis() < (1000 / 60) {
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
                model.toggle_pause();
            }
            KeyPressed(key) => {
                Direction::from_key(key).map(|dir| model.change_direction(dir));
            }
            _ => (),
        },
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GREENYELLOW);

    for &point in &model.snake {
        draw.rect()
            .xy(point)
            .wh(vec2(SQUARE_SIZE, SQUARE_SIZE))
            .color(BLACK);
    }

    draw.rect()
        .xy(model.food)
        .wh(vec2(SQUARE_SIZE, SQUARE_SIZE))
        .color(RED);

    // draw the score in the top right corner
    draw.text(&model.score.to_string())
        .xy(pt2(HALF_WINDOW_SIZE - 10.0, HALF_WINDOW_SIZE - 10.0))
        .color(BLACK);

    // if the game is paused, draw a pause message
    if model.status != Status::Playing {
        draw.text(model.status.to_string())
            .xy(pt2(HALF_WINDOW_SIZE - 30.0, HALF_WINDOW_SIZE - 30.0))
            .color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
}
