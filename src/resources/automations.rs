//! Automations — workflows that run steps for contacts.

use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{client::Sendry, error::Error, DeleteResponse, Page};

/// Automations resource handle.
#[derive(Debug, Clone)]
pub struct Automations {
    client: Sendry,
}

impl Automations {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// List automations.
    pub async fn list(&self, params: ListAutomations) -> Result<Page<Automation>, Error> {
        let q = params.to_query();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/automations", &q, None),
            )
            .await
    }

    /// Get an automation by id.
    pub async fn get(&self, id: &str) -> Result<Automation, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/automations/{id}"),
                &[],
                None,
            ))
            .await
    }

    /// Create an automation.
    pub async fn create(&self, params: CreateAutomation) -> Result<Automation, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/automations", &[], Some(&params)),
            )
            .await
    }

    /// Update an automation (partial).
    pub async fn update(
        &self,
        id: &str,
        params: UpdateAutomation,
    ) -> Result<Automation, Error> {
        self.client
            .request(self.client.build(
                Method::PATCH,
                &format!("/v1/automations/{id}"),
                &[],
                Some(&params),
            ))
            .await
    }

    /// Delete an automation.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/automations/{id}"),
                &[],
                None,
            ))
            .await
    }

    /// Activate an automation.
    pub async fn activate(&self, id: &str) -> Result<Automation, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::POST,
                &format!("/v1/automations/{id}/activate"),
                &[],
                None,
            ))
            .await
    }

    /// Pause an automation.
    pub async fn pause(&self, id: &str) -> Result<Automation, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::POST,
                &format!("/v1/automations/{id}/pause"),
                &[],
                None,
            ))
            .await
    }

    /// Archive an automation.
    pub async fn archive(&self, id: &str) -> Result<Automation, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::POST,
                &format!("/v1/automations/{id}/archive"),
                &[],
                None,
            ))
            .await
    }

    /// Steps sub-resource.
    #[must_use]
    pub fn steps(&self) -> AutomationSteps {
        AutomationSteps { client: self.client.clone() }
    }

    /// Runs sub-resource.
    #[must_use]
    pub fn runs(&self) -> AutomationRuns {
        AutomationRuns { client: self.client.clone() }
    }
}

/// Automation steps sub-resource handle.
#[derive(Debug, Clone)]
pub struct AutomationSteps {
    client: Sendry,
}

impl AutomationSteps {
    /// List steps for an automation.
    pub async fn list(&self, automation_id: &str) -> Result<AutomationStepList, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/automations/{automation_id}/steps"),
                &[],
                None,
            ))
            .await
    }

    /// Add a new step.
    pub async fn add(
        &self,
        automation_id: &str,
        params: AddAutomationStep,
    ) -> Result<AutomationStep, Error> {
        self.client
            .request(self.client.build(
                Method::POST,
                &format!("/v1/automations/{automation_id}/steps"),
                &[],
                Some(&params),
            ))
            .await
    }

    /// Update a step.
    pub async fn update(
        &self,
        automation_id: &str,
        step_id: &str,
        params: UpdateAutomationStep,
    ) -> Result<AutomationStep, Error> {
        self.client
            .request(self.client.build(
                Method::PATCH,
                &format!("/v1/automations/{automation_id}/steps/{step_id}"),
                &[],
                Some(&params),
            ))
            .await
    }

    /// Delete a step.
    pub async fn delete(
        &self,
        automation_id: &str,
        step_id: &str,
    ) -> Result<DeleteResponse, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/automations/{automation_id}/steps/{step_id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Automation runs sub-resource handle.
#[derive(Debug, Clone)]
pub struct AutomationRuns {
    client: Sendry,
}

impl AutomationRuns {
    /// List runs for an automation.
    pub async fn list(
        &self,
        automation_id: &str,
        params: ListAutomationRuns,
    ) -> Result<Page<AutomationRun>, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/automations/{automation_id}/runs"),
                &q,
                None,
            ))
            .await
    }

    /// Get a single run.
    pub async fn get(
        &self,
        automation_id: &str,
        run_id: &str,
    ) -> Result<AutomationRun, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/automations/{automation_id}/runs/{run_id}"),
                &[],
                None,
            ))
            .await
    }

    /// List a run's executed steps.
    pub async fn list_steps(
        &self,
        automation_id: &str,
        run_id: &str,
    ) -> Result<AutomationRunStepList, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/automations/{automation_id}/runs/{run_id}/steps"),
                &[],
                None,
            ))
            .await
    }

    /// Cancel an in-progress run.
    pub async fn cancel(
        &self,
        automation_id: &str,
        run_id: &str,
    ) -> Result<AutomationRun, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::POST,
                &format!("/v1/automations/{automation_id}/runs/{run_id}/cancel"),
                &[],
                None,
            ))
            .await
    }

    /// Create a manual run for a contact.
    pub async fn create(
        &self,
        automation_id: &str,
        params: CreateAutomationRun,
    ) -> Result<AutomationRun, Error> {
        self.client
            .request(self.client.build(
                Method::POST,
                &format!("/v1/automations/{automation_id}/runs"),
                &[],
                Some(&params),
            ))
            .await
    }
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Automation record.
#[derive(Debug, Clone, Deserialize)]
pub struct Automation {
    /// Automation id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Description.
    pub description: Option<String>,
    /// `draft`, `active`, `paused`, or `archived`.
    pub status: String,
    /// Trigger type.
    pub trigger_type: String,
    /// Trigger config blob.
    pub trigger_config: Value,
    /// Optional entry segment.
    pub entry_segment_id: Option<String>,
    /// Re-entry policy.
    pub reentry_policy: String,
    /// Cooldown in seconds (when policy = cooldown).
    pub reentry_cooldown_seconds: Option<u64>,
    /// Total runs ever.
    pub total_runs: u64,
    /// Currently active runs.
    pub active_runs: u64,
    /// Completed runs.
    pub completed_runs: u64,
    /// Failed runs.
    pub failed_runs: u64,
    /// Created.
    pub created_at: String,
    /// Updated.
    pub updated_at: String,
}

