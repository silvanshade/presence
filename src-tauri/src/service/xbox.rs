use serde::Deserialize;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    ModelUpdateGui {
        source: crate::app::model::Error,
    },
    Oauth2AuthUrlNew {
        source: url::ParseError,
    },
    Oauth2CsrfTokenStateSecretMismatch {
        state: String,
        csrf_token: oauth2::CsrfToken,
    },
    Oauth2ExchangeCode {
        source: oauth2::basic::BasicRequestTokenError<oauth2::reqwest::Error<reqwest::Error>>,
    },
    Oauth2RedirectUrlNew {
        source: url::ParseError,
    },
    Oauth2TokenUrlNew {
        source: url::ParseError,
    },
    ReqwestRequestGet {
        source: reqwest::Error,
    },
    ReqwestResponseJson {
        source: reqwest::Error,
    },
    ReqwestRequestSend {
        source: reqwest::Error,
    },
    SerdeJsonFromValue {
        source: serde_json::Error,
    },
    SerdeUrlEncoded {
        source: serde::de::value::Error,
    },
    StdSyncMpscReceive {
        source: std::sync::mpsc::RecvError,
    },
    TauriWindowNavigate {
        source: crate::service::Error,
    },
    TauriSpawn {
        source: tauri::Error,
    },
    TauriWindowBuilderNew {
        source: tauri::Error,
    },
    TauriWindowClose {
        source: tauri::Error,
    },
    UrlDropResizeParams,
    UrlParse {
        source: url::ParseError,
    },
    UrlQuery,
    XboxTokenXui,
}

pub mod api {
    pub mod authorize {
        use super::super::{
            Error,
            ModelUpdateGuiSnafu,
            Oauth2AuthUrlNewSnafu,
            Oauth2ExchangeCodeSnafu,
            Oauth2RedirectUrlNewSnafu,
            Oauth2TokenUrlNewSnafu,
            ReqwestRequestSendSnafu,
            ReqwestResponseJsonSnafu,
            SerdeUrlEncodedSnafu,
            StdSyncMpscReceiveSnafu,
            TauriSpawnSnafu,
            TauriWindowBuilderNewSnafu,
            TauriWindowCloseSnafu,
            TauriWindowNavigateSnafu,
            UrlParseSnafu,
            UrlQuerySnafu,
            XboxTokenXuiSnafu,
        };
        use crate::service::TauriWindowExt;
        use serde::Deserialize;
        use snafu::prelude::*;
        use tap::prelude::*;

        const CLIENT_ID: &str = "6d97ccd0-5a71-48c5-9bc3-a203a183da22";

        const REDIRECT_URL: &str = "http://localhost:3000/api/xbox/authorize/redirect";

        const OAUTH2_SCOPES: [&str; 2] = ["xboxlive.signin", "xboxlive.offline_access"];

        const OAUTH2_AUTH_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";

        const OAUTH2_TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";

        const XBOX_USER_AUTH_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";

        const XBOX_XSTS_AUTH_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";

