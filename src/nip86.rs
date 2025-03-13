use base64::Engine;
use dioxus::logger::tracing::info;
use http::uri::{Scheme, Uri};
use nostr::event::id::EventId;
use nostr::event::{Event, Kind, Tag, TagStandard, Tags, UnsignedEvent};
use nostr::key::public_key::PublicKey;
use nostr::nips::nip07::BrowserSigner;
use nostr::nips::nip98::HttpMethod;
use nostr::signer::NostrSigner;
use nostr::types::time::Timestamp;
use nostr::util::{hex, JsonUtil};
use reqwest::Client as HttpClient;
use secp256k1::hashes::{sha256, Hash};
use serde::Deserialize;
use serde_json::{json, Map, Value};

async fn auth_header(uri: &Uri, payload: &str) -> Result<String, Box<dyn std::error::Error>> {
    let payload_hash = sha256::Hash::hash(payload.as_bytes());
    let payload_hash_bytes = <sha256::Hash as AsRef<[u8]>>::as_ref(&payload_hash);
    let payload_hash_string = hex::encode(payload_hash_bytes);
    let event = auth_event(uri, &payload_hash_string).await?;
    let base64 = base64::engine::general_purpose::STANDARD.encode(event.as_json());
    Ok(format!("Nostr {}", base64))
}

async fn auth_event(uri: &Uri, payload_hash: &str) -> Result<Event, Box<dyn std::error::Error>> {
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

#[derive(Debug, Clone, Deserialize)]
pub struct Nip86Response {
    #[serde(default)]
    pub error: Option<String>,
    pub result: Value,
}

async fn post(uri: &Uri, body: String) -> Result<Nip86Response, Box<dyn std::error::Error>> {
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

    let client = HttpClient::builder().build()?;
    let http_response = client
        .post(format!("{}", uri))
        .header("Host", host)
        .header("Content-Type", "application/nostr+json+rpc")
        .header("Authorization", auth)
        .body(body)
        .send()
        .await?;

    let status = http_response.status().as_u16();
    let http_response_text = http_response.text().await?;
    if status != 200 {
        return Err(Box::new(std::io::Error::other(format!(
            "Server responded with {}",
            status
        ))));
    }

    let nip86_response: Nip86Response = serde_json::from_str(&http_response_text)?;
    Ok(nip86_response)
}

pub async fn run_command_on_relay(
    url: &str,
    method: &str,
    params: Value,
) -> Result<Nip86Response, Box<dyn std::error::Error>> {
    let cmd = json!({
        "method": method,
        "params": params
    });
    let cmdstr = serde_json::to_string(&cmd)?;
    let uri = url.parse::<Uri>()?;
    let nip86response = post(&uri, cmdstr).await?;
    Ok(nip86response)
}

pub async fn stats(url: &str) -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "stats", json!([])).await?;

    let err = |s| -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
        Err(Box::new(std::io::Error::other(s)))
    };

    if let Some(err) = response.error {
        Err(Box::new(std::io::Error::other(err)))
    } else if let Value::Object(m) = response.result {
        Ok(m)
    } else {
        err("Result was not an object.")
    }
}

pub async fn mod_queue(
    url: &str,
    _reload_trick: usize,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "listeventsneedingmoderation", json!([])).await?;

    let err = |s| -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        Err(Box::new(std::io::Error::other(s)))
    };

    if let Some(err) = response.error {
        return Err(Box::new(std::io::Error::other(err)));
    }

    let Value::Array(arr) = response.result else {
        return err("Result was not an array");
    };

    let filter = crate::nostr::id_list_to_filter(arr);
    crate::nostr::fetch_events(url, filter).await
}

pub async fn allow_event(url: &str, id: EventId) -> Result<(), Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "allowevent", json!([id, "unspecified",])).await?;
    info!("{response:?}");

    Ok(())
}

pub async fn ban_event(url: &str, id: EventId) -> Result<(), Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "banevent", json!([id, "unspecified",])).await?;
    info!("{response:?}");

    Ok(())
}

