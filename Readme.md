# Franz, a pure-rust kafka protocol implementation

## Goals

* Pure rust Kafka client
* Support for multiple Kafka protocol versions
* Competitive performance; make this the fastest Kafka client if possible
* Full API Coverage
* Blocking and Non-blocking APIs (using futures)

## Current status

* In development, not usable

## Progress

* ( ) Implement derive(FromKafkaBytes, ToKafkaBytes)
* ( ) Implement message!()
* ( ) Implement all messages
* ( ) Protocol Versioning concept
* ( ) Error handling
* ( ) Write a client

## Kafka Resources

* [A Guide To The Kafka Protocol](https://cwiki.apache.org/confluence/display/KAFKA/A+Guide+To+The+Kafka+Protocol)
* [Protocol Spec](http://kafka.apache.org/protocol.html)
* [KIP-227: Introduce Incremental FetchRequests to Increase Partition Scalability](https://cwiki.apache.org/confluence/display/KAFKA/KIP-227%3A+Introduce+Incremental+FetchRequests+to+Increase+Partition+Scalability) (About "Fetch Sessions", i.e. what values to set Fetch session_id and session_epoch to.)
* [ KIP-98 - Exactly Once Delivery and Transactional Messaging](https://cwiki.apache.org/confluence/display/KAFKA/KIP-98+-+Exactly+Once+Delivery+and+Transactional+Messaging)