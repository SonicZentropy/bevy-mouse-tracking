use bevy::prelude::*;

/// Plugin that tracks mouse motion.
pub struct MouseMotionPlugin;

#[derive(Debug, Resource, Clone, Copy, PartialEq)]
pub struct MouseMotion {
    pub delta: Vec2,
}

impl bevy::app::Plugin for MouseMotionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(MouseMotion { delta: Vec2::ZERO });
        app.add_system_to_stage(
            CoreStage::First,
            update_mouse_motion.after(Events::<MouseMotion>::update_system),
        );
    }
}

fn update_mouse_motion(mut events: EventReader<MouseMotion>, mut res: ResMut<MouseMotion>) {
    let delta = events.iter().fold(Vec2::ZERO, |acc, e| acc + e.delta);
    *res = MouseMotion { delta };
}
