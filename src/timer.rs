use std::time::Duration;

use macroquad::time::get_frame_time;

#[derive(PartialEq)]
pub enum TimerState {
    Running,
    Paused,
    Finished,
}

pub struct Timer {
    duration: Duration,
    elapsed: f32,
    state: TimerState,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            elapsed: 0.0,
            state: TimerState::Running,
        }
    }

    pub fn tick(&mut self) {
        if self.state != TimerState::Running {
            return;
        }

        self.elapsed += get_frame_time();

        if self.elapsed >= self.duration.as_secs_f32() {
            self.finish();
        }
    }

    pub fn pause(&mut self) {
        self.state = TimerState::Paused;
    }

    pub fn finish(&mut self) {
        self.state = TimerState::Finished;
    }

    pub fn start(&mut self) {
        self.state = TimerState::Running;
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
    }

    pub fn get_state(&self) -> &TimerState {
        &self.state
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.state, TimerState::Finished)
    }

    pub fn get_elapsed(&self) -> f32 {
        self.elapsed
    }
}
