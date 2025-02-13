use base64::Engine;
use http::uri::{Scheme, Uri};
use nostr::event::{Event, Kind, Tags, Tag, TagStandard, UnsignedEvent};
use nostr::nips::nip07::BrowserSigner;
use nostr::nips::nip98::HttpMethod;
use nostr::signer::NostrSigner;
use nostr::types::time::Timestamp;
use nostr::util::{hex, JsonUtil};
use reqwest::Client;
use secp256k1::hashes::{sha256, Hash};
use serde_json::{json, Value, Map};

async fn auth_header(uri: &Uri, payload: &str)
                         -> Result<String, Box<dyn std::error::Error>>
{
    let payload_hash = sha256::Hash::hash(payload.as_bytes());
    let payload_hash_bytes = <sha256::Hash as AsRef<[u8]>>::as_ref(&payload_hash);
    let payload_hash_string = hex::encode(payload_hash_bytes);
    let event = auth_event(uri, &payload_hash_string).await?;
    let base64 = base64::engine::general_purpose::STANDARD.encode(event.as_json());
    Ok(format!("Nostr {}", base64))
}

async fn auth_event(uri: &Uri, payload_hash: &str)
                  -> Result<Event, Box<dyn std::error::Error>>
{
    let signer = BrowserSigner::new()?;

    let tags = Tags::new(vec![
        Tag::parse(["u", &format!("{}", uri)])?,
        Tag::parse(["payload", payload_hash])?,
        TagStandard::Method(HttpMethod::POST).into(),
    ]);

    let unsigned = UnsignedEvent {
        id: None,
        pubkey: signer.get_public_key().await?,
        created_at: Timestamp::now(),
        kind: Kind::HttpAuth,
        tags,
        content: "".to_owned(),
    };

    let event = signer.sign_event(unsigned).await?;

    Ok(event)
}

async fn post(uri: &Uri, body: String) -> Result<Value, Box<dyn std::error::Error>> {
    let host = {
        let authority = uri.authority().expect("Has no hostname").as_str();
        authority
            .find('@')
            .map(|idx| authority.split_at(idx + 1).1)
            .unwrap_or_else(|| authority)
            .to_owned()
    };
    let mut parts = uri.clone().into_parts();
    if host.is_empty() {
        panic!("URL has empty hostname");
    }
    parts.scheme = match parts
        .scheme
        .ok_or(std::io::Error::other("Missing scheme"))?
        .as_str()
    {
        "wss" => Some(Scheme::HTTPS),
        "ws" => Some(Scheme::HTTP),
        "https" => Some(Scheme::HTTPS),
        "http" => Some(Scheme::HTTP),
        _ => panic!("We don't support that scheme."),
    };
    let uri = Uri::from_parts(parts)?;

    let auth = auth_header(&uri, &body).await?;

    let client = Client::builder()
        .build()?;
    let response = client
        .post(format!("{}", uri))
        .header("Host", host)
        .header("Content-Type", "application/nostr+json+rpc")
        .header("Authorization", auth)
        .body(body)
        .send().await?;

    let status = response.status().as_u16();
    let response_text = response.text().await?;
    if status != 200 {
        return Err(Box::new(std::io::Error::other(format!("Server responded with {}", status))));
    }

    let value: Value = serde_json::from_str(&response_text)?;
    Ok(value)
}

pub async fn run_command_on_relay(
    url: &str,
    method: &str,
    params: Value
) -> Result<Value, Box<dyn std::error::Error>> {
    let cmd = json!({
        "method": method,
        "params": params
    });
    let cmdstr = serde_json::to_string(&cmd)?;
    let uri = url.parse::<Uri>()?;
    let value = post(&uri, cmdstr).await?;
    Ok(value)
}

pub async fn stats(
    url: &str,
    method: &str,
    params: Value
) -> Map<String, Value> {
    let mut rval = Map::<String, Value>::new();
    let _ = rval.insert("error".to_string(), Value::Null);
    let _ = rval.insert("result".to_string(), Value::Null);

    let value = match run_command_on_relay(url, method, params).await {
        Ok(v) => v,
        Err(e) => {
            let _ = rval.insert("client_error".to_string(), Value::String(e.to_string()));
            return rval;
        }
    };

    let _ = match value {
        Value::Null => { },
        Value::Bool(_) => {
            rval.insert("result".to_owned(), value);
        },
        Value::Number(_) => {
            rval.insert("result".to_owned(), value);
        },
        Value::String(_) => {
            rval.insert("result".to_owned(), value);
        },
        Value::Array(_) => {
            rval.insert("result".to_owned(), value);
        },
        Value::Object(m) => rval = m,
    };

    rval
}
