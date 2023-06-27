// ---------------------------------------------------------------------
// NTP protocol utilities
// ---------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// See LICENSE for details
// ---------------------------------------------------------------------
use std::fmt::Display;
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd)]
pub struct NtpTimeStamp(u64);

pub struct NtpDuration(u64);

const NTP_OFFSET: u64 = 2_208_988_800;
const NTP_SCALE: f64 = 4_294_967_295.0;
const MAX_NANOS: f64 = 1_000_000_000.0;

impl NtpTimeStamp {
    pub fn now() -> NtpTimeStamp {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before Unix epoch");
        NtpTimeStamp(
            ((now.as_secs() + NTP_OFFSET) << 32)
                + (now.subsec_nanos() as f64 * NTP_SCALE / MAX_NANOS) as u64,
        )
    }
    pub fn secs(&self) -> u32 {
        (self.0 >> 32) as u32
    }
    pub fn fracs(&self) -> u32 {
        (self.0 & 0xffffffff) as u32
    }
}

impl NtpDuration {
    pub fn num_nanoseconds(&self) -> u64 {
        (self.0 >> 32) * 1_000_000_000
            + ((((self.0 & 0xffffffff) as f64) * MAX_NANOS / NTP_SCALE) as u64)
    }
}

impl Sub for NtpTimeStamp {
    type Output = NtpDuration;

    fn sub(self, rhs: Self) -> Self::Output {
        NtpDuration(self.0 - rhs.0)
    }
}

impl Sub for NtpDuration {
    type Output = NtpDuration;

    fn sub(self, rhs: Self) -> Self::Output {
        NtpDuration(self.0 - rhs.0)
    }
}

impl From<u64> for NtpTimeStamp {
    fn from(value: u64) -> Self {
        NtpTimeStamp(value)
    }
}

impl From<NtpTimeStamp> for u64 {
    fn from(value: NtpTimeStamp) -> Self {
        value.0
    }
}

impl Display for NtpTimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::NtpTimeStamp;

    // fn get_utc_timestamp() -> UtcDateTime {
    //     Utc.timestamp_millis_opt(1613124000500).unwrap()
    // }

    fn get_ntp_timestamp() -> NtpTimeStamp {
        NtpTimeStamp::from((3822112800 << 32) + 2147483647)
    }

    #[test]
    fn test_secs() {
        let ts = get_ntp_timestamp();
        assert_eq!(ts.secs(), 3822112800);
    }
    #[test]
    fn test_fracs() {
        let ts = get_ntp_timestamp();
        assert_eq!(ts.fracs(), 2147483647);
    }
}
