#![allow(dead_code)]

use animation_engine::executor::*;
use animation_engine::*;
use futures::{select, FutureExt};

async fn wait_greater_axis_value(cx: &AnimationEngineContext, axis: Axis, value: f32) {
    loop {
        if cx.axis(axis) > value {
            break;
        }
        next_frame().await;
    }
}

async fn wait_less_axis_value(cx: &AnimationEngineContext, axis: Axis, value: f32) {
    loop {
        if cx.axis(axis) < value {
            break;
        }
        next_frame().await;
    }
}

pub async fn wait_select_button(cx: &AnimationEngineContext) {
    select! {
        _ = cx.wait_key_down(KeyCode::Z).fuse() => (),
        _ = cx.wait_button_down(Button::South).fuse() => (),
        _ = cx.wait_button_down(Button::West).fuse() => (),
    }
}

pub async fn wait_cancel_button(cx: &AnimationEngineContext) {
    select! {
        _ = cx.wait_key_down(KeyCode::X).fuse() => (),
        _ = cx.wait_button_down(Button::North).fuse() => (),
        _ = cx.wait_button_down(Button::East).fuse() => (),
    }
}

pub async fn wait_sub_button(cx: &AnimationEngineContext) {
    select! {
        _ = cx.wait_key_down(KeyCode::C).fuse() => (),
        _ = cx.wait_button_down(Button::Start).fuse() => (),
        _ = cx.wait_button_down(Button::Select).fuse() => (),
    }
}

pub async fn wait_left_trigger(cx: &AnimationEngineContext) {
    select! {
        _ = cx.wait_button_down(Button::LeftTrigger).fuse() => (),
        _ = cx.wait_button_down(Button::LeftTrigger2).fuse() => (),
    }
}

pub async fn wait_right_trigger(cx: &AnimationEngineContext) {
    select! {
        _ = cx.wait_button_down(Button::RightTrigger).fuse() => (),
        _ = cx.wait_button_down(Button::RightTrigger2).fuse() => (),
    }
}

pub async fn wait_left(cx: &AnimationEngineContext) {
    select! {
        _ = cx.wait_key_down(KeyCode::Left).fuse() => (),
        _ = cx.wait_button_down(Button::DPadLeft).fuse() => (),
        _ = wait_less_axis_value(cx, Axis::LeftStickX, -0.7).fuse() => (),
    }
}

pub async fn wait_right(cx: &AnimationEngineContext) {
    select! {
        _ = cx.wait_key_down(KeyCode::Right).fuse() => (),
        _ = cx.wait_button_down(Button::DPadRight).fuse() => (),
        _ = wait_greater_axis_value(cx, Axis::LeftStickX, 0.7).fuse() => (),
    }
}

pub async fn wait_up(cx: &AnimationEngineContext) {
    select! {
        _ = cx.wait_key_down(KeyCode::Up).fuse() => (),
        _ = cx.wait_button_down(Button::DPadUp).fuse() => (),
        _ = wait_greater_axis_value(cx, Axis::LeftStickY, 0.7).fuse() => (),
    }
}

pub async fn wait_down(cx: &AnimationEngineContext) {
    select! {
        _ = cx.wait_key_down(KeyCode::Down).fuse() => (),
        _ = cx.wait_button_down(Button::DPadDown).fuse() => (),
        _ = wait_less_axis_value(cx, Axis::LeftStickY, -0.7).fuse() => (),
    }
}
