use bevy::prelude::*;
use bevy_devtools::DevToolsExt;

mod rotates;
pub use self::rotates::Rotates;

mod environment;
pub use self::environment::spawn_environment;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(rotates::rotator_system.system())
            .add_plugin(bevy_devtools::DevToolsPlugin::<()>::default())
            .devtools_enabled()
            .add_startup_system(spawn_environment.system().label("environment"));
    }
}
