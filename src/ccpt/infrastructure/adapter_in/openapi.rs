use super::card::dto::{MessageResponse, PriceHistoryEntryResponse};
use super::collection::dto::{
    CollectionCardResponse, PaginatedCollectionResponse, PriceGuideResponse, SortByParam,
    SortDirParam,
};
use super::maintenance::dto::{EnqueueResponse, StatsResponse};
use super::trade::dto::CreateTradeRequest;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::collection::controller::get_collection,
        super::card::controller::import_cards,
        super::card::controller::get_card_info,
        super::card::controller::get_card_price_history,
        super::maintenance::controller::get_stats,
        super::maintenance::controller::trigger_price_update,
        super::maintenance::controller::update_cardmarket_ids,
        super::user::controller::register,
        super::trade::controller::create_trade,
    ),
    components(schemas(
        PriceGuideResponse,
        CollectionCardResponse,
        MessageResponse,
        PaginatedCollectionResponse,
        PriceHistoryEntryResponse,
        SortByParam,
        SortDirParam,
        StatsResponse,
        EnqueueResponse,
        CreateTradeRequest,
    )),
    modifiers(&SecurityAddon),
    info(
        title = "Card Collection Price Tracker API",
        version = "0.1.0",
        description = "REST API for tracking Magic: The Gathering card prices",
        license(name = "MIT", url = "https://opensource.org/licenses/MIT")
    ),
    tags(
        (name = "cards", description = "Card catalog, import and price history (authentication required)"),
        (name = "collection", description = "Player's collection (authentication required)"),
        (name = "maintenance", description = "Maintenance operations (public)"),
        (name = "auth", description = "Authentication and user registration (authentication required)"),
        (name = "trades", description = "Trade requests between two collectors (authentication required)"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
