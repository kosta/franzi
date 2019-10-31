// TODO: Find out how to deal with this :)

#![allow(non_camel_case_types)]

pub struct KafkaErrorCode {
    pub value: i8,
    pub retryable: bool,
    pub description: &'static str,
}

pub const UNKNOWN_SERVER_ERROR: KafkaErrorCode = KafkaErrorCode {
    value: -1,
    retryable: false,
    description: "The server experienced an unexpected error when processing the request.",
};
pub const NONE: KafkaErrorCode = KafkaErrorCode {
    value: 0,
    retryable: false,
    description: "",
};
pub const OFFSET_OUT_OF_RANGE: KafkaErrorCode = KafkaErrorCode {
    value: 1,
    retryable: false,
    description:
        "The requested offset is not within the range of offsets maintained by the server.",
};
pub const CORRUPT_MESSAGE: KafkaErrorCode = KafkaErrorCode{value: 2, retryable: true, description: "This message has failed its CRC checksum, exceeds the valid size, has a null key for a compacted topic, or is otherwise corrupt."};
pub const UNKNOWN_TOPIC_OR_PARTITION: KafkaErrorCode = KafkaErrorCode {
    value: 3,
    retryable: true,
    description: "This server does not host this topic-partition.",
};
pub const INVALID_FETCH_SIZE: KafkaErrorCode = KafkaErrorCode {
    value: 4,
    retryable: false,
    description: "The requested fetch size is invalid.",
};
pub const LEADER_NOT_AVAILABLE: KafkaErrorCode = KafkaErrorCode{value: 5, retryable: true, description: "There is no leader for this topic-partition as we are in the middle of a leadership election."};
pub const NOT_LEADER_FOR_PARTITION: KafkaErrorCode = KafkaErrorCode {
    value: 6,
    retryable: true,
    description: "This server is not the leader for that topic-partition.",
};
pub const REQUEST_TIMED_OUT: KafkaErrorCode = KafkaErrorCode {
    value: 7,
    retryable: true,
    description: "The request timed out.",
};
pub const BROKER_NOT_AVAILABLE: KafkaErrorCode = KafkaErrorCode {
    value: 8,
    retryable: false,
    description: "The broker is not available.",
};
pub const REPLICA_NOT_AVAILABLE: KafkaErrorCode = KafkaErrorCode {
    value: 9,
    retryable: false,
    description: "The replica is not available for the requested topic-partition.",
};
pub const MESSAGE_TOO_LARGE: KafkaErrorCode = KafkaErrorCode {
    value: 10,
    retryable: false,
    description:
        "The request included a message larger than the max message size the server will accept.",
};
pub const STALE_CONTROLLER_EPOCH: KafkaErrorCode = KafkaErrorCode {
    value: 11,
    retryable: false,
    description: "The controller moved to another broker.",
};
pub const OFFSET_METADATA_TOO_LARGE: KafkaErrorCode = KafkaErrorCode {
    value: 12,
    retryable: false,
    description: "The metadata field of the offset request was too large.",
};
pub const NETWORK_EXCEPTION: KafkaErrorCode = KafkaErrorCode {
    value: 13,
    retryable: true,
    description: "The server disconnected before a response was received.",
};
pub const COORDINATOR_LOAD_IN_PROGRESS: KafkaErrorCode = KafkaErrorCode {
    value: 14,
    retryable: true,
    description: "The coordinator is loading and hence can't process requests.",
};
pub const COORDINATOR_NOT_AVAILABLE: KafkaErrorCode = KafkaErrorCode {
    value: 15,
    retryable: true,
    description: "The coordinator is not available.",
};
pub const NOT_COORDINATOR: KafkaErrorCode = KafkaErrorCode {
    value: 16,
    retryable: true,
    description: "This is not the correct coordinator.",
};
pub const INVALID_TOPIC_EXCEPTION: KafkaErrorCode = KafkaErrorCode {
    value: 17,
    retryable: false,
    description: "The request attempted to perform an operation on an invalid topic.",
};
pub const RECORD_LIST_TOO_LARGE: KafkaErrorCode = KafkaErrorCode {
    value: 18,
    retryable: false,
    description:
        "The request included message batch larger than the configured segment size on the server.",
};
pub const NOT_ENOUGH_REPLICAS: KafkaErrorCode = KafkaErrorCode {
    value: 19,
    retryable: true,
    description: "Messages are rejected since there are fewer in-sync replicas than required.",
};
pub const NOT_ENOUGH_REPLICAS_AFTER_APPEND: KafkaErrorCode = KafkaErrorCode {
    value: 20,
    retryable: true,
    description: "Messages are written to the log, but to fewer in-sync replicas than required.",
};
pub const INVALID_REQUIRED_ACKS: KafkaErrorCode = KafkaErrorCode {
    value: 21,
    retryable: false,
    description: "Produce request specified an invalid value for required acks.",
};
pub const ILLEGAL_GENERATION: KafkaErrorCode = KafkaErrorCode {
    value: 22,
    retryable: false,
    description: "Specified group generation id is not valid.",
};
pub const INCONSISTENT_GROUP_PROTOCOL: KafkaErrorCode = KafkaErrorCode{value: 23, retryable: false, description: "The group member's supported protocols are incompatible with those of existing members or first group member tried to join with empty protocol type or empty protocol list."};
pub const INVALID_GROUP_ID: KafkaErrorCode = KafkaErrorCode {
    value: 24,
    retryable: false,
    description: "The configured groupId is invalid.",
};
pub const UNKNOWN_MEMBER_ID: KafkaErrorCode = KafkaErrorCode {
    value: 25,
    retryable: false,
    description: "The coordinator is not aware of this member.",
};
pub const INVALID_SESSION_TIMEOUT: KafkaErrorCode = KafkaErrorCode{value: 26, retryable: false, description: "The session timeout is not within the range allowed by the broker (as configured by group.min.session.timeout.ms and group.max.session.timeout.ms)."};
pub const REBALANCE_IN_PROGRESS: KafkaErrorCode = KafkaErrorCode {
    value: 27,
    retryable: false,
    description: "The group is rebalancing, so a rejoin is needed.",
};
pub const INVALID_COMMIT_OFFSET_SIZE: KafkaErrorCode = KafkaErrorCode {
    value: 28,
    retryable: false,
    description: "The committing offset data size is not valid.",
};
pub const TOPIC_AUTHORIZATION_FAILED: KafkaErrorCode = KafkaErrorCode {
    value: 29,
    retryable: false,
    description: "Not authorized to access topics: [Topic authorization failed.]",
};
pub const GROUP_AUTHORIZATION_FAILED: KafkaErrorCode = KafkaErrorCode {
    value: 30,
    retryable: false,
    description: "Not authorized to access group: Group authorization failed.",
};
pub const CLUSTER_AUTHORIZATION_FAILED: KafkaErrorCode = KafkaErrorCode {
    value: 31,
    retryable: false,
    description: "Cluster authorization failed.",
};
pub const INVALID_TIMESTAMP: KafkaErrorCode = KafkaErrorCode {
    value: 32,
    retryable: false,
    description: "The timestamp of the message is out of acceptable range.",
};
pub const UNSUPPORTED_SASL_MECHANISM: KafkaErrorCode = KafkaErrorCode {
    value: 33,
    retryable: false,
    description: "The broker does not support the requested SASL mechanism.",
};
pub const ILLEGAL_SASL_STATE: KafkaErrorCode = KafkaErrorCode {
    value: 34,
    retryable: false,
    description: "Request is not valid given the current SASL state.",
};
pub const UNSUPPORTED_VERSION: KafkaErrorCode = KafkaErrorCode {
    value: 35,
    retryable: false,
    description: "The version of API is not supported.",
};
pub const TOPIC_ALREADY_EXISTS: KafkaErrorCode = KafkaErrorCode {
    value: 36,
    retryable: false,
    description: "Topic with this name already exists.",
};
pub const INVALID_PARTITIONS: KafkaErrorCode = KafkaErrorCode {
    value: 37,
    retryable: false,
    description: "Number of partitions is below 1.",
};
pub const INVALID_REPLICATION_FACTOR: KafkaErrorCode = KafkaErrorCode {
    value: 38,
    retryable: false,
    description: "Replication factor is below 1 or larger than the number of available brokers.",
};
pub const INVALID_REPLICA_ASSIGNMENT: KafkaErrorCode = KafkaErrorCode {
    value: 39,
    retryable: false,
    description: "Replica assignment is invalid.",
};
pub const INVALID_CONFIG: KafkaErrorCode = KafkaErrorCode {
    value: 40,
    retryable: false,
    description: "Configuration is invalid.",
};
pub const NOT_CONTROLLER: KafkaErrorCode = KafkaErrorCode {
    value: 41,
    retryable: true,
    description: "This is not the correct controller for this cluster.",
};
pub const INVALID_REQUEST: KafkaErrorCode = KafkaErrorCode{value: 42, retryable: false, description: "This most likely occurs because of a request being malformed by the client library or the message was sent to an incompatible broker. See the broker logs for more details."};
pub const UNSUPPORTED_FOR_MESSAGE_FORMAT: KafkaErrorCode = KafkaErrorCode {
    value: 43,
    retryable: false,
    description: "The message format version on the broker does not support the request.",
};
pub const POLICY_VIOLATION: KafkaErrorCode = KafkaErrorCode {
    value: 44,
    retryable: false,
    description: "Request parameters do not satisfy the configured policy.",
};
pub const OUT_OF_ORDER_SEQUENCE_NUMBER: KafkaErrorCode = KafkaErrorCode {
    value: 45,
    retryable: false,
    description: "The broker received an out of order sequence number.",
};
pub const DUPLICATE_SEQUENCE_NUMBER: KafkaErrorCode = KafkaErrorCode {
    value: 46,
    retryable: false,
    description: "The broker received a duplicate sequence number.",
};
pub const INVALID_PRODUCER_EPOCH: KafkaErrorCode = KafkaErrorCode{value: 47, retryable: false, description: "Producer attempted an operation with an old epoch. Either there is a newer producer with the same transactionalId, or the producer's transaction has been expired by the broker."};
pub const INVALID_TXN_STATE: KafkaErrorCode = KafkaErrorCode {
    value: 48,
    retryable: false,
    description: "The producer attempted a transactional operation in an invalid state.",
};
pub const INVALID_PRODUCER_ID_MAPPING: KafkaErrorCode = KafkaErrorCode{value: 49, retryable: false, description: "The producer attempted to use a producer id which is not currently assigned to its transactional id."};
pub const INVALID_TRANSACTION_TIMEOUT: KafkaErrorCode = KafkaErrorCode{value: 50, retryable: false, description: "The transaction timeout is larger than the maximum value allowed by the broker (as configured by transaction.max.timeout.ms)."};
pub const CONCURRENT_TRANSACTIONS: KafkaErrorCode = KafkaErrorCode{value: 51, retryable: false, description: "The producer attempted to update a transaction while another concurrent operation on the same transaction was ongoing."};
pub const TRANSACTION_COORDINATOR_FENCED: KafkaErrorCode = KafkaErrorCode{value: 52, retryable: false, description: "Indicates that the transaction coordinator sending a WriteTxnMarker is no longer the current coordinator for a given producer."};
pub const TRANSACTIONAL_ID_AUTHORIZATION_FAILED: KafkaErrorCode = KafkaErrorCode {
    value: 53,
    retryable: false,
    description: "Transactional Id authorization failed.",
};
pub const SECURITY_DISABLED: KafkaErrorCode = KafkaErrorCode {
    value: 54,
    retryable: false,
    description: "Security features are disabled.",
};
pub const OPERATION_NOT_ATTEMPTED: KafkaErrorCode = KafkaErrorCode{value: 55, retryable: false, description: "The broker did not attempt to execute this operation. This may happen for batched RPCs where some operations in the batch failed, causing the broker to respond without trying the rest."};
pub const KAFKA_STORAGE_ERROR: KafkaErrorCode = KafkaErrorCode {
    value: 56,
    retryable: true,
    description: "Disk error when trying to access log file on the disk.",
};
pub const LOG_DIR_NOT_FOUND: KafkaErrorCode = KafkaErrorCode {
    value: 57,
    retryable: false,
    description: "The user-specified log directory is not found in the broker config.",
};
pub const SASL_AUTHENTICATION_FAILED: KafkaErrorCode = KafkaErrorCode {
    value: 58,
    retryable: false,
    description: "SASL Authentication failed.",
};
pub const UNKNOWN_PRODUCER_ID: KafkaErrorCode = KafkaErrorCode{value: 59, retryable: false, description: "This exception is raised by the broker if it could not locate the producer metadata associated with the producerId in question. This could happen if, for instance, the producer's records were deleted because their retention time had elapsed. Once the last records of the producerId are removed, the producer's metadata is removed from the broker, and future appends by the producer will return this exception."};
pub const REASSIGNMENT_IN_PROGRESS: KafkaErrorCode = KafkaErrorCode {
    value: 60,
    retryable: false,
    description: "A partition reassignment is in progress.",
};
pub const DELEGATION_TOKEN_AUTH_DISABLED: KafkaErrorCode = KafkaErrorCode {
    value: 61,
    retryable: false,
    description: "Delegation Token feature is not enabled.",
};
pub const DELEGATION_TOKEN_NOT_FOUND: KafkaErrorCode = KafkaErrorCode {
    value: 62,
    retryable: false,
    description: "Delegation Token is not found on server.",
};
pub const DELEGATION_TOKEN_OWNER_MISMATCH: KafkaErrorCode = KafkaErrorCode {
    value: 63,
    retryable: false,
    description: "Specified Principal is not valid Owner/Renewer.",
};
pub const DELEGATION_TOKEN_REQUEST_NOT_ALLOWED: KafkaErrorCode = KafkaErrorCode{value: 64, retryable: false, description: "Delegation Token requests are not allowed on PLAINTEXT/1-way SSL channels and on delegation token authenticated channels."};
pub const DELEGATION_TOKEN_AUTHORIZATION_FAILED: KafkaErrorCode = KafkaErrorCode {
    value: 65,
    retryable: false,
    description: "Delegation Token authorization failed.",
};
pub const DELEGATION_TOKEN_EXPIRED: KafkaErrorCode = KafkaErrorCode {
    value: 66,
    retryable: false,
    description: "Delegation Token is expired.",
};
pub const INVALID_PRINCIPAL_TYPE: KafkaErrorCode = KafkaErrorCode {
    value: 67,
    retryable: false,
    description: "Supplied principalType is not supported.",
};
pub const NON_EMPTY_GROUP: KafkaErrorCode = KafkaErrorCode {
    value: 68,
    retryable: false,
    description: "The group is not empty.",
};
pub const GROUP_ID_NOT_FOUND: KafkaErrorCode = KafkaErrorCode {
    value: 69,
    retryable: false,
    description: "The group id does not exist.",
};
pub const FETCH_SESSION_ID_NOT_FOUND: KafkaErrorCode = KafkaErrorCode {
    value: 70,
    retryable: true,
    description: "The fetch session ID was not found.",
};
pub const INVALID_FETCH_SESSION_EPOCH: KafkaErrorCode = KafkaErrorCode {
    value: 71,
    retryable: true,
    description: "The fetch session epoch is invalid.",
};
pub const LISTENER_NOT_FOUND: KafkaErrorCode = KafkaErrorCode{value: 72, retryable: true, description: "There is no listener on the leader broker that matches the listener on which metadata request was processed."};
pub const TOPIC_DELETION_DISABLED: KafkaErrorCode = KafkaErrorCode {
    value: 73,
    retryable: false,
    description: "Topic deletion is disabled.",
};
pub const FENCED_LEADER_EPOCH: KafkaErrorCode = KafkaErrorCode {
    value: 74,
    retryable: true,
    description: "The leader epoch in the request is older than the epoch on the broker",
};
pub const UNKNOWN_LEADER_EPOCH: KafkaErrorCode = KafkaErrorCode {
    value: 75,
    retryable: true,
    description: "The leader epoch in the request is newer than the epoch on the broker",
};
pub const UNSUPPORTED_COMPRESSION_TYPE: KafkaErrorCode = KafkaErrorCode {
    value: 76,
    retryable: false,
    description: "The requesting client does not support the compression type of given partition.",
};

