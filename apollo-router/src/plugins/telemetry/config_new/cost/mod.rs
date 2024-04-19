use schemars::JsonSchema;
use serde::Deserialize;

/// Attributes for cost telemetry
#[derive(Deserialize, JsonSchema, Clone, Default, Debug)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct CostAttributes {
    /// The estimated cost of the operation using the currently configured cost model
    #[serde(rename = "cost.estimated")]
    cost_estimated: Option<bool>,
    /// The actual cost of the operation using the currently configured cost model
    #[serde(rename = "cost.actual")]
    cost_actual: Option<bool>,
    /// The delta between the estimated and actual cost of the operation using the currently configured cost model
    #[serde(rename = "cost.delta")]
    cost_delta: Option<bool>,
    /// The resolution of the cost calculation. This is the error code returned by the cost calculation.
    #[serde(rename = "cost.result")]
    cost_result: Option<bool>,
}

#[derive(Deserialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub(crate) enum CostValue {
    /// The estimated cost of the operation using the currently configured cost model
    Estimated,
    /// The actual cost of the operation using the currently configured cost model
    Actual,
    /// The delta between the estimated and actual cost of the operation using the currently configured cost model
    Delta,
    /// The result of the cost calculation. This is the error code returned by the cost calculation.
    Result,
}
