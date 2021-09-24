use bevy::prelude::*;
use crate::prelude::*;

pub fn toggle_visibility(
    mut events: EventReader<HiltToggleVisibility>,
    mut collider_query: Query<&mut Visible, (
        With<HiltDebugRenderCollider>,
        Without<HiltDebugRenderPosition>,
        Without<HiltDebugRenderPath>
    )>,
    mut positions_query: Query<&mut Visible, (
        With<HiltDebugRenderPosition>,
        Without<HiltDebugRenderCollider>,
        Without<HiltDebugRenderPath>
    )>,
    mut paths_query: Query<&mut Visible, (
        With<HiltDebugRenderPath>,
        Without<HiltDebugRenderCollider>,
        Without<HiltDebugPosition>
    )>
) {
    for event in events.iter() {
        debug!("toggeling visibility on entities: {:?}", event);
        match event.0 {
            HiltEntities::All => {
                for mut visible in collider_query.iter_mut() {
                    visible.is_visible = !visible.is_visible;
                }
                for mut visible in positions_query.iter_mut() {
                    visible.is_visible = !visible.is_visible;
                }
                for mut visible in paths_query.iter_mut() {
                    visible.is_visible = !visible.is_visible;
                }
            },
            HiltEntities::Colliders => {
                for mut visible in collider_query.iter_mut() {
                    visible.is_visible = !visible.is_visible;
                }
            },
            HiltEntities::Positions => {
                for mut visible in positions_query.iter_mut() {
                    visible.is_visible = !visible.is_visible;
                }
            },
            HiltEntities::Path => {
                for mut visible in paths_query.iter_mut() {
                    visible.is_visible = !visible.is_visible;
                }
            }
        }
    }
}
