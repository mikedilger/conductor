use lazy_static::lazy_static;
use dashmap::DashMap;
use nostr::event::id::EventId;
use nostr::event::{Event, Kind};
use nostr::filter::Filter;
use nostr::key::public_key::PublicKey;
use nostr::nips::nip01::Metadata;
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

    // Create client in CLIENTS if missing
    if ! CLIENTS.contains_key(url) {
        let client = NostrClient::default();
        client.set_signer(BrowserSigner::new()?).await;
        client.add_relay(url).await?;
        CLIENTS.insert(url.to_owned(), client);
    }

    let client = CLIENTS.get(url).unwrap();
    client.connect().await;
    let events = client
        .fetch_events(filter, Duration::from_secs(5))
        .await?
        .to_vec();

    Ok(events)
}

lazy_static! {
    static ref METADATA: DashMap<PublicKey, Option<Metadata>> = {
        DashMap::new()
    };

    static ref CLIENTS: DashMap<String, NostrClient> = {
        DashMap::new()
    };
}


pub async fn get_metadata(pubkey: PublicKey, discovery_relay_url: String) ->
    Result<Option<Metadata>, Box<dyn std::error::Error>>
{
    if let Some(optmd) = METADATA.get(&pubkey) {
        return Ok(optmd.clone());
    }

    let filter: Filter = Filter::default()
        .author(pubkey)
        .kind(Kind::RelayList);
    let events = get_events(&*discovery_relay_url, filter).await?;
    if events.is_empty() {
        METADATA.insert(pubkey, None);
        return Ok(None);
    }

    // collect 'r' tags that have no marker or that have a 'read' marker.
    let mut relays: Vec<String> = vec![];
    for tag in events[0].tags.iter().cloned() {
        let fields = tag.to_vec();
        if fields.get(0) == Some(&"r".to_string()) {
            if let Some(url) = fields.get(1) {
                if fields.get(2) == None || fields.get(2) == Some(&"read".to_owned()) {
                    relays.push(url.to_owned());
                }
            }
        }
    }
    if relays.is_empty() {
        METADATA.insert(pubkey, None);
        return Ok(None);
    }

    // get metadata
    let filter: Filter = Filter::default()
        .author(pubkey)
        .kind(Kind::Metadata);
    let events = get_events(&*relays[0], filter).await?;
    if events.is_empty() {
        METADATA.insert(pubkey, None);
        return Ok(None);
    }

    let metadata: Metadata = serde_json::from_str(&events[0].content)?;

    METADATA.insert(pubkey, Some(metadata.clone()));

    Ok(Some(metadata))
}
