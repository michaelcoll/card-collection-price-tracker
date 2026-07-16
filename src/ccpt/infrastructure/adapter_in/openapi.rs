use super::card::dto::{
    CollectionCardResponse, MessageResponse, PaginatedCollectionResponse, PriceGuideResponse,
    PriceHistoryEntryResponse, SortByParam, SortDirParam,
};
use super::maintenance::dto::{EnqueueResponse, StatsResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::card::controller::get_collection,
        super::card::controller::import_cards,
        super::card::controller::get_card_info,
        super::card::controller::get_card_price_history,
        super::maintenance::controller::get_stats,
        super::maintenance::controller::trigger_price_update,
        super::maintenance::controller::update_cardmarket_ids,
        super::user::controller::register,
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
    )),
    modifiers(&SecurityAddon),
    info(
        title = "Card Collection Price Tracker API",
        version = "0.1.0",
        description = "REST API for tracking Magic: The Gathering card prices",
        license(name = "MIT", url = "https://opensource.org/licenses/MIT")
    ),
    tags(
        (name = "cards", description = "Collection management (authentication required)"),
        (name = "maintenance", description = "Maintenance operations (public)"),
        (name = "auth", description = "Authentication and user registration (authentication required)"),
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
