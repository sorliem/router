use std::collections::HashMap;

use http::header::CONTENT_LENGTH;
use opentelemetry_api::Key;
use schemars::JsonSchema;
use serde::Deserialize;
use tower::BoxError;

use super::attributes::GetAttributes;
use crate::plugins::telemetry::config::AttributeValue;
use crate::plugins::telemetry::config_new::attributes::DefaultAttributeRequirementLevel;
use crate::plugins::telemetry::config_new::attributes::Extendable;
use crate::plugins::telemetry::config_new::attributes::HttpCommonAttributes;
use crate::plugins::telemetry::config_new::attributes::HttpServerAttributes;
use crate::plugins::telemetry::config_new::attributes::RouterCustomAttribute;
use crate::plugins::telemetry::config_new::attributes::SubgraphAttributes;
use crate::plugins::telemetry::config_new::attributes::SubgraphCustomAttribute;
use crate::plugins::telemetry::config_new::attributes::SupergraphAttributes;
use crate::plugins::telemetry::config_new::attributes::SupergraphCustomAttribute;
use crate::services::router;
use crate::tracer::TraceId;

#[allow(dead_code)]
#[derive(Deserialize, JsonSchema, Clone, Default, Debug)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct Spans {
    #[serde(skip)]
    /// Whether to create a `request` span. This will be removed in future, and users should set this to false.
    pub(crate) legacy_request_span: bool,

    /// The attributes to include by default in spans based on their level as specified in the otel semantic conventions and Apollo documentation.
    pub(crate) default_attribute_requirement_level: DefaultAttributeRequirementLevel,

    /// Configuration of router spans.
    /// Log events inherit attributes from the containing span, so attributes configured here will be included on log events for a request.
    /// Router spans contain http request and response information and therefore contain http specific attributes.
    pub(crate) router: RouterSpans,

    /// Configuration of supergraph spans.
    /// Supergraph spans contain information about the graphql request and response and therefore contain graphql specific attributes.
    pub(crate) supergraph: SupergraphSpans,

    /// Attributes to include on the subgraph span.
    /// Subgraph spans contain information about the subgraph request and response and therefore contain subgraph specific attributes.
    pub(crate) subgraph: SubgraphSpans,
}

#[allow(dead_code)]
#[derive(Deserialize, JsonSchema, Clone, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct RouterSpans {
    /// Custom attributes that are attached to the router span.
    pub(crate) attributes: Extendable<RouterAttributes, RouterCustomAttribute>,
}

#[allow(dead_code)]
#[derive(Deserialize, JsonSchema, Clone, Default, Debug)]
#[serde(default)]
pub(crate) struct RouterAttributes {
    /// Attach the datadog trace ID to the router span as dd.trace_id.
    /// This can be output in logs and used to correlate traces in Datadog.
    #[serde(rename = "dd.trace_id")]
    datadog_trace_id: Option<bool>,

    /// Attach the opentelemetry trace ID to the router span as trace_id.
    /// This can be output in logs.
    #[serde(rename = "trace_id")]
    trace_id: Option<bool>,

    /// Span http attributes from Open Telemetry semantic conventions.
    #[serde(flatten)]
    common: HttpCommonAttributes,
    /// Span http server attributes from Open Telemetry semantic conventions.
    // TODO unskip it
    #[serde(flatten, skip)]
    server: HttpServerAttributes,
}

#[allow(dead_code)]
#[derive(Deserialize, JsonSchema, Clone, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct SupergraphSpans {
    /// Custom attributes that are attached to the supergraph span.
    attributes: Extendable<SupergraphAttributes, SupergraphCustomAttribute>,
}

#[allow(dead_code)]
#[derive(Deserialize, JsonSchema, Clone, Default, Debug)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct SubgraphSpans {
    /// Custom attributes that are attached to the subgraph span.
    attributes: Extendable<SubgraphAttributes, SubgraphCustomAttribute>,
}

impl GetAttributes<router::Request, router::Response> for RouterAttributes {
    fn on_request(&self, request: &router::Request) -> HashMap<Key, AttributeValue> {
        let mut attrs = self.common.on_request(request);
        if let Some(true) = &self.trace_id {
            if let Some(trace_id) = TraceId::maybe_new().map(|t| t.to_string()) {
                attrs.insert("trace_id".into(), AttributeValue::String(trace_id));
            }
        }
        if let Some(true) = &self.datadog_trace_id {
            if let Some(trace_id) = TraceId::maybe_new().map(|t| t.to_u128()) {
                // TODO: fix it and use u128
                attrs.insert("dd.trace_id".into(), AttributeValue::I64(trace_id as i64));
            }
        }
        // if let Some(true) = &self.server.client_address {
        //     todo!();
        // }
        // if let Some(true) = &self.server.client_port {
        //     todo!();
        // }
        // if let Some(true) = &self.server.http_route {
        //     todo!();
        // }
        // if let Some(true) = &self.server.network_peer_address {
        //     todo!();
        // }
        // TODO: take other ones

        attrs
    }

    fn on_response(&self, response: &router::Response) -> HashMap<Key, AttributeValue> {
        let mut attrs = self.common.on_response(response);
        attrs
    }

    fn on_error(&self, error: &BoxError) -> HashMap<Key, AttributeValue> {
        let mut attrs = self.common.on_error(error);

        attrs
    }
}
