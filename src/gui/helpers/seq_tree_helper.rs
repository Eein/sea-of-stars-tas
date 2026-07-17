//! Seq Tree tab: renders the route sequencer's whole node tree with the
//! currently executing chain highlighted, so you can see where the run is at
//! a glance. Phase 2 will make nodes clickable to start playback from them —
//! the child-index path rendered here is the jump target for that.

use std::collections::HashSet;

use egui::{Color32, RichText};
use seq::prelude::SeqTreeNode;

use super::GuiHelper;
use crate::{state::GameState, tas_runner::TasRunner};

pub const NAME: &str = "Seq Tree";

/// The tree snapshot walks every node and allocates its labels, so it's
/// rebuilt on a timer rather than every repaint (the GUI shares its thread
/// with the update loop, and timed inputs die when a frame runs long).
const REFRESH_SECONDS: f64 = 0.25;

/// The chain of currently executing nodes.
const ACTIVE: Color32 = Color32::from_rgb(255, 210, 90);
/// The deepest executing node — "you are here".
const CURRENT: Color32 = Color32::from_rgb(120, 220, 120);

/// The tree owns its headers' open state instead of leaving it to egui's
/// (session-persisted) memory: a node is open iff it's on the active chain
/// (while following) or the user clicked it open. So a new run starts fully
/// collapsed except the running chain, nodes close on their own once the run
/// moves past them, and anything opened by hand stays open until clicked
/// closed.
pub struct SeqTreeHelper {
    tree: Option<SeqTreeNode>,
    last_refresh: std::time::Instant,
    /// Keep the active chain's headers open, tracking the run.
    follow: bool,
    /// Hide moves (leaf nodes) that have already executed, so an open
    /// section shows only the current move onward.
    hide_completed: bool,
    /// Child-index paths the user opened by hand.
    manual_open: HashSet<Vec<usize>>,
    /// Whether a TAS was loaded last frame — a fresh one resets manual opens.
    had_runner: bool,
}

impl SeqTreeHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            tree: None,
            last_refresh: std::time::Instant::now(),
            follow: true,
            hide_completed: true,
            manual_open: HashSet::new(),
            had_runner: false,
        })
    }
}

/// Draw `node` and its subtree. The snapshot carries the live flags: `active`
/// nodes are the executing chain (the deepest one is the running node) and
/// `completed` sections have fully finished (rendered dimmed). A header click
/// toggles the node in `manual_open`; see [`SeqTreeHelper`] for the open-state
/// rules.
fn draw_node(
    ui: &mut egui::Ui,
    node: &SeqTreeNode,
    path: &mut Vec<usize>,
    follow: bool,
    hide_completed: bool,
    manual_open: &mut HashSet<Vec<usize>>,
) {
    let is_current = node.active && node.active_child.is_none();
    let label = if is_current {
        RichText::new(format!("▶ {}", node.label))
            .color(CURRENT)
            .strong()
    } else if node.active {
        RichText::new(&node.label).color(ACTIVE)
    } else if node.completed {
        RichText::new(&node.label).weak()
    } else {
        RichText::new(&node.label)
    };

    if node.children.is_empty() {
        ui.label(label);
        return;
    }
    let manually_open = manual_open.contains(path.as_slice());
    let open = manually_open || (follow && node.active);
    let response = egui::CollapsingHeader::new(label)
        .id_salt(path.last().copied().unwrap_or_default())
        .open(Some(open))
        .show(ui, |ui| {
            let hidden = node
                .children
                .iter()
                .filter(|c| hide_completed && c.completed && c.children.is_empty())
                .count();
            if hidden > 0 {
                ui.weak(format!("… {hidden} done"));
            }
            for (i, child) in node.children.iter().enumerate() {
                // Hide executed moves (leaves): an open section reads from
                // the current move onward. Completed *sections* stay listed
                // (collapsed and dimmed) so they remain reachable.
                if hide_completed && child.completed && child.children.is_empty() {
                    continue;
                }
                path.push(i);
                draw_node(ui, child, path, follow, hide_completed, manual_open);
                path.pop();
            }
        });
    if response.header_response.clicked() {
        if manually_open {
            manual_open.remove(path.as_slice());
        } else if !open {
            // Clicking a follow-forced-open header is ignored: follow keeps
            // the active chain open, and marking it manual would pin it open
            // after the run moves on.
            manual_open.insert(path.clone());
        }
    }
}

impl GuiHelper for SeqTreeHelper {
    fn draw(
        &mut self,
        _game_state: &mut GameState,
        tas_runner: &mut Option<TasRunner>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let Some(tas_runner) = tas_runner else {
            ui.label("No TAS loaded.");
            self.tree = None;
            self.had_runner = false;
            return;
        };
        if !self.had_runner {
            // A fresh run: collapse everything except the active chain.
            self.had_runner = true;
            self.manual_open.clear();
        }

        if self.tree.is_none() || self.last_refresh.elapsed().as_secs_f64() >= REFRESH_SECONDS {
            self.tree = Some(tas_runner.seq_tree());
            self.last_refresh = std::time::Instant::now();
        }

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.follow, "Follow the run");
            ui.checkbox(&mut self.hide_completed, "Hide completed moves");
        });
        ui.separator();
        if let Some(tree) = &self.tree {
            let manual_open = &mut self.manual_open;
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    draw_node(
                        ui,
                        tree,
                        &mut Vec::new(),
                        self.follow,
                        self.hide_completed,
                        manual_open,
                    );
                });
        }
    }
}
