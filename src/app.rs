use crate::events::Event;
use crate::system::SystemData;
use crate::ui;
use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeNode {
    Overview,
    Cpu,
    Memory,
    Disks,
    Network,
    Processes,
    Devices,
    Logs,
}

impl TreeNode {
    pub fn all() -> Vec<TreeNode> {
        vec![
            TreeNode::Overview,
            TreeNode::Cpu,
            TreeNode::Memory,
            TreeNode::Disks,
            TreeNode::Network,
            TreeNode::Processes,
            TreeNode::Devices,
            TreeNode::Logs,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            TreeNode::Overview => "Overview",
            TreeNode::Cpu => "CPU",
            TreeNode::Memory => "Memory",
            TreeNode::Disks => "Disks",
            TreeNode::Network => "Network",
            TreeNode::Processes => "Processes",
            TreeNode::Devices => "Devices",
            TreeNode::Logs => "Logs",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            TreeNode::Overview => "[*]",
            TreeNode::Cpu => "[C]",
            TreeNode::Memory => "[M]",
            TreeNode::Disks => "[D]",
            TreeNode::Network => "[N]",
            TreeNode::Processes => "[P]",
            TreeNode::Devices => "[V]",
            TreeNode::Logs => "[L]",
        }
    }

    pub fn shortcut_key(&self) -> char {
        match self {
            TreeNode::Overview => '*',
            TreeNode::Cpu => 'c',
            TreeNode::Memory => 'm',
            TreeNode::Disks => 'd',
            TreeNode::Network => 'n',
            TreeNode::Processes => 'p',
            TreeNode::Devices => 'v',
            TreeNode::Logs => 'l',
        }
    }

    pub fn from_shortcut(key: char) -> Option<TreeNode> {
        match key.to_ascii_lowercase() {
            '*' => Some(TreeNode::Overview),
            'c' => Some(TreeNode::Cpu),
            'm' => Some(TreeNode::Memory),
            'd' => Some(TreeNode::Disks),
            'n' => Some(TreeNode::Network),
            'p' => Some(TreeNode::Processes),
            'v' => Some(TreeNode::Devices),
            'l' => Some(TreeNode::Logs),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub node: TreeNode,
    pub scroll_offset: u16,
    pub selected_item: usize,
}

impl Tab {
    pub fn new(node: TreeNode) -> Self {
        Self {
            node,
            scroll_offset: 0,
            selected_item: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Tree,
    Tabs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Command,
}

pub struct App {
    pub running: bool,
    pub focus: Focus,
    pub mode: Mode,
    pub tree_nodes: Vec<TreeNode>,
    pub selected_tree_index: usize,
    pub tabs: Vec<Tab>,
    pub active_tab_index: usize,
    pub system_data: SystemData,
    pub show_help: bool,
    pub command_buffer: String,
    pub command_error: Option<String>,
    pub show_device_popup: bool,
    pub selected_device_index: Option<usize>,
    pub show_kill_confirm: bool,
    pub kill_target_pid: Option<u32>,
    pub kill_target_name: Option<String>,
    pub status_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            focus: Focus::Tree,
            mode: Mode::Normal,
            tree_nodes: TreeNode::all(),
            selected_tree_index: 0,
            tabs: vec![Tab::new(TreeNode::Overview)],
            active_tab_index: 0,
            system_data: SystemData::new(),
            show_help: false,
            command_buffer: String::new(),
            command_error: None,
            show_device_popup: false,
            selected_device_index: None,
            show_kill_confirm: false,
            kill_target_pid: None,
            kill_target_name: None,
            status_message: None,
        }
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<Event>(100);

        // Spawn system data refresh task
        let tx_refresh = tx.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(2));
            loop {
                interval.tick().await;
                if tx_refresh.send(Event::Refresh).await.is_err() {
                    break;
                }
            }
        });

