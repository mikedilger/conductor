TODO
====

* When we load metadata, we spin up a new client and new connection for each person.
  For long lists of events, this causes 429 too many events.  We need to somehow
  share and reuse the client and connections.
