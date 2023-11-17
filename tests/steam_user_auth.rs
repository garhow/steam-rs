use steam_rs::Steam;

mod common;

#[test]
pub fn authenticate_user_ticket() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        //12900 = Audiosurf
        let query = steam
            .authenticate_user_ticket(
                12900,
                &std::env::var("STEAM_GAME_TICKET").expect("Ticket should be provided"),
            )
            .await
            .unwrap();
        println!("{:?}", query);
    });
}