/// Filters for [`Automations::list`].
#[derive(Debug, Clone, Default)]
pub struct ListAutomations {
    /// Page size.
    pub limit: Option<u32>,
    /// Cursor.
    pub cursor: Option<String>,
    /// Filter by status.
    pub status: Option<String>,
}

impl ListAutomations {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = self.limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = &self.cursor {
            q.push(("cursor", v.clone()));
        }
        if let Some(v) = &self.status {
            q.push(("status", v.clone()));
        }
        q
    }
}

/// Parameters for [`Automations::create`].
#[derive(Debug, Clone, Serialize)]
pub struct CreateAutomation {
    /// Display name.
    pub name: String,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Trigger type: `event`, `contact_added_to_segment`, `schedule`, `manual`.
    pub trigger_type: String,
    /// Trigger config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_config: Option<Value>,
    /// Entry segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_segment_id: Option<String>,
    /// Re-entry policy: `once_per_contact`, `cooldown`, `always`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reentry_policy: Option<String>,
    /// Cooldown in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reentry_cooldown_seconds: Option<u64>,
}

/// Parameters for [`Automations::update`]. All fields optional.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAutomation {
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Trigger config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_config: Option<Value>,
    /// Entry segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_segment_id: Option<String>,
    /// Re-entry policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reentry_policy: Option<String>,
    /// Cooldown in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reentry_cooldown_seconds: Option<u64>,
}

/// A/B split definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbSplit {
    /// Weight for branch A.
    pub a: u32,
    /// Weight for branch B.
    pub b: u32,
}

/// Branch condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCondition {
    /// Operator (e.g. `eq`, `gt`, `contains`, `in_segment`, `did_event`).
    pub op: String,
    /// Property path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// Comparison value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// Segment id (for `in_segment`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// Event name (for `did_event`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// Within window in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub within_seconds: Option<u64>,
}

