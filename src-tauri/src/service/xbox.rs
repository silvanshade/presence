use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
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
    ReqwestRequestJson {
        source: reqwest::Error,
    },
    ReqwestRequestSend {
        source: reqwest::Error,
    },
    SerdeUrlEncoded {
        source: serde::de::value::Error,
    },
    StdSyncMpscReceive {
        source: std::sync::mpsc::RecvError,
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
    TauriWithWebview {
        source: tauri::Error,
    },
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
            Oauth2AuthUrlNewSnafu,
            Oauth2ExchangeCodeSnafu,
            Oauth2RedirectUrlNewSnafu,
            Oauth2TokenUrlNewSnafu,
            ReqwestRequestJsonSnafu,
            ReqwestRequestSendSnafu,
            SerdeUrlEncodedSnafu,
            StdSyncMpscReceiveSnafu,
            TauriSpawnSnafu,
            TauriWindowBuilderNewSnafu,
            TauriWindowCloseSnafu,
            TauriWithWebviewSnafu,
            UrlParseSnafu,
            UrlQuerySnafu,
            XboxTokenXuiSnafu,
        };
        use crate::service::PlatformWebviewExt;
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

        struct AuthCodeData {
            code: oauth2::AuthorizationCode,
            state: oauth2::CsrfToken,
        }

        impl AuthCodeData {
            pub fn new(code: String, state: String) -> Self {
                let code = oauth2::AuthorizationCode::new(code);
                let state = oauth2::CsrfToken::new(state);
                Self { code, state }
            }
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
            display_claims: XboxUserTokenDisplayClaims,
            #[serde(with = "time::serde::iso8601")]
            issue_instant: time::OffsetDateTime,
            #[serde(with = "time::serde::iso8601")]
            not_after: time::OffsetDateTime,
            token: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct XboxUserTokenDisplayClaims {
            #[serde(deserialize_with = "from_xbox_xui_datas")]
            xui: XboxUserTokenXuiData,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct XboxUserTokenXuiData {
            uhs: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct XboxXstsToken {
            display_claims: XboxXstsTokenDisplayClaims,
            #[serde(with = "time::serde::iso8601")]
            issue_instant: time::OffsetDateTime,
            #[serde(with = "time::serde::iso8601")]
            not_after: time::OffsetDateTime,
            token: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct XboxXstsTokenDisplayClaims {
            #[serde(deserialize_with = "from_xbox_xui_datas")]
            xui: XboxXstsTokenXuiData,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct XboxXstsTokenXuiData {
            agg: String,
            gtg: String,
            prv: String,
            uhs: String,
            usr: String,
            utr: String,
            xid: String,
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

        pub async fn flow(app: &tauri::AppHandle<tauri::Wry>, reauthorize: bool) -> Result<(), Error> {
            use oauth2::TokenResponse;
            use tokio::io::AsyncWriteExt;

            let client = client()?;
            let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
            let AuthCodeData { code, state } =
                flow_get_oauth2_auth_code(app, reauthorize, &client, pkce_code_challenge).await?;
            let bearer_token_response = flow_get_oauth2_bearer_token(&client, code, pkce_code_verifier).await?;

            let xbox_user_token = flow_get_xbox_user_token(bearer_token_response.access_token()).await?;

            let mut file = tokio::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(r"C:\Users\silvanshade\Desktop\user.txt")
                .await
                .unwrap();
            {
                let bytes = format!("{:#?}", xbox_user_token);
                let bytes = bytes.as_bytes();
                file.write_all(bytes).await.unwrap();
            }

            let xbox_xsts_token = flow_get_xbox_xsts_token(&xbox_user_token).await?;

            let mut file = tokio::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(r"C:\Users\silvanshade\Desktop\xsts.txt")
                .await
                .unwrap();
            {
                let bytes = format!("{:#?}", xbox_xsts_token);
                let bytes = bytes.as_bytes();
                file.write_all(bytes).await.unwrap();
            }

            Ok(())
        }

        async fn flow_get_oauth2_auth_code(
            app: &tauri::AppHandle<tauri::Wry>,
            reauthorize: bool,
            client: &oauth2::basic::BasicClient,
            pkce_code_challenge: oauth2::PkceCodeChallenge,
        ) -> Result<AuthCodeData, Error> {
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
                tauri::WindowBuilder::new(
                    app,
                    "twitch-integration-authorization",
                    tauri::WindowUrl::App("/html/empty".into()),
                )
                .on_navigation(move |url: String| {
                    println!("navigating: {}", url);
                    if url.starts_with(REDIRECT_URL) {
                        tx.send(url).expect("failed to send redirect URL back from window");
                        return false;
                    }
                    true
                })
                .build()
                .context(TauriWindowBuilderNewSnafu)?
            };
            window
                .with_webview({
                    let reauthorize = true;
                    move |webview| webview.navigate(auth_url, reauthorize).unwrap()
                })
                .context(TauriWithWebviewSnafu)?;

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

            Ok(AuthCodeData::new(code, state))
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
                .context(ReqwestRequestJsonSnafu)
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
                .context(ReqwestRequestJsonSnafu)
        }
    }
}
