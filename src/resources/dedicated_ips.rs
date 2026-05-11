//! Dedicated IP addresses.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, DeleteResponse, Page, PaginationParams};

/// Dedicated IPs resource handle.
#[derive(Debug, Clone)]
pub struct DedicatedIps {
    client: Sendry,
}

impl DedicatedIps {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Provision a new dedicated IP.
    pub async fn provision(&self, params: ProvisionDedicatedIp) -> Result<DedicatedIp, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/ips", &[], Some(&params)),
            )
            .await
    }

    /// List dedicated IPs.
    pub async fn list(&self, params: PaginationParams) -> Result<Page<DedicatedIp>, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(Method::GET, "/v1/ips", &q, None))
            .await
    }

    /// Retrieve a specific IP.
    pub async fn get(&self, id: &str) -> Result<DedicatedIp, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/ips/{id}"),
                &[],
                None,
            ))
            .await
    }

    /// Assign an IP to a domain.
    pub async fn assign(&self, ip_id: &str, params: AssignIp) -> Result<IpAssignment, Error> {
        self.client
            .request(self.client.build(
                Method::POST,
                &format!("/v1/ips/{ip_id}/assign"),
                &[],
                Some(&params),
            ))
            .await
    }

    /// Remove an IP assignment.
    pub async fn remove_assignment(
        &self,
        ip_id: &str,
        assignment_id: &str,
    ) -> Result<DeleteResponse, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/ips/{ip_id}/assign/{assignment_id}"),
                &[],
                None,
            ))
            .await
    }

    /// Release a dedicated IP.
    pub async fn release(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/ips/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// One IP assignment record (embedded in [`DedicatedIp`]).
#[derive(Debug, Clone, Deserialize)]
pub struct DedicatedIpAssignment {
    /// Assignment id.
    pub id: String,
    /// Domain id.
    pub domain_id: String,
    /// Domain name.
    pub domain_name: String,
    /// Creation timestamp.
    pub created_at: String,
}

/// Dedicated IP record.
#[derive(Debug, Clone, Deserialize)]
pub struct DedicatedIp {
    /// IP id.
    pub id: String,
    /// IPv4 / IPv6 address.
    pub ip_address: String,
    /// `ses` or `mailgun`.
    pub provider: String,
    /// `provisioning`, `warming`, `active`, or `inactive`.
    pub status: String,
    /// Warmup day (0-indexed).
    pub warmup_day: u32,
    /// Warmup progress percentage 0–100.
    pub warmup_progress: f64,
    /// Pool name.
    pub pool_name: Option<String>,
    /// Current assignments.
    #[serde(default)]
    pub assignments: Vec<DedicatedIpAssignment>,
    /// Creation timestamp.
    pub created_at: String,
}

/// Parameters for [`DedicatedIps::provision`].
#[derive(Debug, Clone, Serialize, Default)]
pub struct ProvisionDedicatedIp {
    /// `ses` (default) or `mailgun`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

/// Parameters for [`DedicatedIps::assign`].
#[derive(Debug, Clone, Serialize)]
pub struct AssignIp {
    /// Domain id.
    pub domain_id: String,
}

/// Assignment record returned by [`DedicatedIps::assign`].
#[derive(Debug, Clone, Deserialize)]
pub struct IpAssignment {
    /// Assignment id.
    pub id: String,
    /// IP id.
    pub ip_id: String,
    /// Domain id.
    pub domain_id: String,
    /// Creation timestamp.
    pub created_at: String,
}