pub async fn clear_event(url: &str, id: EventId) -> Result<(), Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "clearevent", json!([id,])).await?;
    info!("{response:?}");

    Ok(())
}

pub async fn remove_event(url: &str, id: EventId) -> Result<(), Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "removeevent", json!([id,])).await?;
    info!("{response:?}");

    Ok(())
}

pub async fn allow_pubkey(url: &str, pubkey: PublicKey) -> Result<(), Box<dyn std::error::Error>> {
    let response =
        run_command_on_relay(url, "allowpubkey", json!([pubkey, "unspecified",])).await?;
    info!("{response:?}");

    Ok(())
}

pub async fn ban_pubkey(url: &str, pubkey: PublicKey) -> Result<(), Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "banpubkey", json!([pubkey, "unspecified",])).await?;
    info!("{response:?}");

    Ok(())
}

pub async fn clear_pubkey(url: &str, pubkey: PublicKey) -> Result<(), Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "clearpubkey", json!([pubkey,])).await?;
    info!("{response:?}");

    Ok(())
}

pub async fn listallowedevents(
    url: &str,
    _reload_trick: usize,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "listallowedevents", json!([])).await?;

    let err = |s| -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        Err(Box::new(std::io::Error::other(s)))
    };

    if let Some(err) = response.error {
        return Err(Box::new(std::io::Error::other(err)));
    }

    let Value::Array(arr) = response.result else {
        return err("Result was not an array");
    };

    info!("Loaded {} allowed event IDs", arr.len());

    let filter = crate::nostr::id_list_to_filter(arr);
    let events = crate::nostr::fetch_events(url, filter).await?;

    info!("Loaded {} allowed events", events.len());

    Ok(events)
}

pub async fn listbannedevents(
    url: &str,
    _reload_trick: usize,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "listbannedevents2", json!([])).await?;

    let err = |s| -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        Err(Box::new(std::io::Error::other(s)))
    };

    if let Some(err) = response.error {
        return Err(Box::new(std::io::Error::other(err)));
    }

    let Value::Array(arr) = response.result else {
        return err("Result was not an array");
    };

    info!("Loaded {} banned events", arr.len());

    let mut events: Vec<Event> = Vec::new();
    for elem in arr.iter() {
        if let Some(map) = elem.as_object() {
            if let Some(val) = map.get("event") {
                if let Some(s) = val.as_str() {
                    let event = nostr::Event::from_json(s)?;
                    events.push(event);
                }
            }
        }
    }

    Ok(events)
}

pub async fn listallowedpubkeys(
    url: &str,
    _reload_trick: usize,
) -> Result<Vec<PublicKey>, Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "listallowedpubkeys", json!([])).await?;

    let err = |s| -> Result<Vec<PublicKey>, Box<dyn std::error::Error>> {
        Err(Box::new(std::io::Error::other(s)))
    };

    if let Some(err) = response.error {
        return Err(Box::new(std::io::Error::other(err)));
    }

    let Value::Array(arr) = response.result else {
        return err("Result was not an array");
    };

    info!("Loaded allowed pubkeys");

    Ok(crate::nostr::pubkey_list_to_vec(arr))
}

pub async fn listbannedpubkeys(
    url: &str,
    _reload_trick: usize,
) -> Result<Vec<PublicKey>, Box<dyn std::error::Error>> {
    let response = run_command_on_relay(url, "listbannedpubkeys", json!([])).await?;

    let err = |s| -> Result<Vec<PublicKey>, Box<dyn std::error::Error>> {
        Err(Box::new(std::io::Error::other(s)))
    };

    if let Some(err) = response.error {
        return Err(Box::new(std::io::Error::other(err)));
    }

    let Value::Array(arr) = response.result else {
        return err("Result was not an array");
    };

    info!("Loaded banned pubkeys");

    Ok(crate::nostr::pubkey_list_to_vec(arr))
}
