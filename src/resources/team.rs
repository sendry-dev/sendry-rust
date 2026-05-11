//! Team membership and invites.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, DeleteResponse};

/// Team resource handle.
#[derive(Debug, Clone)]
pub struct Team {
    client: Sendry,
}

impl Team {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// List members and seat usage.
    pub async fn list(&self) -> Result<ListTeamResponse, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/team", &[], None),
            )
            .await
    }

    /// Invite a member by email.
    pub async fn invite(&self, params: InviteTeamMember) -> Result<TeamMember, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/team/invite", &[], Some(&params)),
            )
            .await
    }

    /// Remove a member.
    pub async fn remove(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/team/{id}"),
                &[],
                None,
            ))
            .await
    }

    /// Change a member's role.
    pub async fn update_role(
        &self,
        id: &str,
        params: UpdateTeamMemberRole,
    ) -> Result<TeamMember, Error> {
        self.client
            .request(self.client.build(
                Method::PATCH,
                &format!("/v1/team/{id}/role"),
                &[],
                Some(&params),
            ))
            .await
    }
}

/// One member or pending invite.
#[derive(Debug, Clone, Deserialize)]
pub struct TeamMember {
    /// Member id.
    pub id: String,
    /// Owning org id.
    pub org_id: String,
    /// User id (null if invite not accepted).
    pub user_id: Option<String>,
    /// Email.
    pub email: String,
    /// Display name.
    pub name: Option<String>,
    /// Role.
    pub role: String,
    /// `pending` or `active`.
    pub status: String,
    /// Invite timestamp.
    pub invited_at: String,
    /// Acceptance timestamp.
    pub joined_at: Option<String>,
}

/// Seat usage.
#[derive(Debug, Clone, Deserialize)]
pub struct TeamSeats {
    /// Used seats.
    pub used: u32,
    /// Limit.
    pub limit: u32,
    /// Whether the plan is unlimited.
    pub unlimited: bool,
}

/// Response from [`Team::list`].
#[derive(Debug, Clone, Deserialize)]
pub struct ListTeamResponse {
    /// Members.
    pub data: Vec<TeamMember>,
    /// Seat usage.
    pub seats: TeamSeats,
    /// Current plan.
    pub plan: String,
}

/// Parameters for [`Team::invite`].
#[derive(Debug, Clone, Serialize)]
pub struct InviteTeamMember {
    /// Email.
    pub email: String,
    /// `admin` or `member` (default `member`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Parameters for [`Team::update_role`].
#[derive(Debug, Clone, Serialize)]
pub struct UpdateTeamMemberRole {
    /// New role.
    pub role: String,
}
