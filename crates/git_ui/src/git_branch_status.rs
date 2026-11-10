use gpui::{App, Context, Entity, IntoElement, Render, Subscription, WeakEntity, Window};
use project::git_store::{GitStore, GitStoreEvent, RepositoryEvent};
use workspace::{StatusItemView, Workspace};

use crate::git_panel::GitPanel;

/// A status bar item that shows the current git branch and remote sync status
/// (Fetch/Push/Pull buttons) based on the active repository's upstream tracking.
pub struct GitBranchStatus {
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

impl GitBranchStatus {
    pub fn new(
        workspace: &Entity<Workspace>,
        git_store: &Entity<GitStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = Vec::new();
        subscriptions.push(cx.subscribe_in(
            &git_store,
            window,
            |_this, _git_store, event, _window, cx| {
                match event {
                    GitStoreEvent::RepositoryUpdated(
                        _,
                        RepositoryEvent::StatusesChanged | RepositoryEvent::HeadChanged,
                        true,
                    )
                    | GitStoreEvent::RepositoryAdded
                    | GitStoreEvent::RepositoryRemoved(_)
                    | GitStoreEvent::ActiveRepositoryChanged(_) => {
                        cx.notify();
                    }
                    _ => {}
                }
            },
        ));

        Self {
            workspace: workspace.downgrade(),
            _subscriptions: subscriptions,
        }
    }
}

impl Render for GitBranchStatus {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let Some(workspace) = self.workspace.upgrade() else {
            return gpui::div().into_any_element();
        };

        let Some(git_panel) = workspace.read(cx).panel::<GitPanel>(cx) else {
            return gpui::div().into_any_element();
        };

        git_panel
            .read(cx)
            .render_remote_button(cx)
            .unwrap_or_else(|| gpui::div().into_any_element())
    }
}

impl StatusItemView for GitBranchStatus {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn workspace::item::ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        None
    }
}
