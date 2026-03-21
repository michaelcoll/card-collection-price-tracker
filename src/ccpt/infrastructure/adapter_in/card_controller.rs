use crate::application::error::AppError;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::body::to_bytes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;

pub fn create_card_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/import", post(import_cards))
        .route("/card-info", post(get_card_info))
}

async fn import_cards(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    body: axum::body::Body,
) -> Result<String, AppError> {
    let bytes = to_bytes(body, 10 * 1024 * 1024)
        .await
        .map_err(|e| AppError::WrongFormat(format!("Failed to read body: {}", e)))?;

    let csv = String::from_utf8(bytes.to_vec())
        .map_err(|_| AppError::WrongFormat("Body is not valid UTF-8".to_string()))?;

    // L'user.id est maintenant disponible pour associer les données importées à l'utilisateur
    println!("Importing cards for user: {} ({})", user.email, user.id);

    state
        .import_card_use_case
        .clone()
        .import_cards(&csv, user)
        .await?;

    Ok("Cards imported successfully".to_string())
}

async fn get_card_info(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    println!("Getting card info for user: {} ({})", user.email, user.id);

    state
        .edh_rec_caller_adapter
        .get_card_info("Sol Ring".to_string())
        .await
        .expect("panic message");

    Ok("card Info".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;
    use axum::body::Body;

    #[tokio::test]
    async fn import_cards_succeeds_with_valid_csv() {
        let app_state = AppState::for_testing(std::sync::Arc::new(
            crate::application::use_case::MockStatsUseCase::new(),
        ));

        let csv_body = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR";

        let test_user = User::for_testing();
        let result = import_cards(
            AuthenticatedUser(test_user),
            State(app_state),
            Body::from(csv_body),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Cards imported successfully");
    }

    #[tokio::test]
    async fn import_cards_fails_with_invalid_utf8() {
        let app_state = AppState::for_testing(std::sync::Arc::new(
            crate::application::use_case::MockStatsUseCase::new(),
        ));

        // Create invalid UTF-8 bytes
        let invalid_bytes = vec![0xFF, 0xFE, 0xFD];

        let test_user = User::for_testing();
        let result = import_cards(
            AuthenticatedUser(test_user),
            State(app_state),
            Body::from(invalid_bytes),
        )
        .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::WrongFormat(msg) => {
                assert_eq!(msg, "Body is not valid UTF-8");
            }
            _ => panic!("Expected WrongFormat error"),
        }
    }

    #[tokio::test]
    async fn import_cards_succeeds_with_multiple_cards() {
        let app_state = AppState::for_testing(std::sync::Arc::new(
            crate::application::use_case::MockStatsUseCase::new(),
        ));

        let csv_body = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR\n\
        bulk,binder,Repeal,GPT,Guildpact,32,normal,common,2,27563,9e7dd929-4bba-46a6-86c9-b8ed853eb721,0.17,false,false,near_mint,fr,EUR";

        let test_user = User::for_testing();
        let result = import_cards(
            AuthenticatedUser(test_user),
            State(app_state),
            Body::from(csv_body),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Cards imported successfully");
    }

    #[tokio::test]
    async fn import_cards_succeeds_with_foil_cards() {
        let app_state = AppState::for_testing(std::sync::Arc::new(
            crate::application::use_case::MockStatsUseCase::new(),
        ));

        let csv_body = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Sol Ring,FDN,Foundations,42,foil,mythic,1,101500,11111111-1111-1111-1111-111111111111,5.00,false,false,near_mint,en,EUR";

        let test_user = User::for_testing();
        let result = import_cards(
            AuthenticatedUser(test_user),
            State(app_state),
            Body::from(csv_body),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Cards imported successfully");
    }

    #[tokio::test]
    async fn import_cards_succeeds_with_special_characters_in_card_name() {
        let app_state = AppState::for_testing(std::sync::Arc::new(
            crate::application::use_case::MockStatsUseCase::new(),
        ));

        let csv_body = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,\"Dwynen, Gilt-Leaf Daen\",FDN,Foundations,217,normal,uncommon,2,100086,01c00d7b-7fac-4f8c-a1ea-de2cf4d06627,0.2,false,false,near_mint,fr,EUR";

        let test_user = User::for_testing();
        let result = import_cards(
            AuthenticatedUser(test_user),
            State(app_state),
            Body::from(csv_body),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Cards imported successfully");
    }
}
