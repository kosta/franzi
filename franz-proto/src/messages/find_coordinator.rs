use franz_macros::kafka_message;

kafka_message!(
  "FindCoordinator Request (Version: 2) => coordinator_key coordinator_type
    coordinator_key => STRING
    coordinator_type => INT8"
);

// "Field 	Description
// coordinator_key	Id to use for finding the coordinator (for groups, this is the groupId, for transactional producers, this is the transactional id)
// coordinator_type	The type of coordinator to find (0 = group, 1 = transaction)"
