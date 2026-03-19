//! Tzinor Phase-Locked Loop (T-PLL)
//!
//! Synchronizes VerCore internal clock with NavIC atomic clocks.
//! Provides a unified "Arkhe Time" for the entire system.

use std::time::{Duration, Instant};

/// Configuration for the Tzinor PLL
pub struct TzinorPllConfig {
    /// Proportional gain (how fast we correct phase)
    pub kp: f64,
    /// Integral gain (how much we correct accumulated drift)
    pub ki: f64,
    /// Derivative gain (damping)
    pub kd: f64,
}

/// The Tzinor PLL Engine
pub struct TzinorPll {
    /// Local time estimate (VerCore clock)
    pub local_time: Instant,

    /// Last received NavIC 1PPS timestamp
    pub last_pps: Option<Instant>,

    /// Phase error accumulator
    pub phase_error: f64,

    /// Frequency adjustment (ppm)
    pub freq_adjustment: f64,

    /// The ontological anchor (fallback simulation)
    pub xi_anchor: XiCCPlusOrb,

    /// Config
    pub config: TzinorPllConfig,
}

pub struct XiCCPlusOrb;
impl XiCCPlusOrb {
    pub fn simulated_decay_ticks(&self) -> u64 { 0 }
}

/// Unified Arkhe Time
#[derive(Debug, Clone)]
pub struct ArkheTime {
    /// Unix timestamp (nanoseconds)
    pub unix_ns: u64,

    /// Coherence score (0.0 - 1.0)
    pub coherence: f64,

    /// Source of time (NavIC, XiCore, Fallback)
    pub source: TimeSource,
}

#[derive(Debug, Clone)]
pub enum TimeSource {
    NavicAtomic,
    XiCoreOntological,
    VerCoreFreeRun, // Unreliable
}

impl TzinorPll {
    pub fn new(xi_anchor: XiCCPlusOrb) -> Self {
        Self {
            local_time: Instant::now(),
            last_pps: None,
            phase_error: 0.0,
            freq_adjustment: 0.0,
            xi_anchor,
            config: TzinorPllConfig {
                kp: 0.1,
                ki: 0.01,
                kd: 0.001,
            },
        }
    }

    /// Input: NavIC 1PPS signal received via SDR
    pub fn feed_pps(&mut self, pps_timestamp: Instant) {
        if let Some(last) = self.last_pps {
            // Calculate expected interval (should be exactly 1 second)
            let measured_interval = pps_timestamp.duration_since(last);

            // Phase Error = Measured - Expected
            let expected_ns = 1_000_000_000.0;
            let measured_ns = measured_interval.as_nanos() as f64;
            let error_ns = measured_ns - expected_ns;

            // Update phase error
            self.phase_error = error_ns;

            // Apply PID control to adjust VerCore frequency
            let correction = self.config.kp * error_ns;

            // Apply correction
            self.freq_adjustment += correction;

            // Update local time to align with NavIC
            self.local_time = pps_timestamp;
        }

        self.last_pps = Some(pps_timestamp);
    }

    /// Get the current Unified Arkhe Time
    pub fn get_time(&self) -> ArkheTime {
        // Check if NavIC signal is recent (within 2 seconds)
        let navic_valid = self.last_pps.map_or(false, |pps| {
            pps.elapsed() < Duration::from_secs(2)
        });

        if navic_valid {
            // Synchronized mode: Use NavIC time + local correction
            let base_ns = self.last_pps.unwrap().elapsed().as_nanos() as u64;

            ArkheTime {
                unix_ns: base_ns,
                coherence: 1.0,
                source: TimeSource::NavicAtomic,
            }
        } else {
            // Fallback mode: Use Ξcc⁺ ontological clock
            let xi_time = self.xi_anchor.simulated_decay_ticks();

            ArkheTime {
                unix_ns: xi_time,
                coherence: 0.7,
                source: TimeSource::XiCoreOntological,
            }
        }
    }

    /// Monitor for drift and report to Entropy Shield
    pub fn check_drift(&self) -> DriftReport {
        DriftReport {
            phase_error_ns: self.phase_error,
            freq_correction_ppm: self.freq_adjustment,
            source: if self.last_pps.is_some() { "NavIC" } else { "XiCore" },
        }
    }
}

#[derive(Debug)]
pub struct DriftReport {
    pub phase_error_ns: f64,
    pub freq_correction_ppm: f64,
    pub source: &'static str,
}
