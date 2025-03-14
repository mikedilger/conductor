use crate::fetch_job::FetchJob;
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

lazy_static! {
    static ref CLIENTS: DashMap<String, NostrClient> = {
        DashMap::new()
    };

    static ref METADATA: DashMap<PublicKey, FetchJob<Option<Metadata>>> = {
        DashMap::new()
    };

}

pub async fn get_metadata(pubkey: PublicKey, discovery_relay_url: String) ->
    Result<Option<Metadata>, Box<dyn std::error::Error>>
{
    {
        match METADATA.get(&pubkey) {
            Some(fj) => return Ok(fj.get().await),
            None => { }
        }
    }

    let fj = FetchJob::new();
    METADATA.insert(pubkey, fj.clone());
    fetch_metadata(pubkey, discovery_relay_url, fj).await?;
    match METADATA.get(&pubkey) {
        Some(fj) => Ok(fj.get().await),
        None => panic!("Impossible"),
    }
}

pub async fn fetch_metadata(
    pubkey: PublicKey,
    discovery_relay_url: String,
    fj: FetchJob<Option<Metadata>>
) -> Result<(), Box<dyn std::error::Error>> {
    let filter: Filter = Filter::default()
        .author(pubkey)
        .kind(Kind::RelayList);
    let events = fetch_events(&discovery_relay_url, filter).await?;
    if events.is_empty() {
        fj.complete(None);
        return Ok(());
    }

    // collect 'r' tags that have no marker or that have a 'read' marker.
    let mut relays: Vec<String> = vec![];
    for tag in events[0].tags.iter().cloned() {
        let fields = tag.to_vec();
        if fields.first() == Some(&"r".to_string()) {
            if let Some(url) = fields.get(1) {
                if fields.get(2).is_none() || fields.get(2) == Some(&"read".to_owned()) {
                    relays.push(url.to_owned());
                }
            }
        }
    }
    if relays.is_empty() {
        fj.complete(None);
        return Ok(());
    }

    // get metadata
    let filter: Filter = Filter::default()
        .author(pubkey)
        .kind(Kind::Metadata);
    let events = fetch_events(&relays[0], filter).await?;
    if events.is_empty() {
        fj.complete(None);
        return Ok(());
    }

    let metadata: Metadata = serde_json::from_str(&events[0].content)?;
    fj.complete(Some(metadata));
    Ok(())
}

pub async fn fetch_events(
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
