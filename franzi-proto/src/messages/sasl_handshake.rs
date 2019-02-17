kafka_message!(
    "SaslHandshake Request (Version: 0) => mechanism
  mechanism => STRING

Field 	Description
mechanism	SASL Mechanism chosen by the client."
);

kafka_message!(
    "SaslHandshake Request (Version: 1) => mechanism
  mechanism => STRING

Field 	Description
mechanism	SASL Mechanism chosen by the client."
);

kafka_message!(
    "SaslHandshake Response (Version: 0) => error_code [enabled_mechanisms]
  error_code => INT16
  enabled_mechanisms => STRING

Field 	Description
error_code	Response error code
enabled_mechanisms	Array of mechanisms enabled in the server."
);

kafka_message!(
    "SaslHandshake Response (Version: 1) => error_code [enabled_mechanisms]
  error_code => INT16
  enabled_mechanisms => STRING

Field 	Description
error_code	Response error code
enabled_mechanisms	Array of mechanisms enabled in the server."
);
