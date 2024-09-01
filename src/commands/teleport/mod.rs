

use valence::command_macros::Command;
use valence::command::parsers::{EntitySelector, Vec3 as Vec3Parser};
use valence::prelude::Entity;



#[derive(Command, Debug, Clone)]
#[paths("teleport", "tp")]
#[scopes("command.teleport")]
pub enum Command {
    #[paths = "{location}"]
    ExecutorToLocation { location: Vec3Parser },
    #[paths = "{target}"]
    ExecutorToTarget { target: EntitySelector },
    #[paths = "{from} {to}"]
    TargetToTarget {
        from: EntitySelector,
        to: EntitySelector,
    },
    #[paths = "{target} {location}"]
    TargetToLocation {
        target: EntitySelector,
        location: Vec3Parser,
    },
}

pub enum TeleportTarget {
    Targets(Vec<Entity>),
}

#[derive(Debug)]
pub enum TeleportDestination {
    Location(Vec3Parser),
    Target(Option<Entity>),
}

use valence::command::parsers::entity_selector::EntitySelectors;
use valence::rand;
use valence::rand::seq::IteratorRandom;
use valence::{entity::living::LivingEntity, prelude::*};
use valence::command::handler::CommandResultEvent;

pub fn handle(
    mut events: EventReader<CommandResultEvent<Command>>,
    living_entities: Query<Entity, With<LivingEntity>>,
    mut clients: Query<(Entity, &mut Client)>,
    entity_layers: Query<&EntityLayerId>,
    mut positions: Query<&mut Position>,
    usernames: Query<(Entity, &Username)>,
) {
    for event in events.read() {
        let compiled_command = match &event.result {
            Command::ExecutorToLocation { location } => (
                TeleportTarget::Targets(vec![event.executor]),
                TeleportDestination::Location(*location),
            ),
            Command::ExecutorToTarget { target } => (
                TeleportTarget::Targets(vec![event.executor]),
                TeleportDestination::Target(
                    find_targets(
                        &living_entities,
                        &mut clients,
                        &positions,
                        &entity_layers,
                        &usernames,
                        event,
                        target,
                    )
                    .first()
                    .copied(),
                ),
            ),
            Command::TargetToTarget { from, to } => (
                TeleportTarget::Targets(
                    find_targets(
                        &living_entities,
                        &mut clients,
                        &positions,
                        &entity_layers,
                        &usernames,
                        event,
                        from,
                    )
                    .clone(),
                ),
                TeleportDestination::Target(
                    find_targets(
                        &living_entities,
                        &mut clients,
                        &positions,
                        &entity_layers,
                        &usernames,
                        event,
                        to,
                    )
                    .first()
                    .copied(),
                ),
            ),
            Command::TargetToLocation { target, location } => (
                TeleportTarget::Targets(
                    find_targets(
                        &living_entities,
                        &mut clients,
                        &positions,
                        &entity_layers,
                        &usernames,
                        event,
                        target,
                    )
                    .clone(),
                ),
                TeleportDestination::Location(*location),
            ),
        };

        let (TeleportTarget::Targets(targets), destination) = compiled_command;

        println!("executing teleport command {targets:#?} -> {destination:#?}");
        match destination {
            TeleportDestination::Location(location) => {
                for target in targets {
                    let mut pos = positions.get_mut(target).unwrap();
                    pos.0.x = f64::from(location.x.get(pos.0.x as f32));
                    pos.0.y = f64::from(location.y.get(pos.0.y as f32));
                    pos.0.z = f64::from(location.z.get(pos.0.z as f32));
                }
            }
            TeleportDestination::Target(target) => {
                let target = target.unwrap();
                let target_pos = **positions.get(target).unwrap();
                for target in targets {
                    let mut position = positions.get_mut(target).unwrap();
                    position.0 = target_pos;
                }
            }
        }
    }
    
    fn find_targets(
        living_entities: &Query<Entity, With<LivingEntity>>,
        clients: &mut Query<(Entity, &mut Client)>,
        positions: &Query<&mut Position>,
        entity_layers: &Query<&EntityLayerId>,
        usernames: &Query<(Entity, &Username)>,
        event: &CommandResultEvent<Command>,
        target: &EntitySelector,
    ) -> Vec<Entity> {
        match target {
            EntitySelector::SimpleSelector(selector) => match selector {
                EntitySelectors::AllEntities => {
                    let executor_entity_layer = *entity_layers.get(event.executor).unwrap();
                    living_entities
                        .iter()
                        .filter(|entity| {
                            let entity_layer = entity_layers.get(*entity).unwrap();
                            entity_layer.0 == executor_entity_layer.0
                        })
                        .collect()
                }
                EntitySelectors::SinglePlayer(name) => {
                    let target = usernames.iter().find(|(_, username)| username.0 == *name);
                    match target {
                        None => {
                            let client = &mut clients.get_mut(event.executor).unwrap().1;
                            client.send_chat_message(format!("Could not find target: {name}"));
                            vec![]
                        }
                        Some(target_entity) => {
                            vec![target_entity.0]
                        }
                    }
                }
                EntitySelectors::AllPlayers => {
                    let executor_entity_layer = *entity_layers.get(event.executor).unwrap();
                    clients
                        .iter_mut()
                        .filter_map(|(entity, ..)| {
                            let entity_layer = entity_layers.get(entity).unwrap();
                            if entity_layer.0 == executor_entity_layer.0 {
                                Some(entity)
                            } else {
                                None
                            }
                        })
                        .collect()
                }
                EntitySelectors::SelfPlayer => {
                    vec![event.executor]
                }
                EntitySelectors::NearestPlayer => {
                    let executor_entity_layer = *entity_layers.get(event.executor).unwrap();
                    let executor_pos = positions.get(event.executor).unwrap();
                    let target = clients
                        .iter_mut()
                        .filter(|(entity, ..)| {
                            *entity_layers.get(*entity).unwrap() == executor_entity_layer
                        })
                        .filter(|(target, ..)| *target != event.executor)
                        .map(|(target, ..)| target)
                        .min_by(|target, target2| {
                            let target_pos = positions.get(*target).unwrap();
                            let target2_pos = positions.get(*target2).unwrap();
                            let target_dist = target_pos.distance(**executor_pos);
                            let target2_dist = target2_pos.distance(**executor_pos);
                            target_dist.partial_cmp(&target2_dist).unwrap()
                        });
                    match target {
                        None => {
                            let mut client = clients.get_mut(event.executor).unwrap().1;
                            client.send_chat_message("Could not find target".to_owned());
                            vec![]
                        }
                        Some(target_entity) => {
                            vec![target_entity]
                        }
                    }
                }
                EntitySelectors::RandomPlayer => {
                    let executor_entity_layer = *entity_layers.get(event.executor).unwrap();
                    let target = clients
                        .iter_mut()
                        .filter(|(entity, ..)| {
                            *entity_layers.get(*entity).unwrap() == executor_entity_layer
                        })
                        .choose(&mut rand::thread_rng())
                        .map(|(target, ..)| target);
                    match target {
                        None => {
                            let mut client = clients.get_mut(event.executor).unwrap().1;
                            client.send_chat_message("Could not find target".to_owned());
                            vec![]
                        }
                        Some(target_entity) => {
                            vec![target_entity]
                        }
                    }
                }
            },
            EntitySelector::ComplexSelector(_, _) => {
                let mut client = clients.get_mut(event.executor).unwrap().1;
                client.send_chat_message("complex selector not implemented".to_owned());
                vec![]
            }
        }
    }

}

