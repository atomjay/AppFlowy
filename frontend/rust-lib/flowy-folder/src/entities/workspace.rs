use crate::{
    entities::parser::workspace::{WorkspaceDesc, WorkspaceIdentify, WorkspaceName},
    entities::{app::RepeatedAppPB, view::ViewPB},
    errors::*,
    impl_def_and_def_mut,
};
use flowy_derive::ProtoBuf;
use folder_rev_model::WorkspaceRevision;
use std::convert::TryInto;

#[derive(Eq, PartialEq, ProtoBuf, Default, Debug, Clone)]
pub struct WorkspacePB {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2)]
    pub name: String,

    #[pb(index = 3)]
    pub desc: String,

    #[pb(index = 4)]
    pub apps: RepeatedAppPB,

    #[pb(index = 5)]
    pub modified_time: i64,

    #[pb(index = 6)]
    pub create_time: i64,
}

impl std::convert::From<WorkspaceRevision> for WorkspacePB {
    fn from(workspace_serde: WorkspaceRevision) -> Self {
        WorkspacePB {
            id: workspace_serde.id,
            name: workspace_serde.name,
            desc: workspace_serde.desc,
            apps: workspace_serde.apps.into(),
            modified_time: workspace_serde.modified_time,
            create_time: workspace_serde.create_time,
        }
    }
}
#[derive(PartialEq, Debug, Default, ProtoBuf)]
pub struct RepeatedWorkspacePB {
    #[pb(index = 1)]
    pub items: Vec<WorkspacePB>,
}

impl_def_and_def_mut!(RepeatedWorkspacePB, WorkspacePB);

#[derive(ProtoBuf, Default)]
pub struct CreateWorkspacePayloadPB {
    #[pb(index = 1)]
    pub name: String,

    #[pb(index = 2)]
    pub desc: String,
}

#[derive(Clone, Debug)]
pub struct CreateWorkspaceParams {
    pub name: String,
    pub desc: String,
}

impl TryInto<CreateWorkspaceParams> for CreateWorkspacePayloadPB {
    type Error = ErrorCode;

    fn try_into(self) -> Result<CreateWorkspaceParams, Self::Error> {
        let name = WorkspaceName::parse(self.name)?;
        let desc = WorkspaceDesc::parse(self.desc)?;

        Ok(CreateWorkspaceParams {
            name: name.0,
            desc: desc.0,
        })
    }
}

// Read all workspaces if the workspace_id is None
#[derive(Clone, ProtoBuf, Default, Debug)]
pub struct WorkspaceIdPB {
    #[pb(index = 1, one_of)]
    pub value: Option<String>,
}

impl WorkspaceIdPB {
    pub fn new(workspace_id: Option<String>) -> Self {
        Self { value: workspace_id }
    }
}

#[derive(Default, ProtoBuf, Clone)]
pub struct WorkspaceSettingPB {
    #[pb(index = 1)]
    pub workspace: WorkspacePB,

    #[pb(index = 2, one_of)]
    pub latest_view: Option<ViewPB>,
}

#[derive(ProtoBuf, Default)]
pub struct UpdateWorkspacePayloadPB {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2, one_of)]
    pub name: Option<String>,

    #[pb(index = 3, one_of)]
    pub desc: Option<String>,
}

#[derive(Clone, Debug)]
pub struct UpdateWorkspaceParams {
    pub id: String,
    pub name: Option<String>,
    pub desc: Option<String>,
}

impl TryInto<UpdateWorkspaceParams> for UpdateWorkspacePayloadPB {
    type Error = ErrorCode;

    fn try_into(self) -> Result<UpdateWorkspaceParams, Self::Error> {
        let name = match self.name {
            None => None,
            Some(name) => Some(WorkspaceName::parse(name)?.0),
        };
        let id = WorkspaceIdentify::parse(self.id)?;

        Ok(UpdateWorkspaceParams {
            id: id.0,
            name,
            desc: self.desc,
        })
    }
}
