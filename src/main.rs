mod ram_doc;

use crate::ram_doc::backend::grab_processes;
use crate::ram_doc::utils::format_number;

fn main() {
    let fetched_sys_info = grab_processes();
    println!(
        "Total Memory: {} MBs, Used Memory: {} MBs, Total Swap: {} MBs, Used Swap: {} MBs",
        format_number(fetched_sys_info.total_memory),
        format_number(fetched_sys_info.used_memory),
        format_number(fetched_sys_info.total_swap),
        format_number(fetched_sys_info.used_swap)
    );
    for proc in fetched_sys_info.processes {
        println!(
            "PID: {}, Name: {}, Memory Usage: {} MBs, Path: {}",
            proc.pid,
            proc.name,
            format_number(proc.memory_usage),
            proc.path
        );
    }
    println!(
        "Process List Count: {}, Process List Memory Usage: {} MBs",
        format_number(fetched_sys_info.process_list_count),
        format_number(fetched_sys_info.process_list_memory_usage)
    );
}
