use colored::Colorize;
use std::collections::{HashMap, HashSet};

use gskits::{
    ds::ReadInfo,
    gsbam::read_bam,
    pbar::{get_spin_pb, DEFAULT_INTERVAL},
};
use rust_htslib::bam::{self, Read};

use crate::cli::BamDiffArgs;

#[derive(Debug, Default)]
struct StatCounts {
    in_a_not_in_b: usize,
    in_b_not_in_a: usize,
    seq_diff: usize,
    qual_diff: usize,
    a_tot: usize,
    b_tot: usize,
}

impl StatCounts {
    fn succ(&self) -> bool {
        self.a_tot == self.b_tot
            && self.in_a_not_in_b == 0
            && self.in_b_not_in_a == 0
            && self.seq_diff == 0
            && self.qual_diff == 0
    }
}

struct DiffStat {
    base_line_read_infos: HashMap<String, ReadInfo>,
    stat_counts: StatCounts,
}

impl DiffStat {
    fn new(base_line_read_infos: HashMap<String, ReadInfo>) -> Self {
        let a_tot = base_line_read_infos.len();
        let mut counts = StatCounts::default();
        counts.a_tot = a_tot;
        Self {
            base_line_read_infos,
            stat_counts: counts,
        }
    }

    fn check(&mut self, read_info: &ReadInfo) {
        self.stat_counts.b_tot += 1;
        if let Some(a_read_info) = self.base_line_read_infos.get(&read_info.name) {
            if a_read_info.seq != read_info.seq {
                self.stat_counts.seq_diff += 1;
            }

            if (a_read_info.rq.unwrap_or(0.) - read_info.rq.unwrap_or(0.)).abs() > 1e-3 {
                self.stat_counts.qual_diff += 1;
            }

            self.base_line_read_infos.remove(&read_info.name);
        } else {
            self.stat_counts.in_b_not_in_a += 1;
        }
    }

    fn finish(&mut self) -> &StatCounts {
        self.stat_counts.in_a_not_in_b = self.base_line_read_infos.len();
        &self.stat_counts
    }
}

pub fn bam_diff(bam_diff_args: &BamDiffArgs) {
    let a_bam = read_bam(&bam_diff_args.a_bam, Some(40));
    let a_bam = a_bam
        .into_iter()
        .map(|read_info| (read_info.name.clone(), read_info))
        .collect::<HashMap<_, _>>();

    let mut b_reader = bam::Reader::from_path(&bam_diff_args.b_bam).unwrap();
    b_reader.set_threads(40).unwrap();

    let pbar = get_spin_pb("diff checking".to_string(), DEFAULT_INTERVAL);

    let mut diff_stat = DiffStat::new(a_bam);

    for record in b_reader.records() {
        pbar.inc(1);
        let record = record.unwrap();
        let read_info = ReadInfo::from_bam_record(&record, None, &HashSet::new());
        diff_stat.check(&read_info);
    }
    pbar.finish();
    let stat_counts = diff_stat.finish();
    println!("");
    print!("BamDiffCheck: ");
    if stat_counts.succ() {
        println!("{}", "Successed".green());
    } else {
        println!("{}", "Failed".red());
    }
    
    println!("");
    println!("{:#?}", stat_counts);

    assert!(stat_counts.succ());
}