#[allow(dead_code)] // TODO: Use me
enum KafkaError {
    UNKNOWN_SERVER_ERROR = -1,
    NONE = 0,
    OFFSET_OUT_OF_RANGE = 1,
    CORRUPT_MESSAGE = 2,
    UNKNOWN_TOPIC_OR_PARTITION = 3,
    INVALID_FETCH_SIZE = 4,
    LEADER_NOT_AVAILABLE = 5,
    NOT_LEADER_FOR_PARTITION = 6,
    REQUEST_TIMED_OUT = 7,
    BROKER_NOT_AVAILABLE = 8,
    REPLICA_NOT_AVAILABLE = 9,
    MESSAGE_TOO_LARGE = 10,
    STALE_CONTROLLER_EPOCH = 11,
    OFFSET_METADATA_TOO_LARGE = 12,
    NETWORK_EXCEPTION = 13,
    COORDINATOR_LOAD_IN_PROGRESS = 14,
    COORDINATOR_NOT_AVAILABLE = 15,
    NOT_COORDINATOR = 16,
    INVALID_TOPIC_EXCEPTION = 17,
    RECORD_LIST_TOO_LARGE = 18,
    NOT_ENOUGH_REPLICAS = 19,
    NOT_ENOUGH_REPLICAS_AFTER_APPEND = 20,
    INVALID_REQUIRED_ACKS = 21,
    ILLEGAL_GENERATION = 22,
    INCONSISTENT_GROUP_PROTOCOL = 23,
    INVALID_GROUP_ID = 24,
    UNKNOWN_MEMBER_ID = 25,
    INVALID_SESSION_TIMEOUT = 26,
    REBALANCE_IN_PROGRESS = 27,
    INVALID_COMMIT_OFFSET_SIZE = 28,
    TOPIC_AUTHORIZATION_FAILED = 29,
    GROUP_AUTHORIZATION_FAILED = 30,
    CLUSTER_AUTHORIZATION_FAILED = 31,
    INVALID_TIMESTAMP = 32,
    UNSUPPORTED_SASL_MECHANISM = 33,
    ILLEGAL_SASL_STATE = 34,
    UNSUPPORTED_VERSION = 35,
    TOPIC_ALREADY_EXISTS = 36,
    INVALID_PARTITIONS = 37,
    INVALID_REPLICATION_FACTOR = 38,
    INVALID_REPLICA_ASSIGNMENT = 39,
    INVALID_CONFIG = 40,
    NOT_CONTROLLER = 41,
    INVALID_REQUEST = 42,
    UNSUPPORTED_FOR_MESSAGE_FORMAT = 43,
    POLICY_VIOLATION = 44,
    OUT_OF_ORDER_SEQUENCE_NUMBER = 45,
    DUPLICATE_SEQUENCE_NUMBER = 46,
    INVALID_PRODUCER_EPOCH = 47,
    INVALID_TXN_STATE = 48,
    INVALID_PRODUCER_ID_MAPPING = 49,
    INVALID_TRANSACTION_TIMEOUT = 50,
    CONCURRENT_TRANSACTIONS = 51,
    TRANSACTION_COORDINATOR_FENCED = 52,
    TRANSACTIONAL_ID_AUTHORIZATION_FAILED = 53,
    SECURITY_DISABLED = 54,
    OPERATION_NOT_ATTEMPTED = 55,
    KAFKA_STORAGE_ERROR = 56,
    LOG_DIR_NOT_FOUND = 57,
    SASL_AUTHENTICATION_FAILED = 58,
    UNKNOWN_PRODUCER_ID = 59,
    REASSIGNMENT_IN_PROGRESS = 60,
    DELEGATION_TOKEN_AUTH_DISABLED = 61,
    DELEGATION_TOKEN_NOT_FOUND = 62,
    DELEGATION_TOKEN_OWNER_MISMATCH = 63,
    DELEGATION_TOKEN_REQUEST_NOT_ALLOWED = 64,
    DELEGATION_TOKEN_AUTHORIZATION_FAILED = 65,
    DELEGATION_TOKEN_EXPIRED = 66,
    INVALID_PRINCIPAL_TYPE = 67,
    NON_EMPTY_GROUP = 68,
    GROUP_ID_NOT_FOUND = 69,
    FETCH_SESSION_ID_NOT_FOUND = 70,
    INVALID_FETCH_SESSION_EPOCH = 71,
    LISTENER_NOT_FOUND = 72,
    TOPIC_DELETION_DISABLED = 73,
    FENCED_LEADER_EPOCH = 74,
    UNKNOWN_LEADER_EPOCH = 75,
    UNSUPPORTED_COMPRESSION_TYPE = 76,
}
