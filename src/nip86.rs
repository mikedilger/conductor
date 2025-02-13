use nostr::event::{Event, Kind, Tags, Tag, TagStandard, UnsignedEvent};
use nostr::nips::nip07::BrowserSigner;
use nostr::nips::nip98::HttpMethod;
use nostr::signer::NostrSigner;
use nostr::types::time::Timestamp;
use nostr::types::url::RelayUrl;

// Create auth event

//
// content-type: application/nostr+json+rpc
// authorization: (nip-98)
//    payload tag is required, u tag is the relay url


pub async fn auth(relay_url: RelayUrl, method: HttpMethod, payload: &str)
                  -> Result<Event, Box<dyn std::error::Error>>
{
    let signer = BrowserSigner::new()?;

    let tags = Tags::new(vec![
        TagStandard::Url(relay_url.try_into().unwrap()).into(),
        TagStandard::Method(method).into(),
        Tag::parse(["payload", payload])?,
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
