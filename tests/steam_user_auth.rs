use steam_rs::Steam;

mod common;

#[test]
pub fn authenticate_user_ticket() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        let app_id = 12900; // Audiosurf
        let ticket = &std::env::var("STEAM_GAME_TICKET").expect("Ticket should be provided");

        let query = steam
            .authenticate_user_ticket(
                app_id,
                ticket,
            )
            .await
            .unwrap();

        println!("{:?}", query);
    });
}
