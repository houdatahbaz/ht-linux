use sysinfo::{Disks, Networks, System};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
    pub file_system: String,
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub name: String,
    pub received: u64,
    pub transmitted: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub device_type: String,
    pub size: String,
    pub mountpoint: Option<String>,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub serial: Option<String>,
    pub state: Option<String>,
    pub subsystem: String,
}

#[derive(Debug, Clone)]
pub struct OverviewInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub uptime: u64,
    pub cpu_count: usize,
    pub total_memory: u64,
}

pub struct SystemData {
    sys: System,
    disks: Disks,
    networks: Networks,
    pub overview: OverviewInfo,
    pub cpus: Vec<CpuInfo>,
    pub memory: MemoryInfo,
    pub disk_list: Vec<DiskInfo>,
    pub network_list: Vec<NetworkInfo>,
    pub processes: Vec<ProcessInfo>,
    pub devices: Vec<DeviceInfo>,
    pub logs: Vec<String>,
}

impl SystemData {
    pub fn new() -> Self {
        let sys = System::new_all();
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        let mut data = Self {
            overview: OverviewInfo {
                hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
                os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
                os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
                kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
                uptime: System::uptime(),
                cpu_count: sys.cpus().len(),
                total_memory: sys.total_memory(),
            },
            cpus: Vec::new(),
            memory: MemoryInfo {
                total: 0,
                used: 0,
                available: 0,
                swap_total: 0,
                swap_used: 0,
            },
            disk_list: Vec::new(),
            network_list: Vec::new(),
            processes: Vec::new(),
            devices: Vec::new(),
            logs: Vec::new(),
            sys,
            disks,
            networks,
        };

        data.refresh();
        data
    }

    pub fn refresh(&mut self) {
        // Refresh system info
        self.sys.refresh_all();
        self.disks.refresh();
        self.networks.refresh();

        // Update overview
        self.overview.uptime = System::uptime();

        // Update CPU info
        self.cpus = self
            .sys
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| CpuInfo {
                name: format!("CPU {}", i),
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
            })
            .collect();

        // Update memory info
        self.memory = MemoryInfo {
            total: self.sys.total_memory(),
            used: self.sys.used_memory(),
            available: self.sys.available_memory(),
            swap_total: self.sys.total_swap(),
            swap_used: self.sys.used_swap(),
        };

