use bevy::prelude::*;
use bevy_renet::RenetClient;
use renet::DefaultChannel;
use crate::client::plugin::SendToServer;

pub struct ResponsePlugin;

impl Plugin for ResponsePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, recieve_message_system);
    }
}

fn recieve_message_system(mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let text = String::from_utf8(message.to_vec()).unwrap();
        match text.split_once(" ") {
            Some((identifier, rest)) => println!("Server sent {identifier}: {rest}"),
            _ => println!("Server sent: {text}"),
        }

        respond(&text);
    }
}

fn respond(text: &str) -> Option<SendToServer> {
    let (identifier, _rest) = text.split_once(" ").unwrap_or((text, " "));

    match identifier {
        _ => None,
    }
}
