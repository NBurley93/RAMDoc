use std::path::Path;
use std::vec::Vec;
use sysinfo::System;

use crate::ram_doc::models::{ProcessInfo, SystemInfo};
use crate::ram_doc::utils::bytes_to_megabytes;

fn add_process_info(pid: u32, name: String, mem: u64, path: String) -> ProcessInfo {
    return ProcessInfo {
        pid: pid,
        name: name,
        memory_usage: bytes_to_megabytes(mem),
        path: path,
    };
}

fn build_process_list(sys: &System) -> Vec<ProcessInfo> {
    let mut plist: Vec<ProcessInfo> = Vec::new();
    for (pid, process) in sys.processes() {
        let virtual_mem: u64 = process.memory();
        let proc_exe_opt: Option<&Path> = process.exe();
        if proc_exe_opt.is_none() {
            continue;
        }
        let proc_exe: &Path = proc_exe_opt.unwrap();

        let proc_name: String = proc_exe.file_name().unwrap().to_str().unwrap().to_string();
        let proc_path: String = proc_exe.to_str().unwrap().to_string();

        plist.push(add_process_info(
            pid.as_u32(),
            proc_name,
            virtual_mem,
            proc_path,
        ));
    }

    // Filter out any item within the Windows directory, as those are likely system processes that
    // we don't care about
    plist.retain(|proc| !proc.path.to_lowercase().contains("windows"));

    // Since each thread is a separate process, we want to represent ALL of them, so we'll compress
    // them together into one item per process name, and sum their memory usage together
    plist = plist.into_iter().fold(Vec::new(), |mut acc, proc| {
        if let Some(existing_proc) = acc.iter_mut().find(|p| p.name == proc.name) {
            existing_proc.memory_usage += proc.memory_usage;
        } else {
            acc.push(proc);
        }
        acc
    });

    // Now, we sort the list by memory usage, highest first
    plist.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));

    return plist;
}

fn build_system_info(
    processes: Vec<ProcessInfo>,
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
) -> SystemInfo {
    let list_count = processes.len() as u64;
    let list_memory_usage = processes.iter().map(|p| p.memory_usage).sum();

    return SystemInfo {
        processes,
        total_memory: bytes_to_megabytes(total_memory),
        used_memory: bytes_to_megabytes(used_memory),
        total_swap: bytes_to_megabytes(total_swap),
        used_swap: bytes_to_megabytes(used_swap),
        process_list_count: list_count,
        process_list_memory_usage: list_memory_usage,
    };
}

pub fn grab_processes() -> SystemInfo {
    // First we gotta make our sysinfo calls
    let mut sys = System::new_all();
    sys.refresh_all();

    let si = build_system_info(
        build_process_list(&sys),
        sys.total_memory(),
        sys.used_memory(),
        sys.total_swap(),
        sys.used_swap(),
    );

    return si;
}
