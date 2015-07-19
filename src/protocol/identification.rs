// use std;
use serde;


#[derive(Clone,Debug,Default)]
pub struct Identification<'i> {
    /// an identifier used to disambiguate this client (ie. something specific to the consumer)
    pub client_id: Option<&'i str>,
    /// the hostname where the client is deployed
    pub hostname: Option<&'i str>,
    /// bool used to indicate that the client supports feature negotiation. If the server is capable, it will send back a JSON payload of supported features and metadata.
    pub feature_negotiation: bool,
}

impl<'i> Identification<'i> {
    pub fn new() -> Identification<'i> {
        Identification {
            client_id: None,
            hostname: None,
            feature_negotiation: true,
        }
    }
    pub fn builder() -> IdentificationBuilder<'i> {
        IdentificationBuilder {
            i: Self::new(),
        }
    }
}

impl<'i> From<IdentificationBuilder<'i>> for Identification<'i> {
    fn from(i: IdentificationBuilder<'i>) -> Identification<'i> {
        i.build()
    }
}

impl<'i> From<Identification<'i>> for String {
    fn from(i: Identification<'i>) -> String {
        let mut json = serde::json::builder::ObjectBuilder::new();
        if let Some(client_id) = i.client_id {
            json = json.insert("client_id".to_string(), client_id);
        }
        if let Some(hostname) = i.hostname {
            json = json.insert("hostname".to_string(), hostname);
        }
        json = json.insert("feature_negotiation".to_string(), i.feature_negotiation);
        // heartbeat_interval
        // output_buffer_size
        // output_buffer_timeout
        // tls_v1
        // snappy
        // deflate
        // deflate_level
        // sample_rate
        // user_agent
        // msg_timeout
        let json: serde::json::Value = json.unwrap();
        serde::json::to_string(&json).unwrap()
    }
}


pub struct IdentificationBuilder<'i> {
    i: Identification<'i>,
}

impl<'i> IdentificationBuilder<'i> {
    /*
feature_negotiation (nsqd v0.2.19+) bool used to indicate that the client supports feature negotiation. If the server is capable, it will send back a JSON payload of supported features and metadata.

heartbeat_interval (nsqd v0.2.19+) milliseconds between heartbeats.

Valid range: 1000 <= heartbeat_interval <= configured_max (-1 disables heartbeats)

--max-heartbeat-interval (nsqd flag) controls the max

Defaults to --client-timeout / 2

output_buffer_size (nsqd v0.2.21+) the size in bytes of the buffer nsqd will use when writing to this client.

Valid range: 64 <= output_buffer_size <= configured_max (-1 disables output buffering)

--max-output-buffer-size (nsqd flag) controls the max

Defaults to 16kb

output_buffer_timeout (nsqd v0.2.21+) the timeout after which any data that nsqd has buffered will be flushed to this client.

Valid range: 1ms <= output_buffer_timeout <= configured_max (-1 disables timeouts)

--max-output-buffer-timeout (nsqd flag) controls the max

Defaults to 250ms

Warning: configuring clients with an extremely low (< 25ms) output_buffer_timeout has a significant effect on nsqd CPU usage (particularly with > 50 clients connected).

This is due to the current implementation relying on Go timers which are maintained by the Go runtime in a priority queue. See the commit message in pull request #236 for more details.

tls_v1 (nsqd v0.2.22+) enable TLS for this connection.

--tls-cert and --tls-key (nsqd flags) enable TLS and configure the server certificate

If the server supports TLS it will reply "tls_v1": true

The client should begin the TLS handshake immediately after reading the IDENTIFY response

The server will respond OK after completing the TLS handshake

snappy (nsqd v0.2.23+) enable snappy compression for this connection.

--snappy (nsqd flag) enables support for this server side

The client should expect an additional, snappy compressed OK response immediately after the IDENTIFY response.

A client cannot enable both snappy and deflate.

deflate (nsqd v0.2.23+) enable deflate compression for this connection.

--deflate (nsqd flag) enables support for this server side

The client should expect an additional, deflate compressed OK response immediately after the IDENTIFY response.

A client cannot enable both snappy and deflate.

deflate_level (nsqd v0.2.23+) configure the deflate compression level for this connection.

--max-deflate-level (nsqd flag) configures the maximum allowed value

Valid range: 1 <= deflate_level <= configured_max

Higher values mean better compression but more CPU usage for nsqd.

sample_rate (nsqd v0.2.25+) deliver a percentage of all messages received to this connection.

Valid range: 0 <= sample_rate <= 99 (0 disables sampling)

Defaults to 0

user_agent (nsqd v0.2.25+) a string identifying the agent for this client in the spirit of HTTP

Default: <client_library_name>/<version>

msg_timeout (nsqd v0.2.28+) configure the server-side message timeout in milliseconds for messages delivered to this client.

*/
    /// an identifier used to disambiguate this client (ie. something specific to the consumer)
    pub fn client_id(mut self, client_id: &'i str) -> IdentificationBuilder<'i> {
        self.i.client_id = Some(client_id);
        self
    }

    /// the hostname where the client is deployed
    pub fn hostname(mut self, hostname: &'i str) -> IdentificationBuilder<'i> {
        self.i.hostname = Some(hostname);
        self
    }

    pub fn build(self) -> Identification<'i> {
        self.i
    }
}
