use crate::app::App;

pub struct FrameContext<'a> {
    pub dt: f32,
    pub app: &'a App,
}
