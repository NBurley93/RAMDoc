pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_usage: u64,
    pub path: String,
}

pub struct SystemInfo {
    pub processes: Vec<ProcessInfo>,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub process_list_count: u64,
    pub process_list_memory_usage: u64,
}
