use bevy::prelude::*;
use bevy::render::render_graph::base::MainPass;
use crate::prelude::*;
use crate::render::HiltPass;

pub fn toggle_render_pass(
    mut commands: Commands,
    mut events: EventReader<HiltToggleRenderPass>,
    mut collider_query: Query<(Entity, Option<&MainPass>, Option<&HiltPass>), (
        With<HiltDebugRenderCollider>,
        Without<HiltDebugRenderPosition>,
        Without<HiltDebugRenderPath>
    )>,
    mut positions_query: Query<(Entity, Option<&MainPass>, Option<&HiltPass>), (
        With<HiltDebugRenderPosition>,
        Without<HiltDebugRenderPath>,
        Without<HiltDebugRenderCollider>
    )>,
    mut paths_query: Query<(Entity, Option<&MainPass>, Option<&HiltPass>), (
        With<HiltDebugRenderPath>,
        Without<HiltDebugRenderCollider>,
        Without<HiltDebugRenderPosition>
    )>,
) {
    for event in events.iter() {
        debug!("toggeling hilt render pass on entities: {:?}", event);
        match event.0 {
            HiltEntities::All => {
                for (entity, main_pass, hilt_pass) in collider_query.iter_mut() {
                    if hilt_pass.is_some() && main_pass.is_none() {
                        commands.entity(entity)
                            .remove::<HiltPass>()
                            .insert(MainPass);
                    } else if hilt_pass.is_none() && main_pass.is_some() {
                        commands.entity(entity)
                            .remove::<MainPass>()
                            .insert(HiltPass);
                    }
                }
                for (entity, main_pass, hilt_pass) in positions_query.iter_mut() {
                    if hilt_pass.is_some() && main_pass.is_none() {
                        commands.entity(entity)
                            .remove::<HiltPass>()
                            .insert(MainPass);
                    } else if hilt_pass.is_none() && main_pass.is_some() {
                        commands.entity(entity)
                            .remove::<MainPass>()
                            .insert(HiltPass);
                    }
                }
                for (entity, main_pass, hilt_pass) in paths_query.iter_mut() {
                    if hilt_pass.is_some() && main_pass.is_none() {
                        commands.entity(entity)
                            .remove::<HiltPass>()
                            .insert(MainPass);
                    } else if hilt_pass.is_none() && main_pass.is_some() {
                        commands.entity(entity)
                            .remove::<MainPass>()
                            .insert(HiltPass);
                    }
                }
            },
            HiltEntities::Colliders => {
                for (entity, main_pass, hilt_pass) in collider_query.iter_mut() {
                    if hilt_pass.is_some() && main_pass.is_none() {
                        commands.entity(entity)
                            .remove::<HiltPass>()
                            .insert(MainPass);
                    } else if hilt_pass.is_none() && main_pass.is_some() {
                        commands.entity(entity)
                            .remove::<MainPass>()
                            .insert(HiltPass);
                    }
                }
            },
            HiltEntities::Positions => {
                for (entity, main_pass, hilt_pass) in positions_query.iter_mut() {
                    if hilt_pass.is_some() && main_pass.is_none() {
                        commands.entity(entity)
                            .remove::<HiltPass>()
                            .insert(MainPass);
                    } else if hilt_pass.is_none() && main_pass.is_some() {
                        commands.entity(entity)
                            .remove::<MainPass>()
                            .insert(HiltPass);
                    }
                }
            },
            HiltEntities::Path => {
                for (entity, main_pass, hilt_pass) in paths_query.iter_mut() {
                    if hilt_pass.is_some() && main_pass.is_none() {
                        commands.entity(entity)
                            .remove::<HiltPass>()
                            .insert(MainPass);
                    } else if hilt_pass.is_none() && main_pass.is_some() {
                        commands.entity(entity)
                            .remove::<MainPass>()
                            .insert(HiltPass);
                    }
                }
            }
        }
    }
}
