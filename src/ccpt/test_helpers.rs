pub mod fixtures {
    use crate::domain::card::{Card, CardId};
    use crate::domain::language_code::LanguageCode;
    use crate::domain::rarity_code::RarityCode;
    use crate::domain::set_name::SetCode;

    pub fn single_rsa_jwks() -> &'static str {
        r#"{"keys":[{"kty":"RSA","kid":"test-key","n":"test-n-value","e":"AQAB","alg":"RS256","use":"sig"}]}"#
    }

    pub fn empty_jwks() -> &'static str {
        r#"{"keys":[]}"#
    }

    pub fn multi_key_jwks() -> &'static str {
        r#"{"keys":[{"kty":"RSA","kid":"key-1","n":"test-n-1","e":"AQAB","alg":"RS256","use":"sig"},{"kty":"RSA","kid":"key-2","n":"test-n-2","e":"AQAB","alg":"RS256","use":"sig"}]}"#
    }

    pub fn make_card_id(n: &str) -> CardId {
        CardId::new(SetCode::new("FDN"), n, LanguageCode::FR, false)
    }

    pub fn make_card(set_code: &str, collector_number: &str) -> Card {
        Card::new(
            set_code,
            format!("Set {set_code}"),
            collector_number,
            LanguageCode::EN,
            false,
            "Test Card",
            RarityCode::C,
            1,
            100,
        )
    }
}

pub mod http {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    pub async fn mock_get(server: &MockServer, url_path: &str, body: &str) {
        Mock::given(method("GET"))
            .and(path(url_path))
            .respond_with(ResponseTemplate::new(200).set_body_string(body.to_owned()))
            .mount(server)
            .await;
    }

    pub async fn jwks_server(jwks: &str) -> MockServer {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(jwks.to_owned()))
            .mount(&server)
            .await;
        server
    }
}
