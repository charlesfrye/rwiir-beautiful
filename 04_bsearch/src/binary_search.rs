use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct BinarySearch {
    keys: Vec<String>,
    values: Vec<Vec<(String, &'static str)>>,
    data: HashMap<String, Vec<(String, &'static str)>>,
}

impl BinarySearch {
    pub fn new(file: &str, size: usize) -> Result<Self, std::io::Error> {
        let mut keys = Vec::with_capacity(size);
        let mut values = Vec::with_capacity(size);

        let file = File::open(file)?;
        let reader = BufReader::new(file);

        let mut last = String::new();
        let mut hosts = -1;

        for line in reader.lines() {
            let line = line?;
            let s: Vec<&str> = line.split_whitespace().collect();

            let article = s[2].intern();

            if s[0] != last {
                hosts += 1;
                keys.push(s[0].to_string());
                values.push(Vec::new());
                last = s[0].to_string();
            }

            values[hosts].push((s[1].to_string(), article));
        }

        Ok(BinarySearch { keys, values })
    }

    pub fn find(&self, target: &str) -> Option<&Vec<(String, &'static str)>> {
        let mut low = -1;
        let mut high = self.keys.len() as isize;

        while (high - low) > 1 {
            let probe = (high + low) / 2;
            let probe_index = probe as usize;

            if self.keys[probe_index] > target {
                high = probe;
            } else {
                low = probe;
            }
        }

        let low_index = low as usize;
        if low_index == -1 || self.keys[low_index] != target {
            None
        } else {
            Some(&self.values[low_index])
        }
    }
}
