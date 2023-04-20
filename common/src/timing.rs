// ---------------------------------------------------------------------
// Gufo Agent: Timing structure
// ---------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// ---------------------------------------------------------------------

/// Structure to calculate delay and jitter estimation for series of the tests.
///
/// Jitter is calculated via first-order estimator of variance D
/// with gain parameter `g` (1/16) as:
/// J[i] = (1 - g) * J[i - 1] + g * |D[i]|,
/// Where D[i] = |Delay[i] - Delay[i - 1]|
#[derive(Debug, Default)]
pub struct Timing {
    pub min_ns: i64,
    pub max_ns: i64,
    pub avg_ns: i64,
    pub jitter_ns: i64,
    count: i64,
    last_ns: i64,
    sum_ns: i64,
}

impl Timing {
    /// Register duration
    pub fn apply(&mut self, delta_ns: i64) {
        let diff_ns = if delta_ns > self.last_ns {
            delta_ns - self.last_ns
        } else {
            self.last_ns - delta_ns
        };
        // J = J + (|D| - J) / 16 ==>
        // J = J - J / 16 - |D| / 16
        self.jitter_ns = self.jitter_ns - (self.jitter_ns >> 4) + (diff_ns >> 4);
        if self.count == 0 {
            self.min_ns = delta_ns;
            self.max_ns = delta_ns;
        } else {
            if self.min_ns > delta_ns {
                self.min_ns = delta_ns
            }
            if self.max_ns < delta_ns {
                self.max_ns = delta_ns
            }
        }
        self.count += 1;
        self.last_ns = delta_ns;
        self.sum_ns += delta_ns;
    }
    /// Finish measurements
    pub fn done(&mut self) {
        if self.count > 0 {
            self.avg_ns = self.sum_ns / self.count;
        }
    }
}