        fn from_xbox_xui_datas<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where
            D: serde::de::Deserializer<'de>,
            T: serde::de::DeserializeOwned,
        {
            use serde::de::Error;
            let datas: Vec<T> = serde::de::Deserialize::deserialize(deserializer)?;
            datas
                .into_iter()
                .next()
                .context(XboxTokenXuiSnafu)
                .map_err(D::Error::custom)
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct AuthCodeQuery {
            code: String,
            state: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct XboxUserToken {
            // display_claims: XboxUserTokenDisplayClaims,
            // #[serde(with = "time::serde::iso8601")]
            // issue_instant: time::OffsetDateTime,
            // #[serde(with = "time::serde::iso8601")]
            // not_after: time::OffsetDateTime,
            token: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct XboxUserTokenDisplayClaims {
            // #[serde(deserialize_with = "from_xbox_xui_datas")]
            // xui: XboxUserTokenXuiData,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct XboxUserTokenXuiData {
            // uhs: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct XboxXstsToken {
            pub display_claims: XboxXstsTokenDisplayClaims,
            // #[serde(with = "time::serde::iso8601")]
            // issue_instant: time::OffsetDateTime,
            // #[serde(with = "time::serde::iso8601")]
            // not_after: time::OffsetDateTime,
            pub token: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct XboxXstsTokenDisplayClaims {
            #[serde(deserialize_with = "from_xbox_xui_datas")]
            pub xui: XboxXstsTokenXuiData,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct XboxXstsTokenXuiData {
            // agg: String,
            pub gtg: String,
            // prv: String,
            pub uhs: String,
            // usr: String,
            // utr: String,
            pub xid: String,
        }

        fn client() -> Result<oauth2::basic::BasicClient, Error> {
            let client_id = CLIENT_ID.conv::<String>().pipe(oauth2::ClientId::new);
            let client_secret = None;
            let auth_url = OAUTH2_AUTH_URL
                .conv::<String>()
                .pipe(oauth2::AuthUrl::new)
                .context(Oauth2AuthUrlNewSnafu)?;
            let token_url = OAUTH2_TOKEN_URL
                .conv::<String>()
                .pipe(oauth2::TokenUrl::new)
                .context(Oauth2TokenUrlNewSnafu)?
                .conv::<Option<_>>();
            let redirect_uri = "http://localhost:3000/api/xbox/authorize/redirect"
                .conv::<String>()
                .pipe(oauth2::RedirectUrl::new)
                .context(Oauth2RedirectUrlNewSnafu)?;
            let client = oauth2::basic::BasicClient::new(client_id, client_secret, auth_url, token_url)
                .set_redirect_uri(redirect_uri);
            Ok(client)
        }

        pub async fn flow(app: &tauri::AppHandle, reauthorize: bool) -> Result<(), Error> {
            use oauth2::TokenResponse;
            use tauri::Manager;
            let client = client()?;
            let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
            let code = flow_get_oauth2_auth_code(app, reauthorize, &client, pkce_code_challenge).await?;
            let bearer_token_response = flow_get_oauth2_bearer_token(&client, code, pkce_code_verifier).await?;
            let xbox_user_token = flow_get_xbox_user_token(bearer_token_response.access_token()).await?;
            let xbox_xsts_token = flow_get_xbox_xsts_token(&xbox_user_token).await?;
            println!("{:#?}", xbox_xsts_token);
            let model = app.state::<crate::app::Model>();
            model
                .update_gui(|gui| {
                    use crate::app::model::gui::service::xbox::Data;
                    let gamertag = xbox_xsts_token.display_claims.xui.gtg.clone();
                    if let Some(data) = &mut gui.services.xbox.data {
                        data.gamertag = gamertag;
                    } else {
                        gui.services.xbox.data = Some(Data { gamertag })
                    }
                })
                .await
                .context(ModelUpdateGuiSnafu)?;
            model.notifiers.gui.notify_waiters();
            *model.session.xbox.write().await = Some(xbox_xsts_token);
            Ok(())
        }

        async fn flow_get_oauth2_auth_code(
            app: &tauri::AppHandle,
            reauthorize: bool,
            client: &oauth2::basic::BasicClient,
            pkce_code_challenge: oauth2::PkceCodeChallenge,
        ) -> Result<oauth2::AuthorizationCode, Error> {
            use tauri::Manager;

            let (auth_url, csrf_token) = {
                let scopes = OAUTH2_SCOPES
                    .into_iter()
                    .map(|scope| scope.conv::<String>().pipe(oauth2::Scope::new));
                client
                    .authorize_url(oauth2::CsrfToken::new_random)
                    .add_scopes(scopes)
                    .set_pkce_challenge(pkce_code_challenge)
                    .url()
            };

            let (tx, rx) = std::sync::mpsc::channel::<String>();

            let window = {
                tauri::WindowBuilder::new(app, "auth-xbox", tauri::WindowUrl::App("/html/auth-init.html".into()))
                    .on_navigation(move |url: String| {
                        if url.starts_with(REDIRECT_URL) {
                            tx.send(url).expect("failed to send redirect URL back from window");
                            return false;
                        }
                        true
                    })
                    .build()
                    .context(TauriWindowBuilderNewSnafu)?
            };
            app.state::<crate::app::Model>()
                .notifiers
                .xbox_auth_ready
                .notified()
                .await;
            window
                .navigate(auth_url, reauthorize)
                .context(TauriWindowNavigateSnafu)?;

            let auth_redirect = rx
                .recv()
                .context(StdSyncMpscReceiveSnafu)?
                .as_str()
                .pipe(url::Url::parse)
                .context(UrlParseSnafu)?;

            tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
                .await
                .context(TauriSpawnSnafu)??;

            let AuthCodeQuery { code, state } = auth_redirect
                .query()
                .context(UrlQuerySnafu)?
                .pipe(serde_urlencoded::from_str::<AuthCodeQuery>)
                .context(SerdeUrlEncodedSnafu)?;

            if &state != csrf_token.secret() {
                return Err(Error::Oauth2CsrfTokenStateSecretMismatch { state, csrf_token });
            }

            Ok(oauth2::AuthorizationCode::new(code))
        }

        async fn flow_get_oauth2_bearer_token(
            client: &oauth2::basic::BasicClient,
            code: oauth2::AuthorizationCode,
            pkce_code_verifier: oauth2::PkceCodeVerifier,
        ) -> Result<oauth2::basic::BasicTokenResponse, Error> {
            let token = client
                .exchange_code(code)
                .set_pkce_verifier(pkce_code_verifier)
                .request_async(oauth2::reqwest::async_http_client)
                .await
                .context(Oauth2ExchangeCodeSnafu)?;
            Ok(token)
        }

        async fn flow_get_xbox_user_token(access_token: &oauth2::AccessToken) -> Result<XboxUserToken, Error> {
            reqwest::Client::new()
                .post(XBOX_USER_AUTH_URL)
                .header("x-xbl-contract-version", "1")
                .json(&serde_json::json!({
                    "RelyingParty": "http://auth.xboxlive.com",
                    "TokenType": "JWT",
                    "Properties": {
                        "AuthMethod": "RPS",
                        "SiteName": "user.auth.xboxlive.com",
                        "RpsTicket": format!("d={}", access_token.secret()),
                    },
                }))
                .send()
                .await
                .context(ReqwestRequestSendSnafu)?
                .json::<XboxUserToken>()
                .await
                .context(ReqwestResponseJsonSnafu)
        }

        async fn flow_get_xbox_xsts_token(xbox_user_token: &XboxUserToken) -> Result<XboxXstsToken, Error> {
            reqwest::Client::new()
                .post(XBOX_XSTS_AUTH_URL)
                .header("x-xbl-contract-version", "1")
                .json(&serde_json::json!({
                    "RelyingParty": "http://xboxlive.com",
                    "TokenType": "JWT",
                    "Properties": {
                        "SandboxId": "RETAIL",
                        "UserTokens": [
                            xbox_user_token.token,
                        ],
                    },
                }))
                .send()
                .await
                .context(ReqwestRequestSendSnafu)?
                .json::<XboxXstsToken>()
                .await
                .context(ReqwestResponseJsonSnafu)
        }
    }
    pub mod autosuggest {
        use super::super::{
            Error,
            ReqwestRequestGetSnafu,
            ReqwestResponseJsonSnafu,
            SerdeJsonFromValueSnafu,
            UrlDropResizeParamsSnafu,
            UrlParseSnafu,
        };
        use serde::Deserialize;
        use snafu::prelude::*;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct XboxStoreAutoSuggest {
            pub result_sets: Vec<XboxStoreAutoSuggestResultSet>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct XboxStoreAutoSuggestResultSet {
            pub suggests: Vec<XboxStoreSuggestResult>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct XboxStoreSuggestResult {
            pub source: String,
            pub title: String,
            pub url: String,
            pub image_url: String,
        }

        impl XboxStoreSuggestResult {
            pub fn image_url(&self) -> Result<url::Url, Error> {
                let protocol = "https";
                let url = self.image_url.split('?').next().context(UrlDropResizeParamsSnafu)?;
                url::Url::parse(&format!("{}:{}", protocol, url)).context(UrlParseSnafu)
            }

            pub fn store_url(&self) -> Result<url::Url, Error> {
                let protocol = "https";
                let url = &self.url;
                url::Url::parse(&format!("{}:{}", protocol, url)).context(UrlParseSnafu)
            }
        }

        const ENDPOINT_AUTOSUGGEST: &str = "https://www.microsoft.com/msstoreapiprod/api/autosuggest";

        fn endpoint(query: &str) -> Result<url::Url, Error> {
            use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
            let encoded_query = utf8_percent_encode(query, NON_ALPHANUMERIC).to_string();
            let params = [
                ("market", "en-us"),
                ("sources", "DCatAll-Products"),
                ("query", encoded_query.as_str()),
            ];
            url::Url::parse_with_params(ENDPOINT_AUTOSUGGEST, params).context(UrlParseSnafu)
        }

        pub async fn request(query: &str) -> Result<Option<XboxStoreSuggestResult>, Error> {
            use triple_accel::levenshtein_exp;
            let url = endpoint(query)?;
            let data = reqwest::get(url).await.context(ReqwestRequestGetSnafu)?;
            println!("{:#?}", data);
            let json = data
                .json::<serde_json::Value>()
                .await
                .context(ReqwestResponseJsonSnafu)?;
            println!("{:#?}", json);
            let auto_suggest = serde_json::from_value::<XboxStoreAutoSuggest>(json).context(SerdeJsonFromValueSnafu)?;
            let mut results = auto_suggest
                .result_sets
                .into_iter()
                .flat_map(|result_set| result_set.suggests)
                .filter(|suggest| suggest.source == "Game")
                .map(|suggest| {
                    let query = query.as_bytes();
                    let title = suggest.title.as_bytes();
                    (levenshtein_exp(query, title), suggest)
                })
                .collect::<Vec<_>>();
            results.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
            Ok(results.into_iter().map(|suggest| suggest.1).next())
        }
    }

    use self::autosuggest::XboxStoreSuggestResult;
    use super::Error;

    pub async fn authorize(app: &tauri::AppHandle, reauthorize: bool) -> Result<(), Error> {
        self::authorize::flow(app, reauthorize).await
    }

    pub async fn autosuggest(query: &str) -> Result<Option<XboxStoreSuggestResult>, Error> {
        self::autosuggest::request(query).await
    }
}
