# Franzi, a pure-rust [Kafka](https://kafka.apache.org) protocol implementation

## Goals

* Pure rust Kafka client
* Support for all Kafka protocol versions
* Competitive performance; make this the fastest Kafka client if possible
* Full API Coverage
* Blocking and Non-blocking APIs (using futures/tokio)
* Zero unsafe code (except in "well-known" dependencies, such as std, bytes, tokio, etc.)

## Current status

* In development, not usable
* Non-blocking futures/tokio-based API only

## Progress

* (x) Implement derive(FromKafkaBytes, ToKafkaBytes)
* (x) Implement kafka_message!()
  * ( ) Allow multiple fields with same name but different types (in different (sub)structs)
* ( ) Implement varint, Record, MessageSet
* ( ) Implement all messages
* ( ) Protocol Versioning concept
  * (x) Each Request knows its own version and its response type
* ( ) Error handling
* ( ) Write a client

## Help Needed!

There is some stuff that I probably won't come around to implement, as I won't personally use it in the
forseeable futures:

* Transactional consumer/producer ("stream")
* Group-Offset-Commit coordination
* ...?

## Etymology

`Franzi` is a bavarian nickname for the female version of the name Franz, the first name of Mr. Kafka.

Also, when pronounced wrong, it sounds like 'frenzy', which I find a bit funny.

## Kafka Resources

* [A Guide To The Kafka Protocol](https://cwiki.apache.org/confluence/display/KAFKA/A+Guide+To+The+Kafka+Protocol)
* [Protocol Spec](http://kafka.apache.org/protocol.html)
* [KIP-227: Introduce Incremental FetchRequests to Increase Partition Scalability](https://cwiki.apache.org/confluence/display/KAFKA/KIP-227%3A+Introduce+Incremental+FetchRequests+to+Increase+Partition+Scalability) (About "Fetch Sessions", i.e. what values to set Fetch session_id and session_epoch to.)
* [ KIP-98 - Exactly Once Delivery and Transactional Messaging](https://cwiki.apache.org/confluence/display/KAFKA/KIP-98+-+Exactly+Once+Delivery+and+Transactional+Messaging)