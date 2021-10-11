// This file contains a list of global meters that can be mutated by any part of the program.
// The goal of this global variable is to export metrics.

use lazy_static::lazy_static;
use opentelemetry::{
    metrics::{Counter, Meter},
    KeyValue,
};

const METER_NAME: &str = "kubewarden.io";

lazy_static! {
    pub(crate) static ref POLICY_EVALUATIONS: Counter<u64> =
        opentelemetry::global::meter(METER_NAME)
            .u64_counter("kubewarden_policy_evaluations_total")
            .init();
}

pub(crate) struct PolicyEvaluation {
    pub(crate) policy_name: String,
    pub(crate) resource_name: String,
    pub(crate) resource_kind: String,
    pub(crate) resource_namespace: Option<String>,
    pub(crate) resource_request_operation: String,
    pub(crate) accepted: bool,
    pub(crate) mutated: bool,
    pub(crate) error_code: u16,
}

impl Into<Vec<KeyValue>> for PolicyEvaluation {
    fn into(self) -> Vec<KeyValue> {
        Vec::new()
    }
}

pub(crate) fn registerPolicyEvaluation(policy_evaluation: PolicyEvaluation) {
    POLICY_EVALUATIONS.add(1, &Into::<Vec<KeyValue>>::into(policy_evaluation));
}
