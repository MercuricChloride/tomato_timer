#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod colors;
mod sounds;
use sounds::{finish_sound, start_sound};
use std::{
    thread,
    time::{Duration, SystemTime},
};

pub use app::{TimerStatus, TomatoTimer};

pub fn get_elapsed_time(status: &TimerStatus) -> f32 {
    let current_time = SystemTime::now();
    match status {
        TimerStatus::Running(start_time) => current_time
            .duration_since(*start_time)
            .unwrap()
            .as_secs_f32(),
        TimerStatus::Break(break_start) => current_time
            .duration_since(*break_start)
            .unwrap()
            .as_secs_f32(),
        _ => 0.0,
    }
}

pub fn get_remaining_time(status: &TimerStatus, time_per_round: &f32, time_per_break: &f32) -> f32 {
    let elapsed_time = get_elapsed_time(status);
    match status {
        TimerStatus::Running(_) => (*time_per_round * 60.0) - elapsed_time,
        TimerStatus::Break(_) => (*time_per_break * 60.0) - elapsed_time,
        _ => 0.0,
    }
}

pub fn get_is_round_complete(
    status: &TimerStatus,
    time_per_round: &f32,
    time_per_break: &f32,
) -> bool {
    let remaining_time = get_remaining_time(status, time_per_round, time_per_break);
    remaining_time <= 0.0
}

pub fn handle_round_complete(status: &mut TimerStatus, round_complete: bool) {
    if round_complete {
        match status {
            TimerStatus::Running(_) => {
                notifica::notify("Time is up!", "Take a break").expect("Failed to notify");
                *status = TimerStatus::Break(SystemTime::now());
                thread::spawn(|| {
                    finish_sound();
                });
            }
            TimerStatus::Break(_) => {
                notifica::notify("Back to work!", "Start focusing again :)")
                    .expect("Failed to notify");
                *status = TimerStatus::Running(SystemTime::now());
                thread::spawn(|| {
                    start_sound();
                });
            }
            _ => {}
        }
    }
}

pub fn display_time_remaining(remaining_time: f32) -> String {
    match remaining_time as u32 {
        60..=u32::MAX => format!(
            "{} Minutes left in round",
            (Duration::from_secs_f32(remaining_time).as_secs() / 60) + 1
        ),
        1..=59 => format!(
            "{} Seconds left in round",
            Duration::from_secs_f32(remaining_time).as_secs()
        ),
        _ => format!("Time is up!"),
    }
}
