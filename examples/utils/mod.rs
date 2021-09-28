use bevy::prelude::*;
use bevy_devtools::DevToolsExt;

#[cfg(feature = "3d")]
mod rotates;

mod debug;

mod environment;
pub use self::environment::spawn_environment;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_devtools::DevToolsPlugin::<()>::default())
            .devtools_enabled()
            .add_startup_system(spawn_environment.system().label("environment"))
            .add_system(debug::show_colliders_changed.system())
            .add_system(debug::show_render_pass_changed.system())
            .devtools_setting(debug::settings());

        #[cfg(feature = "3d")]
        app.add_system(rotates::rotator_system.system());
    }
}
