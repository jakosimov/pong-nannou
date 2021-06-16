use nannou::prelude::*;
use std::ops;
use rand::Rng;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
    // nannou::sketch(view).run();
}

#[derive(Copy,Clone)]
struct Vect(f32, f32);

impl ops::Add for Vect {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let Vect(x1, y1) = self;
        let Vect(x2, y2) = rhs;
        Vect(x1 + x2, y1 + y2)
    }
}

impl ops::Sub for Vect {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let Vect(x1, y1) = self;
        let Vect(x2, y2) = rhs;
        Vect(x1 - x2, y1 - y2)
    }
}

struct Model {
    p1_score: i32,
    p2_score: i32,
    ball_pos: Vect,
    p1_pos: Vect,
    p2_pos: Vect,
    ball_v: Vect,
    p1_v: Vect,
    p2_v: Vect,
    up_pressed: bool,
    down_pressed: bool,
    w_pressed: bool,
    s_pressed: bool,
    rand_gen: rand::rngs::ThreadRng
}

const PLAYER_SPEED: f32 = 5.0;
const BALL_HORIZONTAL_SPEED: f32 = 3.0;
const TOP_MARGIN: f32 = 100.0;
const SIDE_MARGIN: f32 = 100.0;
const WIDTH: f32 = 10.0;
const HEIGHT: f32 = 40.0;
const SPEED_TRANSFER_RATE: f32 = 0.8;

fn model(_app: &App) -> Model {
    let mut rand_gen = rand::thread_rng();
    let (ball_pos, ball_v) = reset_ball(&mut rand_gen);

    Model {
        p1_score: 0,
        p2_score: 0,
        p1_pos: Vect(-400.0, 0.0),
        p2_pos: Vect(400.0, 0.0),
        ball_pos,
        ball_v,
        p1_v: Vect(0.0, 0.0),
        p2_v: Vect(0.0, 0.0),
        up_pressed: false,
        down_pressed: false,
        w_pressed: false,
        s_pressed: false,
        rand_gen,
    }
}

fn ball_speed_limit(y_speed: f32) -> f32 {
    y_speed
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let ball_pos: &mut Vect = &mut model.ball_pos;
    let p1_pos: &mut Vect = &mut model.p1_pos;
    let p2_pos: &mut Vect = &mut model.p2_pos;
    let window = app.window_rect();
    *ball_pos = *ball_pos + model.ball_v;
    *p1_pos = *p1_pos + model.p1_v;
    *p2_pos = *p2_pos + model.p2_v;

    if is_overlapping(*p1_pos, *ball_pos) {
        model.ball_v.0 = -model.ball_v.0;
        model.ball_v.1 = model.ball_v.1 + model.p1_v.1 * SPEED_TRANSFER_RATE;
        *ball_pos = *ball_pos + model.ball_v;
    } else if is_overlapping(*p2_pos, *ball_pos) {
        model.ball_v.0 = -model.ball_v.0;
        model.ball_v.1 = model.ball_v.1 + model.p2_v.1 * SPEED_TRANSFER_RATE;
        *ball_pos = *ball_pos + model.ball_v;
    }

    model.ball_v.1 = ball_speed_limit(model.ball_v.1);

    if touching_ceiling(adjust_ball(*ball_pos), window) {
        model.ball_v.1 = -model.ball_v.1;
    }

    if passed_left_side(adjust_ball(*ball_pos), window) {
        model.p2_score += 1;
        let (new_ball_pos, new_ball_v) = reset_ball(&mut model.rand_gen);
        *ball_pos = new_ball_pos;
        model.ball_v = new_ball_v;
    }

    if passed_right_side(adjust_ball(*ball_pos), window) {
        model.p1_score += 1;
        let (new_ball_pos, new_ball_v) = reset_ball(&mut model.rand_gen);
        *ball_pos = new_ball_pos;
        model.ball_v = new_ball_v;
    }

    let bottom_y = -window.h() / 2.0 - TOP_MARGIN;
    let top_y = window.h() / 2.0 + TOP_MARGIN + HEIGHT;
    if p1_pos.1 > top_y {
        p1_pos.1 = bottom_y;
    }
    if p2_pos.1 > top_y {
        p2_pos.1 = bottom_y;
    }
    if p1_pos.1 < bottom_y {
        p1_pos.1 = top_y;
    }
    if p2_pos.1 < bottom_y {
        p2_pos.1 = top_y;
    }
}

fn adjust_ball(ball: Vect) -> Vect {
    Vect(ball.0 - WIDTH / 2.0, ball.1 + WIDTH / 2.0)
}

