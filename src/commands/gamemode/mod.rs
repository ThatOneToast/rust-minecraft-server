use valence::{
    client::{Client, Username},
    command::{
        handler::CommandResultEvent,
        parsers::{entity_selector::EntitySelectors, EntitySelector},
    },
    command_macros::Command,
    entity::Position,
    prelude::*,
    rand::{self, seq::IteratorRandom},
};

/// FROM VALENCE EXAMPLE
/// https://github.com/valence-rs/valence/blob/main/examples/command.rs

#[derive(Command, Debug, Clone)]
#[paths("gamemode", "gm")]
#[scopes("valence.command.gamemode")]
pub enum Command {
    #[paths("survival {target?}", "{/} gms {target?}")]
    Survival { target: Option<EntitySelector> },
    #[paths("creative {target?}", "{/} gmc {target?}")]
    Creative { target: Option<EntitySelector> },
    #[paths("adventure {target?}", "{/} gma {target?}")]
    Adventure { target: Option<EntitySelector> },
    #[paths("spectator {target?}", "{/} gmspec {target?}")]
    Spectator { target: Option<EntitySelector> },
}

#[derive(Command, Debug, Clone)]
#[paths("struct {gamemode} {target?}")]
#[scopes("valence.command.gamemode")]
#[allow(dead_code)]
pub(crate) struct GameModeStructCommand {
    gamemode: GameMode,
    target: Option<EntitySelector>,
}

pub fn handle(
    mut events: EventReader<CommandResultEvent<Command>>,
    mut clients: Query<(&mut Client, &mut GameMode, &Username, Entity)>,
    positions: Query<&Position>,
) {
    for event in events.read() {
        let game_mode_to_set = match &event.result {
            Command::Survival { .. } => GameMode::Survival,
            Command::Creative { .. } => GameMode::Creative,
            Command::Adventure { .. } => GameMode::Adventure,
            Command::Spectator { .. } => GameMode::Spectator,
        };

        let selector = match &event.result {
            Command::Survival { target } => target.clone(),
            Command::Creative { target } => target.clone(),
            Command::Adventure { target } => target.clone(),
            Command::Spectator { target } => target.clone(),
        };

        match selector {
            None => {
                let (mut client, mut game_mode, ..) = clients.get_mut(event.executor).unwrap();
                *game_mode = game_mode_to_set;
                client.send_chat_message(format!(
                    "Gamemode command executor -> self executed with data:\n {:#?}",
                    &event.result
                ));
            }
            Some(selector) => match selector {
                EntitySelector::SimpleSelector(selector) => match selector {
                    EntitySelectors::AllEntities => {
                        for (mut client, mut game_mode, ..) in &mut clients.iter_mut() {
                            *game_mode = game_mode_to_set;
                            client.send_chat_message(format!(
                                "Gamemode command executor -> all entities executed with data:\n \
                                 {:#?}",
                                &event.result
                            ));
                        }
                    }
                    EntitySelectors::SinglePlayer(name) => {
                        let target = clients
                            .iter_mut()
                            .find(|(.., username, _)| username.0 == *name)
                            .map(|(.., target)| target);

                        match target {
                            None => {
                                let client = &mut clients.get_mut(event.executor).unwrap().0;
                                client.send_chat_message(format!("Could not find target: {name}"));
                            }
                            Some(target) => {
                                let mut game_mode = clients.get_mut(target).unwrap().1;
                                *game_mode = game_mode_to_set;

                                let client = &mut clients.get_mut(event.executor).unwrap().0;
                                client.send_chat_message(format!(
                                    "Gamemode command executor -> single player executed with \
                                     data:\n {:#?}",
                                    &event.result
                                ));
                            }
                        }
                    }
                    EntitySelectors::AllPlayers => {
                        for (mut client, mut game_mode, ..) in &mut clients.iter_mut() {
                            *game_mode = game_mode_to_set;
                            client.send_chat_message(format!(
                                "Gamemode command executor -> all entities executed with data:\n \
                                 {:#?}",
                                &event.result
                            ));
                        }
                    }
                    EntitySelectors::SelfPlayer => {
                        let (mut client, mut game_mode, ..) =
                            clients.get_mut(event.executor).unwrap();
                        *game_mode = game_mode_to_set;
                        client.send_chat_message(format!(
                            "Gamemode command executor -> self executed with data:\n {:#?}",
                            &event.result
                        ));
                    }
                    EntitySelectors::NearestPlayer => {
                        let executor_pos = positions.get(event.executor).unwrap();
                        let target = clients
                            .iter_mut()
                            .filter(|(.., target)| *target != event.executor)
                            .min_by(|(.., target), (.., target2)| {
                                let target_pos = positions.get(*target).unwrap();
                                let target2_pos = positions.get(*target2).unwrap();
                                let target_dist = target_pos.distance(**executor_pos);
                                let target2_dist = target2_pos.distance(**executor_pos);
                                target_dist.partial_cmp(&target2_dist).unwrap()
                            })
                            .map(|(.., target)| target);

                        match target {
                            None => {
                                let client = &mut clients.get_mut(event.executor).unwrap().0;
                                client.send_chat_message("Could not find target".to_owned());
                            }
                            Some(target) => {
                                let mut game_mode = clients.get_mut(target).unwrap().1;
                                *game_mode = game_mode_to_set;

                                let client = &mut clients.get_mut(event.executor).unwrap().0;
                                client.send_chat_message(format!(
                                    "Gamemode command executor -> single player executed with \
                                     data:\n {:#?}",
                                    &event.result
                                ));
                            }
                        }
                    }
                    EntitySelectors::RandomPlayer => {
                        let target = clients
                            .iter_mut()
                            .choose(&mut rand::thread_rng())
                            .map(|(.., target)| target);

                        match target {
                            None => {
                                let client = &mut clients.get_mut(event.executor).unwrap().0;
                                client.send_chat_message("Could not find target".to_owned());
                            }
                            Some(target) => {
                                let mut game_mode = clients.get_mut(target).unwrap().1;
                                *game_mode = game_mode_to_set;

                                let client = &mut clients.get_mut(event.executor).unwrap().0;
                                client.send_chat_message(format!(
                                    "Gamemode command executor -> single player executed with \
                                     data:\n {:#?}",
                                    &event.result
                                ));
                            }
                        }
                    }
                },
                EntitySelector::ComplexSelector(_, _) => {
                    let client = &mut clients.get_mut(event.executor).unwrap().0;
                    client
                        .send_chat_message("Complex selectors are not implemented yet".to_owned());
                }
            },
        }
    }
}
