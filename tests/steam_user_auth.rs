use steam_rs::Steam;

mod common;

#[test]
pub fn authenticate_user_ticket() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        let app_id = 12900; // Audiosurf
        let ticket = &std::env::var("STEAM_GAME_TICKET").unwrap_or(String::new());

        if !ticket.is_empty() {
            assert!(
                steam
                    .authenticate_user_ticket(
                        app_id,
                        ticket,
                    )
                    .await
                    .is_ok()
            );
        }
    });
}
