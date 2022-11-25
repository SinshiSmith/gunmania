use bevy::prelude::*;
pub struct CustomInspectorPlugin;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::CombatStats;

impl Plugin for CustomInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<CombatStats>();
    }
}