/// One step config — discriminated by `type` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AutomationStepConfig {
    /// Send an email.
    SendEmail {
        /// Template id.
        #[serde(skip_serializing_if = "Option::is_none")]
        template_id: Option<String>,
        /// From address.
        from: String,
        /// Reply-to address.
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to: Option<String>,
        /// Subject.
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<String>,
        /// HTML body.
        #[serde(skip_serializing_if = "Option::is_none")]
        html: Option<String>,
        /// Plain-text body.
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        /// Message type — `transactional` or `marketing`.
        #[serde(skip_serializing_if = "Option::is_none")]
        message_type: Option<String>,
        /// Topic id.
        #[serde(skip_serializing_if = "Option::is_none")]
        topic_id: Option<String>,
        /// Template variables.
        #[serde(skip_serializing_if = "Option::is_none")]
        variables: Option<HashMap<String, String>>,
    },
    /// Wait for the given number of seconds.
    Wait {
        /// Wait duration.
        duration_seconds: u64,
    },
    /// Branch the flow based on a condition.
    Branch {
        /// Condition.
        condition: BranchCondition,
    },
    /// Split traffic A/B.
    AbSplit {
        /// Split definition.
        split: AbSplit,
        /// Optional deterministic seed.
        #[serde(skip_serializing_if = "Option::is_none")]
        seed: Option<String>,
    },
}

/// One step record.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationStep {
    /// Step id.
    pub id: String,
    /// Owning automation id.
    pub automation_id: String,
    /// Parent step id (for branches).
    pub parent_step_id: Option<String>,
    /// Branch label.
    pub branch_label: Option<String>,
    /// Position within parent.
    pub position: u32,
    /// Step type.
    #[serde(rename = "type")]
    pub step_type: String,
    /// Step config (raw — also includes `type`).
    pub config: Value,
    /// Created.
    pub created_at: String,
    /// Updated.
    pub updated_at: String,
}

/// Steps list response.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationStepList {
    /// Rows.
    pub data: Vec<AutomationStep>,
}

/// Parameters for [`AutomationSteps::add`].
#[derive(Debug, Clone, Serialize)]
pub struct AddAutomationStep {
    /// Parent step id (for branching).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_step_id: Option<String>,
    /// Branch label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_label: Option<String>,
    /// Position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    /// Config — see [`AutomationStepConfig`].
    pub config: AutomationStepConfig,
}

/// Parameters for [`AutomationSteps::update`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAutomationStep {
    /// Parent step id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_step_id: Option<String>,
    /// Branch label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_label: Option<String>,
    /// Position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    /// New config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<AutomationStepConfig>,
}

/// One automation run.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationRun {
    /// Run id.
    pub id: String,
    /// Automation id.
    pub automation_id: String,
    /// Contact id.
    pub contact_id: Option<String>,
    /// Contact email.
    pub contact_email: String,
    /// Trigger event id.
    pub trigger_event_id: Option<String>,
    /// Status.
    pub status: String,
    /// Current step.
    pub current_step_id: Option<String>,
    /// Per-run context.
    pub context: Value,
    /// Started.
    pub started_at: String,
    /// Completed.
    pub completed_at: Option<String>,
    /// Failed.
    pub failed_at: Option<String>,
    /// Failure reason.
    pub failure_reason: Option<String>,
    /// Created.
    pub created_at: String,
    /// Updated.
    pub updated_at: String,
}

/// One step execution within a run.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationRunStep {
    /// Row id.
    pub id: String,
    /// Owning run id.
    pub run_id: String,
    /// Step id.
    pub step_id: String,
    /// Status.
    pub status: String,
    /// Email id (for send steps).
    pub email_id: Option<String>,
    /// Branch taken (for branches).
    pub branch_taken: Option<String>,
    /// Scheduled for.
    pub scheduled_for: Option<String>,
    /// Started.
    pub started_at: Option<String>,
    /// Completed.
    pub completed_at: Option<String>,
    /// Error.
    pub error: Option<String>,
    /// Created.
    pub created_at: String,
}

/// Run steps list response.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationRunStepList {
    /// Rows.
    pub data: Vec<AutomationRunStep>,
}

/// Filters for [`AutomationRuns::list`].
#[derive(Debug, Clone, Default)]
pub struct ListAutomationRuns {
    /// Page size.
    pub limit: Option<u32>,
    /// Cursor.
    pub cursor: Option<String>,
    /// Filter by status.
    pub status: Option<String>,
}

impl ListAutomationRuns {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = self.limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = &self.cursor {
            q.push(("cursor", v.clone()));
        }
        if let Some(v) = &self.status {
            q.push(("status", v.clone()));
        }
        q
    }
}

/// Parameters for [`AutomationRuns::create`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAutomationRun {
    /// Contact id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// Contact email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    /// Run context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,
}
