use std::{net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};
use bevy::prelude::*;
use bevy_renet::{RenetClient, RenetClientPlugin, netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport}};
use renet::{ConnectionConfig, DefaultChannel};

const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000u16);

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, 
        app: &mut App) {
        app.add_plugins(RenetClientPlugin);

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

        app.add_systems(Update, (recieve_message_system, send_message_system));
    }
}

fn send_message_system(mut client: ResMut<RenetClient>) {
    client.send_message(DefaultChannel::ReliableOrdered, "Hello World!");
}

fn recieve_message_system(mut client:ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        println!("Server responded {}", String::from_utf8(message.to_vec()).unwrap());
    }
}