        // Spawn input event task
        let tx_input = tx.clone();
        tokio::spawn(async move {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                    if let Ok(evt) = event::read() {
                        if tx_input.send(Event::Input(evt)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        // Initial data load
        self.system_data.refresh();

        while self.running {
            terminal.draw(|f| ui::draw(f, self))?;

            if let Some(event) = rx.recv().await {
                match event {
                    Event::Input(evt) => self.handle_input(evt),
                    Event::Refresh => self.system_data.refresh(),
                }
            }
        }

        Ok(())
    }

    fn handle_input(&mut self, event: CrosstermEvent) {
        if let CrosstermEvent::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return;
            }

            // Handle based on current mode
            match self.mode {
                Mode::Command => self.handle_command_input(key.code),
                Mode::Normal => self.handle_normal_input(key.code),
            }
        }
    }

    fn handle_command_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.command_buffer.clear();
                self.command_error = None;
            }
            KeyCode::Enter => {
                self.execute_command();
            }
            KeyCode::Backspace => {
                self.command_buffer.pop();
                if self.command_buffer.is_empty() {
                    self.mode = Mode::Normal;
                }
            }
            KeyCode::Char(c) => {
                self.command_buffer.push(c);
                self.command_error = None;
            }
            _ => {}
        }
    }

    fn execute_command(&mut self) {
        let cmd = self.command_buffer.trim();
        match cmd {
            "q" | "quit" => {
                self.running = false;
            }
            "q!" | "quit!" => {
                self.running = false;
            }
            "help" | "h" => {
                self.show_help = true;
                self.mode = Mode::Normal;
                self.command_buffer.clear();
            }
            _ => {
                self.command_error = Some(format!("Unknown command: {}", cmd));
            }
        }
        if self.running && self.command_error.is_none() {
            self.command_buffer.clear();
            self.mode = Mode::Normal;
        }
    }

    fn handle_normal_input(&mut self, key: KeyCode) {
        // Clear status message on any key
        self.status_message = None;

        // Handle kill confirmation popup
        if self.show_kill_confirm {
            match key {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.execute_kill();
                    return;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.show_kill_confirm = false;
                    self.kill_target_pid = None;
                    self.kill_target_name = None;
                    return;
                }
                _ => return,
            }
        }

        // Close device popup if open
        if self.show_device_popup {
            match key {
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                    self.show_device_popup = false;
                    self.selected_device_index = None;
                    return;
                }
                _ => return,
            }
        }

        // Global keys
        match key {
            KeyCode::Char(':') => {
                self.mode = Mode::Command;
                self.command_buffer.clear();
                self.command_error = None;
                return;
            }
            KeyCode::Char('?') => {
                self.show_help = !self.show_help;
                return;
            }
            KeyCode::Esc if self.show_help => {
                self.show_help = false;
                return;
            }
            KeyCode::Tab => {
                self.focus = match self.focus {
                    Focus::Tree => Focus::Tabs,
                    Focus::Tabs => Focus::Tree,
                };
                return;
            }
            _ => {}
        }

        if self.show_help {
            return;
        }

        // Handle shortcut keys to open tabs directly (only when focused on tree)
        if self.focus == Focus::Tree {
            if let KeyCode::Char(c) = key {
                if let Some(node) = TreeNode::from_shortcut(c) {
                    self.open_tab_by_node(node);
                    return;
                }
            }
        }

        match self.focus {
            Focus::Tree => self.handle_tree_input(key),
            Focus::Tabs => self.handle_tabs_input(key),
        }
    }

    fn handle_tree_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_tree_index > 0 {
                    self.selected_tree_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_tree_index < self.tree_nodes.len() - 1 {
                    self.selected_tree_index += 1;
                }
            }
            KeyCode::Enter | KeyCode::Right => {
                self.open_or_switch_tab();
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let idx = c.to_digit(10).unwrap_or(0) as usize;
                if idx > 0 && idx <= self.tabs.len() {
                    self.active_tab_index = idx - 1;
                    self.focus = Focus::Tabs;
                }
            }
            _ => {}
        }
    }

    fn handle_tabs_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left | KeyCode::Char('h') => {
                if self.active_tab_index > 0 {
                    self.active_tab_index -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.active_tab_index < self.tabs.len().saturating_sub(1) {
                    self.active_tab_index += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab_index) {
                    if tab.selected_item > 0 {
                        tab.selected_item -= 1;
                    }
                    if tab.selected_item < tab.scroll_offset as usize {
                        tab.scroll_offset = tab.selected_item as u16;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab_index) {
                    tab.selected_item += 1;
                    tab.scroll_offset = tab.scroll_offset.saturating_add(0);
                }
            }
            KeyCode::Enter => {
                // Open device popup if in Devices tab
                if let Some(tab) = self.active_tab() {
                    if tab.node == TreeNode::Devices {
                        let device_count = self.system_data.devices.len();
                        if device_count > 0 && tab.selected_item < device_count {
                            self.selected_device_index = Some(tab.selected_item);
                            self.show_device_popup = true;
                        }
                    }
                }
            }
            KeyCode::Char('x') | KeyCode::Char('X') | KeyCode::Delete => {
                // Kill process if in Processes tab
                if let Some(tab) = self.active_tab() {
                    if tab.node == TreeNode::Processes {
                        let process_count = self.system_data.processes.len();
                        if process_count > 0 && tab.selected_item < process_count {
                            let proc = &self.system_data.processes[tab.selected_item];
                            self.kill_target_pid = Some(proc.pid);
                            self.kill_target_name = Some(proc.name.clone());
                            self.show_kill_confirm = true;
                        }
                    }
                }
            }
            KeyCode::Char('w') => {
                self.close_current_tab();
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let idx = c.to_digit(10).unwrap_or(0) as usize;
                if idx > 0 && idx <= self.tabs.len() {
                    self.active_tab_index = idx - 1;
                }
            }
            _ => {}
        }
    }

    fn open_or_switch_tab(&mut self) {
        let node = self.tree_nodes[self.selected_tree_index];
        self.open_tab_by_node(node);
    }

    fn open_tab_by_node(&mut self, node: TreeNode) {
        // Check if tab already exists
        if let Some(idx) = self.tabs.iter().position(|t| t.node == node) {
            self.active_tab_index = idx;
        } else {
            // Create new tab
            self.tabs.push(Tab::new(node));
            self.active_tab_index = self.tabs.len() - 1;
        }

        // Update tree selection to match
        if let Some(idx) = self.tree_nodes.iter().position(|&n| n == node) {
            self.selected_tree_index = idx;
        }

        self.focus = Focus::Tabs;
    }

    fn close_current_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.tabs.remove(self.active_tab_index);
            if self.active_tab_index >= self.tabs.len() {
                self.active_tab_index = self.tabs.len() - 1;
            }
        }
    }

    fn execute_kill(&mut self) {
        if let Some(pid) = self.kill_target_pid {
            let name = self.kill_target_name.clone().unwrap_or_default();

            // Try to kill the process using kill command
            let result = std::process::Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        self.status_message = Some(format!("Killed process {} (PID: {})", name, pid));
                        // Refresh process list
                        self.system_data.refresh();
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        self.status_message = Some(format!("Failed to kill {}: {}", name, stderr.trim()));
                    }
                }
                Err(e) => {
                    self.status_message = Some(format!("Error killing {}: {}", name, e));
                }
            }
        }

        self.show_kill_confirm = false;
        self.kill_target_pid = None;
        self.kill_target_name = None;
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab_index)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_tab_index)
    }
}
