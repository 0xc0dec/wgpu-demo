use crate::frame_time::FrameTime;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct State {
    pub running: bool,
    pub frame_time: FrameTime,
}