fn adjust_player(player: Vect) -> Vect {
    Vect(player.0 - WIDTH / 2.0, player.1 + HEIGHT / 2.0)
}

fn reset_ball(rnd: &mut rand::rngs::ThreadRng) -> (Vect, Vect) {
    let x_direction = if rnd.gen::<i16>() % 2 == 0 { 1.0 } else { -1.0 };
    let y_direction = if rnd.gen::<i16>() % 2 == 0 { 1.0 } else { -1.0 };
    let y_scalar = rnd.gen::<f32>();

    (Vect(0.0, 0.0), Vect(BALL_HORIZONTAL_SPEED * x_direction, BALL_HORIZONTAL_SPEED * y_scalar * y_direction))
}

fn is_overlapping(player: Vect, ball: Vect) -> bool {
    let player = adjust_player(player);
    let ball = adjust_ball(ball);

    let x_overlapping = (ball.0 >= player.0 && ball.0 < player.0 + WIDTH)
        || (player.0 >= ball.0 && player.0 < ball.0 + WIDTH);

    let y_overlapping = (ball.1 <= player.1 && ball.1 > player.1 - HEIGHT)
        || (player.1 <= ball.1 && player.1 > ball.1 - WIDTH);

    x_overlapping && y_overlapping
}

fn touching_ceiling(ball: Vect, window: Rect) -> bool {
    ball.1-WIDTH < -window.h() / 2.0 || ball.1 > window.h() / 2.0
}

fn passed_left_side(ball: Vect, window: Rect) -> bool {
    ball.0 < -window.w() / 2.0 - SIDE_MARGIN
}

fn passed_right_side(ball: Vect, window: Rect) -> bool {
    ball.0 + WIDTH > window.w() / 2.0 + SIDE_MARGIN
}

fn draw_player(draw: &Draw, vect: Vect) {
    draw.rect()
        .color(WHITE)
        .width(WIDTH)
        .height(HEIGHT)
        .x_y(vect.0, vect.1);
    let vect = adjust_player(vect);
    draw.rect()
        .color(RED)
        .width(WIDTH / 5.0)
        .height(WIDTH / 5.0)
        .x_y(vect.0, vect.1);
}

fn draw_ball(draw: &Draw, vect: Vect) {
    draw.rect()
        .color(WHITE)
        .width(WIDTH)
        .height(WIDTH)
        .x_y(vect.0, vect.1);
    let vect = adjust_ball(vect);
    draw.rect()
        .color(RED)
        .width(WIDTH / 5.0)
        .height(WIDTH / 5.0)
        .x_y(vect.0, vect.1);
}

const TEXT_Y: f32 = 300.0;
const TEXT_X: f32 = 100.0;

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // let window = app.window_rect();

    draw.background().color(BLACK);

    draw_player(&draw, model.p1_pos);
    draw_player(&draw, model.p2_pos);
    draw_ball(&draw, model.ball_pos);

    draw.text(&model.p1_score.to_string())
        .font_size(35)
        .x_y(-TEXT_X, TEXT_Y);
    draw.text(&model.p2_score.to_string())
        .font_size(35)
        .x_y(TEXT_X, TEXT_Y);

    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(window_event),
        ..
    } = event
    {
        let key_pressed: Option<(Key, bool)> = match window_event {
            WindowEvent::KeyPressed(k) => Some((k, true)),
            WindowEvent::KeyReleased(k) => Some((k, false)),
            _ => None,
        };

        let action: Option<(&mut Vect, &mut bool, bool, bool)> = match key_pressed {
            Some((Key::Up, pressed)) => {
                Some((&mut model.p2_v, &mut model.up_pressed, true, pressed))
            }
            Some((Key::Down, pressed)) => {
                Some((&mut model.p2_v, &mut model.down_pressed, false, pressed))
            }
            Some((Key::W, pressed)) => Some((&mut model.p1_v, &mut model.w_pressed, true, pressed)),
            Some((Key::S, pressed)) => {
                Some((&mut model.p1_v, &mut model.s_pressed, false, pressed))
            }
            _ => None,
        };

        if let Some((p_v, is_pressed, positive_direction, pressed)) = action {
            if pressed {
                if !(*is_pressed) {
                    *is_pressed = true;
                    p_v.1 = p_v.1 + if positive_direction { PLAYER_SPEED } else { -PLAYER_SPEED }
                }
            } else {
                *is_pressed = false;
                p_v.1 = p_v.1 + if positive_direction { -PLAYER_SPEED } else { PLAYER_SPEED }
            }
        }
    }
}