        // Update disk info
        self.disk_list = self
            .disks
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total: disk.total_space(),
                available: disk.available_space(),
                file_system: disk.file_system().to_string_lossy().to_string(),
            })
            .collect();

        // Update network info
        self.network_list = self
            .networks
            .iter()
            .map(|(name, data)| NetworkInfo {
                name: name.clone(),
                received: data.total_received(),
                transmitted: data.total_transmitted(),
            })
            .collect();

        // Update process list
        self.processes = self
            .sys
            .processes()
            .iter()
            .map(|(pid, proc)| ProcessInfo {
                pid: pid.as_u32(),
                name: proc.name().to_string_lossy().to_string(),
                cpu_usage: proc.cpu_usage(),
                memory: proc.memory(),
                status: format!("{:?}", proc.status()),
            })
            .collect();

        // Sort processes by CPU usage (descending)
        self.processes
            .sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));

        // Update devices
        self.refresh_devices();

        // Update logs (last 50 lines from dmesg)
        self.refresh_logs();
    }

    fn refresh_devices(&mut self) {
        self.devices.clear();

        // Get block devices using lsblk
        if let Ok(output) = Command::new("lsblk")
            .args(["-J", "-o", "NAME,TYPE,SIZE,MOUNTPOINT,MODEL,VENDOR,SERIAL,STATE"])
            .output()
        {
            if let Ok(json_str) = String::from_utf8(output.stdout) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    if let Some(devices) = json["blockdevices"].as_array() {
                        for dev in devices {
                            self.parse_device(dev, "block");
                        }
                    }
                }
            }
        }

        // Get input devices
        if let Ok(entries) = std::fs::read_dir("/sys/class/input") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("event") || name.starts_with("mouse") || name.starts_with("js") {
                    let device_name = std::fs::read_to_string(entry.path().join("device/name"))
                        .unwrap_or_else(|_| name.clone())
                        .trim()
                        .to_string();

                    self.devices.push(DeviceInfo {
                        name: device_name,
                        device_type: "input".to_string(),
                        size: "-".to_string(),
                        mountpoint: None,
                        model: None,
                        vendor: None,
                        serial: None,
                        state: Some("active".to_string()),
                        subsystem: "input".to_string(),
                    });
                }
            }
        }

        // Get USB devices
        if let Ok(output) = Command::new("lsusb").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                // Parse: Bus 001 Device 002: ID 1234:5678 Device Name
                if let Some(name_start) = line.find(": ID ") {
                    let after_id = &line[name_start + 5..];
                    if let Some(space_idx) = after_id.find(' ') {
                        let device_name = after_id[space_idx + 1..].to_string();
                        let id = after_id[..space_idx].to_string();

                        self.devices.push(DeviceInfo {
                            name: device_name,
                            device_type: "usb".to_string(),
                            size: "-".to_string(),
                            mountpoint: None,
                            model: None,
                            vendor: Some(id),
                            serial: None,
                            state: Some("connected".to_string()),
                            subsystem: "usb".to_string(),
                        });
                    }
                }
            }
        }

        // Get PCI devices (graphics, network, etc.)
        if let Ok(output) = Command::new("lspci").args(["-mm"]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().take(20) {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 6 {
                    let device_class = parts.get(1).unwrap_or(&"").to_string();
                    let vendor = parts.get(3).unwrap_or(&"").to_string();
                    let device_name = parts.get(5).unwrap_or(&"").to_string();

                    self.devices.push(DeviceInfo {
                        name: device_name,
                        device_type: device_class,
                        size: "-".to_string(),
                        mountpoint: None,
                        model: None,
                        vendor: Some(vendor),
                        serial: None,
                        state: None,
                        subsystem: "pci".to_string(),
                    });
                }
            }
        }
    }

    fn parse_device(&mut self, dev: &serde_json::Value, subsystem: &str) {
        let name = dev["name"].as_str().unwrap_or("unknown").to_string();
        let device_type = dev["type"].as_str().unwrap_or("unknown").to_string();
        let size = dev["size"].as_str().unwrap_or("-").to_string();
        let mountpoint = dev["mountpoint"].as_str().map(|s| s.to_string());
        let model = dev["model"].as_str().map(|s| s.trim().to_string());
        let vendor = dev["vendor"].as_str().map(|s| s.trim().to_string());
        let serial = dev["serial"].as_str().map(|s| s.to_string());
        let state = dev["state"].as_str().map(|s| s.to_string());

        self.devices.push(DeviceInfo {
            name,
            device_type,
            size,
            mountpoint,
            model,
            vendor,
            serial,
            state,
            subsystem: subsystem.to_string(),
        });

        // Parse children (partitions)
        if let Some(children) = dev["children"].as_array() {
            for child in children {
                self.parse_device(child, subsystem);
            }
        }
    }

    fn refresh_logs(&mut self) {
        self.logs.clear();

        // Try journalctl first (usually works without sudo)
        if let Ok(output) = Command::new("journalctl")
            .args(["--no-pager", "-n", "100", "--output=short"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() && output.status.success() {
                self.logs = stdout
                    .lines()
                    .map(|s| s.to_string())
                    .collect();
                return;
            }
        }

        // Try dmesg with timestamp
        if let Ok(output) = Command::new("dmesg")
            .args(["-T", "--time-format=reltime"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                self.logs = stdout
                    .lines()
                    .rev()
                    .take(100)
                    .map(|s| s.to_string())
                    .collect();
                self.logs.reverse();
                return;
            }
        }

        // Try plain dmesg
        if let Ok(output) = Command::new("dmesg").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                self.logs = stdout
                    .lines()
                    .rev()
                    .take(100)
                    .map(|s| s.to_string())
                    .collect();
                self.logs.reverse();
                return;
            }
        }

        // Try reading /var/log/syslog or /var/log/messages
        for log_path in ["/var/log/syslog", "/var/log/messages"] {
            if let Ok(content) = std::fs::read_to_string(log_path) {
                self.logs = content
                    .lines()
                    .rev()
                    .take(100)
                    .map(|s| s.to_string())
                    .collect();
                self.logs.reverse();
                return;
            }
        }

        // Nothing worked
        self.logs = vec![
            "Unable to read system logs.".to_string(),
            "".to_string(),
            "Try one of:".to_string(),
            "  - Run with sudo: sudo cargo run".to_string(),
            "  - Add user to systemd-journal group:".to_string(),
            "    sudo usermod -aG systemd-journal $USER".to_string(),
        ];
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, secs)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
