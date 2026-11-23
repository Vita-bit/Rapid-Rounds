use std::{net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};
use bevy::prelude::*;
use bevy_renet::{RenetClient, RenetClientPlugin, client_connected, netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport}};
use renet::{Bytes, ConnectionConfig, DefaultChannel};
use crate::client::response::plugin::ResponsePlugin;

const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000u16);

#[derive(Message)]
pub struct SendToServer {
    pub channel: u8,
    pub payload: Bytes,
}

impl SendToServer {
    pub fn new(channel: DefaultChannel, payload: impl Into<Bytes>) -> Self {
        Self { channel: channel.into(), payload: payload.into()}
    }
}

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, 
        app: &mut App) {
        app.add_plugins((RenetClientPlugin, ResponsePlugin));

        let client = RenetClient::new(ConnectionConfig::default());
        app.insert_resource(client);

        app.add_plugins(NetcodeClientPlugin);

        let auth = ClientAuthentication::Unsecure {
            server_addr: SERVER_ADDR,
            client_id: 0,
            user_data: None,
            protocol_id: 0,
        };
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let transport = NetcodeClientTransport::new(current_time, auth, socket).unwrap();
        app.insert_resource(transport);

        app.add_message::<SendToServer>();
        app.add_systems(Update, send_message_system.run_if(client_connected));
    }
}

fn send_message_system(mut client: ResMut<RenetClient>, mut outgoing: MessageReader<SendToServer>) {
    for msg in outgoing.read() {
        client.send_message(msg.channel, msg.payload.clone());
    }
}
