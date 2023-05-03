// ---------------------------------------------------------------------
// G.107 E-Model calculations and constants.
// ---------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// See LICENSE for details
// ---------------------------------------------------------------------

// G.107 emodel codec data
pub struct CodecEModel {
    // Codec impairement, Ie
    ie: f32,
    // Packetization delay, ms
    pkt_delay: f32,
    // Packet-loss robustness factor
    bpl: f32,
}

// Commonly-used codecs
pub static G711_CODEC_EMODEL: CodecEModel = CodecEModel {
    ie: 0.0,
    pkt_delay: 0.0,
    bpl: 10.0,
};
pub static G729_CODEC_EMODEL: CodecEModel = CodecEModel {
    ie: 10.0,
    pkt_delay: 25.0,
    bpl: 17.0,
};

// Costants
const DEFAULT_R: f32 = 93.2;
const DEFAULT_BURST_R: f32 = 1.0;
//
impl CodecEModel {
    // packet_loss - in percents
    #[inline]
    pub fn get_r(&'static self, packet_loss: f32, avg_delay_ms: f32, jitter_ms: f32) -> f32 {
        // Effective equipment impairement factor
        let ie_eff =
            self.ie + (95.0 - self.ie) * packet_loss / ((packet_loss / DEFAULT_BURST_R) + self.bpl);
        let mut r = DEFAULT_R - ie_eff;
        // Effective latency
        let eff_latency = avg_delay_ms + self.pkt_delay + (jitter_ms * 2.0) + 10.0;
        r -= if eff_latency < 160.0 {
            eff_latency / 40.0
        } else {
            (eff_latency - 120.0) / 10.0
        };
        r - (packet_loss * 2.5)
    }
    // estimated MOS
    pub fn get_mos(&'static self, packet_loss: f32, avg_delay_ms: f32, jitter_ms: f32) -> f32 {
        let r = self.get_r(packet_loss, avg_delay_ms, jitter_ms);
        if r < 0.0 {
            return 1.0;
        }
        if r < 100.0 {
            return 1.0 + 0.035 * r + r * (r - 60.0) * (100.0 - r) * 7e-6;
        }
        4.5
    }
}
