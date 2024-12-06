use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reports: Vec<Vec<i32>> = Vec::new();
    populate_reports(&mut reports, &args);
    dbg!(count_safe_reports(&reports));
}

fn populate_reports( reports: &mut Vec<Vec<i32>>, args: &Vec<String> )
{
    let file_path = &args[1];
    let file = File::open(file_path).expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();

    while buf_reader.read_line(&mut line).is_ok() && !line.is_empty()
    {
        let mut report: Vec<i32> = Vec::new();
        let levels = line.split(" ");
        for level in levels {
            let trimmed_level = level.trim();
            report.push(trimmed_level.parse().expect("Levels are expected to be i32s"));
        }
        reports.push(report);
        line = String::new();
    }
}

fn count_safe_reports( reports: &Vec<Vec<i32>> ) -> u32
{
    let mut count_safe: u32 = 0;
    let first_time: bool = true;
    for report in reports {
        // No need to check empty or single level reports
        if report.len() < 2 {
            count_safe += 1;
            continue;
        }

        if check_report_safety(report, first_time) {
            count_safe += 1;
        }
    }
    count_safe
}

fn check_report_safety( report: &Vec<i32>, first_time: bool) -> bool
{
    let expected_order_check: fn(i32, i32) -> bool;
    if report[0] < report[1] {
        expected_order_check = |i1: i32, i2: i32| -> bool { i1 < i2 };
    }
    else {
        expected_order_check = |i1: i32, i2: i32| -> bool { i1 > i2 };
    }

    let final_index_to_check_upperbound = report.len() - 1;
    for i in 0..final_index_to_check_upperbound {
        if !expected_order_check(report[i], report[i + 1]) || i32::abs(report[i] - report[i + 1]) > 3 {
            if first_time {
                let mut report_without_i1 = report.clone();
                let mut report_without_i2 = report.clone();
                report_without_i1.remove(i);
                report_without_i2.remove(i + 1);
                // Handle edge case where ordering may be wrong because first element is unsafe
                let mut first_ele_is_unsafe = false;
                if i == 1
                {
                    let mut report_without_first = report.clone();    
                    report_without_first.remove(0);
                    first_ele_is_unsafe = check_report_safety(&report_without_first, false);
                }
                return check_report_safety(&report_without_i2, false) || check_report_safety(&report_without_i1, false) || first_ele_is_unsafe;
            } 
            return false;
        }
    }
    true
}
 
