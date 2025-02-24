use dioxus::prelude::*;

#[component]
pub fn Help() -> Element {
    rsx! {
        h2 {
            "Getting Started"
        }
        p {
            "On the SETUP page, enter the relay URL you wish to manage. Once you have that, and you have a NIP-07 nostr signer setup, you are good to go."
        }

        hr {
        }

        h2 {
            "Use with Chorus"
        }
        p {
            "Chorus has a particular way of categorizing events and pubkeys that may not match other relays. This relay was designed primarily for chorus, but aspires to work with other relays too via NIP-86."
        }

        blockquote {
            h3 {
                class: "underline",
                "How Chorus categorizes Public Keys"
            }

            h4 {
                "Administrators"
            }
            p {
                "Administrators are defined in the config file and not editable via the management interface. Only administrators may manage the roles applied to pubkeys. They can do that through this website."
            }

            h4 {
                "Roles"
            }

            ul {
                li {
                    span {
                        class: "term",
                        "moderator:"
                    }
                    "Moderators can moderate events. They can also ban and unban pubkeys (except pubkeys with a role assigned). Moderators are always also authorized users."
                }
                li {
                    span {
                        class: "term",
                        "authorized user:"
                    }
                    "These users are the ones the relay is serving. Their events are immediately publicly visible, and they can see events in the moderation quere that the public cannot see. But they cannot moderate these events."
                }
            }

            h4 {
                "Moderation States"
            }

            p {
                "These aren't the same as the roles. They apply to all pubkeys that do not have a role."
            }
            ul {
                li {
                    span {
                        class: "term",
                        "allowed pubkey:"
                    }
                    "This pubkey is allowed to post, and their posts are immediately visible (they bypass the moderation queue).  But they are not an authorized user, so they can only post in reply to an authorized user. Moderators can put any pubkey into this state."
                }
                li {
                    span {
                        class: "term",
                        "banned pubkey:"
                    }
                    "This pubkey is not allowed to post, or else their events are insta-banned (I forget which). Moderators can put any pubkey into this state."
                }
                li {
                    span {
                        class: "term",
                        "unlisted:"
                    }
                    "Unlisted users are treated as normal. They can reply to authorized users, but their replies land in the moderation queue and are not publicly visible until a moderator approves them."
                }
            }

            h3 {
                class: "underline",
                "How Chorus categorizes Events"
            }

            ul {
                li {
                    span {
                        class: "term",
                        "allowed event:"
                    }
                    "These events are either allowed because they are authored by an authorized user, or else they were approved in moderation."
                }
                li {
                    span {
                        class: "term",
                        "banned event:"
                    }
                    "These events were not authored by an authorized user, and they were banned in moderation. They still might be present on the relay but are not publicly visible."
                }
                li {
                    span {
                        class: "term",
                        "unlisted (queued):"
                    }
                    "These events were not authored by an authorized user, and they are awaiting moderation."
                }
            }
        }
    }
}
