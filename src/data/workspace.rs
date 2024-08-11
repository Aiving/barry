use hyprland::{
    data::{Workspace as HyprlandWorkspace, Workspaces},
    shared::{HyprData, HyprDataActive, HyprDataVec},
};

#[derive(Clone, PartialEq, Eq)]
pub enum WorkspacePosition {
    First,
    Between,
    Both,
    Last,
}

#[derive(Clone)]
pub struct Workspace {
    pub id: usize,
    pub position: WorkspacePosition,
    pub exists: bool,
    pub active: bool,
}

impl Workspace {
    #[must_use]
    pub fn all() -> Vec<Self> {
        let workspaces = Workspaces::get()
            .map(HyprDataVec::to_vec)
            .unwrap_or_default();
        let active_workspace = HyprlandWorkspace::get_active().ok();

        let mut all = (1..=10)
            .map(|id| Self {
                id,
                position: WorkspacePosition::Between,
                exists: workspaces.iter().any(|workspace| workspace.id == id as i32),
                active: active_workspace
                    .as_ref()
                    .is_some_and(|workspace| workspace.id == id as i32),
            })
            .collect::<Vec<_>>();

        all.chunk_by_mut(|a, b| a.exists == b.exists)
            .map(|workspaces| {
                if let Some(first) = workspaces.first_mut() {
                    first.position = WorkspacePosition::First;
                }

                if let Some(last) = workspaces.last_mut() {
                    if last.position == WorkspacePosition::First {
                        last.position = WorkspacePosition::Both;
                    } else {
                        last.position = WorkspacePosition::Last;
                    }
                }

                workspaces
            })
            .collect::<Vec<_>>()
            .concat()
    }
}
