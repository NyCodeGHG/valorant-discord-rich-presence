use discord_sdk::{
    user::User,
    wheel::{UserState, Wheel},
    Discord, DiscordApp, Subscriptions,
};

pub struct DiscordPresence {
    pub discord: Discord,
    pub user: User,
    pub wheel: Wheel,
}

impl DiscordPresence {
    pub async fn new(client_id: i64) -> DiscordPresence {
        let (wheel, handler) = Wheel::new(Box::new(|err| {
            eprintln!("{}", err);
        }));
        let mut user = wheel.user();
        let discord = Discord::new(
            DiscordApp::PlainId(client_id),
            Subscriptions::ALL,
            Box::new(handler),
        )
        .expect("unable to create discord client.");

        println!("waiting for handshake...");
        user.0.changed().await.unwrap();

        let user = match &*user.0.borrow() {
            UserState::Connected(user) => user.clone(),
            UserState::Disconnected(err) => panic!("failed to connect to Discord: {}", err),
        };

        println!("connected to Discord, local user is {:#?}", user);

        DiscordPresence {
            discord,
            user,
            wheel,
        }
    }
}
