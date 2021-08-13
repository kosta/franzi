# Franzi, a pure-rust [Kafka](https://kafka.apache.org) protocol implementation (not usable yet)

## Goals

* Pure rust Kafka client
* Support for all Kafka protocol versions
* Competitive performance; make this the fastest Kafka client if possible
* Full API Coverage
* Blocking and Non-blocking APIs (using async/await)
* Zero unsafe code (except in "well-known" dependencies, such as std, bytes, tokio, etc.)

## Current status

* In development, not usable
* async/await api only
* Code is very proof-of-concept, hacky, untested. Needs refactorings to get this production-ready.

## Modules overview

TODO: Links to docs.rs once it's up there

* `franzi-base` contains
  * Basic traits (`ToKafkaBytes`, `FromKafkaBytes`, `KafkaRequest`) and error types
  * `ApiKey` enum
  * Kafka primitive types (bool, i16, KafkaString, ...)
* `franzi-macros` contains
  * macros to derive `ToKafkaBytes`, `FroKafkaBytes`
  * macro `kafka_message!()` to generate Kafka messages from the [Kafka Protocol Spec](http://kafka.apache.org/protocol.html)
* `franzi-proto` contains
  * structs for each Kafka messages and its sub-structures
  * structs for Kafka message parts such as RequestHeader, ResponseHeader, Record etc.
* `franzi-client` contains
  * client structs to easily talk to kafka brokers, both low-level (sending specific messages to a specific broker) as well as high-level (producer/consumer etc. TODO)
* `franzi` command-line client (TODO)

## Progress

* (x) Implement derive(FromKafkaBytes, ToKafkaBytes)
* (x) Implement kafka_message!()
* (x) Implement varint, Record, MessageSet
* ( ) Implement all messages
* ( ) Protocol Versioning concept
  * (x) Each Request knows its own version and its response type
* ( ) Error handling
  * (x) Introduce basic error type
* ( ) Write a client
  * (x) Implement very basic client

## Out of scope (for now)

There is some stuff that I probably won't come around to implement, as I won't personally use it in the
forseeable futures:

* Transactional consumer/producer ("stream")
* ...?

## Etymology

`Franzi` is a bavarian diminuitive nickname of the name Franz, the first name of Mr. Kafka.

Also, when pronounced wrong, it sounds like 'frenzy', which I find a bit funny.

## Kafka Resources

* [A Guide To The Kafka Protocol](https://cwiki.apache.org/confluence/display/KAFKA/A+Guide+To+The+Kafka+Protocol)
* [Protocol Spec](http://kafka.apache.org/protocol.html)
* [KIP-227: Introduce Incremental FetchRequests to Increase Partition Scalability](https://cwiki.apache.org/confluence/display/KAFKA/KIP-227%3A+Introduce+Incremental+FetchRequests+to+Increase+Partition+Scalability) (About "Fetch Sessions", i.e. what values to set Fetch session_id and session_epoch to.)
* [KIP-98 - Exactly Once Delivery and Transactional Messaging](https://cwiki.apache.org/confluence/display/KAFKA/KIP-98+-+Exactly+Once+Delivery+and+Transactional+Messaging)
