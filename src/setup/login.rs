use std::net::SocketAddr;
use std::sync::atomic::Ordering;

use valence::network::CleanupFn;
use valence::{prelude::*, PROTOCOL_VERSION};
use valence::{
    network::{async_trait, BroadcastToLan, HandshakeData, ServerListPing},
    text::IntoText,
    MINECRAFT_VERSION,
};

pub struct MyCallbacks;

#[async_trait]
impl NetworkCallbacks for MyCallbacks {
    async fn server_list_ping(
        &self,
        shared: &SharedNetworkState,
        remote_addr: SocketAddr,
        handshake_data: &HandshakeData,
    ) -> ServerListPing {
        #![allow(unused_variables)]

        ServerListPing::Respond {
            online_players: shared.player_count().load(Ordering::Relaxed) as i32,
            max_players: 420 as i32,
            player_sample: vec![],
            description: "A Valence Server".into_text(),
            favicon_png: &[],
            version_name: MINECRAFT_VERSION.to_owned(),
            protocol: PROTOCOL_VERSION,
        }
    }

    async fn broadcast_to_lan(&self, _shared: &SharedNetworkState) -> BroadcastToLan {
        BroadcastToLan::Enabled("Rust Minecraft Server!".into())
    }
    
    async fn login(
        &self,
        shared: &SharedNetworkState,
        info: &NewClientInfo,
    ) -> Result<CleanupFn, Text> {
        let _ = info;

        let max_players = shared.max_players();

        let success = shared
            .player_count()
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| {
                (n < max_players).then_some(n + 1)
            })
            .is_ok();

        if success {
            let shared = shared.clone();

            Ok(Box::new(move || {
                let prev = shared.player_count().fetch_sub(1, Ordering::SeqCst);
                debug_assert_ne!(prev, 0, "player count underflowed");
            }))
        } else {
            Err("Server Full".into_text())
        }
    }
}
