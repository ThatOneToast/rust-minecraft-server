use std::net::SocketAddr;

use valence::prelude::*;
use valence::{
    network::{
        async_trait, BroadcastToLan, HandshakeData, PlayerSampleEntry, ServerListPing,
    },
    text::{Color, IntoText},
    uuid::Uuid,
    MINECRAFT_VERSION,
};


pub struct MyCallbacks;

#[async_trait]
impl NetworkCallbacks for MyCallbacks {
    async fn server_list_ping(
        &self,
        _shared: &SharedNetworkState,
        remote_addr: SocketAddr,
        handshake_data: &HandshakeData,
    ) -> ServerListPing {
        let max_players = 420;

        ServerListPing::Respond {
            online_players: 0,
            max_players,
            player_sample: vec![PlayerSampleEntry {
                name: "foobar".into(),
                id: Uuid::from_u128(12345),
            }],
            description: "Your IP address is ".into_text()
                + remote_addr.to_string().color(Color::rgb(50, 50, 250)),
            favicon_png: include_bytes!("../../assets/server-icon.png"),
            version_name: ("Valence ".color(Color::GOLD) + MINECRAFT_VERSION.color(Color::RED))
                .to_legacy_lossy(),
            protocol: handshake_data.protocol_version,
        }
    }

    async fn broadcast_to_lan(&self, _shared: &SharedNetworkState) -> BroadcastToLan {
        BroadcastToLan::Enabled("Hello Valence!".into())
    }

}
