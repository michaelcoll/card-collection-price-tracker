use super::card_controller::{
    CollectionCardResponse, MessageResponse, PaginatedCollectionResponse, PriceGuideResponse,
    SortByParam, SortDirParam,
};
use super::maintenance_controller::{EnqueueResponse, StatsResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::card_controller::get_collection,
        super::card_controller::import_cards,
        super::card_controller::get_card_info,
        super::maintenance_controller::get_stats,
        super::maintenance_controller::trigger_price_update,
        super::maintenance_controller::update_cardmarket_ids,
    ),
    components(schemas(
        PriceGuideResponse,
        CollectionCardResponse,
        MessageResponse,
        PaginatedCollectionResponse,
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
