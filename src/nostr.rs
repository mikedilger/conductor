use nostr::event::id::EventId;
use nostr::event::Event;
use nostr::filter::Filter;
use nostr::key::public_key::PublicKey;
use nostr::nips::nip07::BrowserSigner;
use nostr_sdk::Client as NostrClient;
use serde_json::Value;
use std::time::Duration;

pub fn id_list_to_filter(arr: Vec<Value>) -> Filter {
    let mut filter: Filter = Default::default();
    for elem in arr.iter() {
        if let Some(map) = elem.as_object() {
            if let Some(val) = map.get("id") {
                if let Some(idstr) = val.as_str() {
                    if let Ok(id) = EventId::parse(idstr) {
                        filter = filter.id(id);
                    }
                }
            }
        }
    }
    filter
}

pub fn pubkey_list_to_vec(arr: Vec<Value>) -> Vec<PublicKey> {
    let mut output: Vec<PublicKey> = Vec::new();
    for elem in arr.iter() {
        if let Some(map) = elem.as_object() {
            if let Some(val) = map.get("pubkey") {
                if let Some(pkstr) = val.as_str() {
                    if let Ok(pk) = PublicKey::parse(pkstr) {
                        output.push(pk)
                    }
                }
            }
        }
    }
    output
}

pub async fn get_events(
    url: &str,
    filter: Filter,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    if filter.is_empty() {
        return Ok(vec![]);
    }

    let client = NostrClient::default();
    client.set_signer(BrowserSigner::new()?).await;
    client.add_relay(url).await?;
    client.connect().await;
    let events = client
        .fetch_events(filter, Duration::from_secs(5))
        .await?
        .to_vec();

    Ok(events)
}
