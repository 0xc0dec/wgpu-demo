use crate::camera::Camera;
use crate::device::{Frame};
use crate::frame_context::FrameContext;
use crate::physics_world::PhysicsWorld;
use crate::scene::character::Character;
use crate::scene::skybox::Skybox;
use crate::scene::test_entity::{TestEntity, TestEntityParams};
use crate::scene::tracer::Tracer;
use cgmath::{Array, Deg, Vector3, Zero};
use crate::app::App;

pub struct Scene {
    character: Character,
    tracer: Tracer,
    skybox: Skybox,
    entities: Vec<TestEntity>,
    physics: PhysicsWorld,
}

impl Scene {
    pub async fn new(app: &mut App) -> Scene {
        let mut physics = PhysicsWorld::new();

        let ground = TestEntity::new(
            app,
            &mut physics,
            TestEntityParams {
                pos: Vector3::zero(),
                scale: Vector3::new(10.0, 0.5, 10.0),
                rotation_axis: Vector3::zero(),
                rotation_angle: Deg(0.0),
                movable: false,
            },
        )
        .await;

        let falling_box = TestEntity::new(
            app,
            &mut physics,
            TestEntityParams {
                pos: Vector3::unit_y() * 15.0,
                scale: Vector3::new(1.0, 1.0, 1.0),
                rotation_axis: Vector3::from_value(1.0),
                rotation_angle: Deg(50.0),
                movable: true,
            },
        )
        .await;

        let character = Character::new(
            Camera::new(
                Vector3::new(10.0, 10.0, 10.0),
                Vector3::new(0.0, 0.0, 0.0),
                app.device.surface_size().into(),
            ),
            &mut physics,
        );

        let tracer = Tracer::new(app).await;
        let skybox = Skybox::new(&app.device).await;

        Self {
            physics,
            character,
            tracer,
            skybox,
            entities: vec![ground, falling_box],
        }
    }

    pub fn update(&mut self, ctx: &FrameContext) {
        self.physics.update(ctx.dt);

        self.character.update(ctx, &mut self.physics);
        self.tracer.update(&self.physics, &self.character);

        for e in &mut self.entities {
            e.update(ctx.dt, &self.physics);
        }
    }

    pub fn render<'a, 'b>(&'a mut self, frame: &mut Frame<'b, 'a>, ctx: &'a FrameContext)
    where
        'a: 'b,
    {
        // TODO Do this only when the size changes
        self.character.camera.set_fov(
            ctx.device.surface_size().width as f32,
            ctx.device.surface_size().height as f32,
        );

        self.skybox
            .render(ctx.device, &self.character.camera, frame);

        for e in &mut self.entities {
            e.render(ctx.device, &self.character.camera, frame);
        }

        self.tracer
            .render(ctx.device, &self.character.camera, frame);
    }

    pub fn build_debug_ui(&self, ui_frame: &mut imgui::Ui) {
        ui_frame
            .window("Debug info")
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .movable(false)
            .resizable(false)
            .always_auto_resize(true)
            .collapsible(false)
            .no_decoration()
            .build(|| {
                let mouse_pos = ui_frame.io().mouse_pos;
                ui_frame.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    }
}
