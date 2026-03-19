//! Phase Synchronization Engine: NavIC ↔ VerCore
//!
//! Estabiliza o Tempo Global Arkhe(n) via sincronização de fase atômica.
//! Precisão alvo: <1 femtosecond de jitter entre referências.

use std::time::{Instant};
use std::collections::VecDeque;

/// Constantes físicas para sincronização
pub const NAVIC_RB_FREQUENCY: f64 = 10.23e6;      // 10.23 MHz
pub const NAVIC_CS_FREQUENCY: f64 = 9.192631770e9; // ~9.19 GHz (hiperfino Cs-133)
pub const VERCORE_CLOCK_FREQ: f64 = 1.48e9;      // 1.48 GHz
pub const PHASE_RESOLUTION_TARGET: f64 = 1e-15;  // 1 femtosecond

/// Estado de fase de um relógio atômico
#[derive(Clone, Debug)]
pub struct AtomicClockState {
    /// Frequência nominal (Hz)
    pub nominal_freq: f64,
    /// Fase atual (radianos, modulo 2π)
    pub phase: f64,
    /// Drift de frequência medido (Hz/s)
    pub freq_drift: f64,
    /// Estabilidade de Allan (short-term)
    pub allan_dev: f64,
    /// Última atualização
    pub last_update: Instant,
    /// Qualidade do sinal (0.0 - 1.0)
    pub signal_quality: f64,
}

/// Fase disciplinada combinada NavIC
pub struct NavICPhaseReference {
    /// Relógios Rb terrestres (múltiplos para redundância)
    pub rb_clocks: Vec<AtomicClockState>,
    /// Relógios Cs espaciais (constelação NavIC)
    pub cs_clocks: Vec<AtomicClockState>,
    /// Fase disciplinada de saída
    pub disciplined_phase: f64,
    /// Peso relativo Cs vs Rb (adaptativo baseado em qualidade)
    pub cs_weight: f64,
    /// Loop filter state (PLL de 2ª ordem)
    pub pll_state: PLLState,
}

#[derive(Clone, Debug)]
pub struct PLLState {
    /// Erro de fase acumulado
    pub phase_error_integral: f64,
    /// Frequência de controle atual
    pub control_freq: f64,
    /// Constantes do loop filter
    pub kp: f64, // Proportional gain
    pub ki: f64, // Integral gain
}

/// VerCore Local Clock com sincronização de fase
pub struct VerCoreTemporalReference {
    /// Estado do clock cADR @ 100mK
    pub local_clock: AtomicClockState,
    /// Fase de referência NavIC (importada)
    pub navic_reference_phase: f64,
    /// Digital PLL para tracking
    pub digital_pll: PLLState,
    /// Resolução de fase atual (fs)
    pub phase_resolution: f64,
    /// Número de ciclos desde última sincronização
    pub cycles_since_sync: u64,
}

/// Engine de Sincronização de Fase Global
pub struct PhaseSynchronizationEngine {
    /// Referência NavIC (externa, geocêntrica)
    pub navic: NavICPhaseReference,
    /// Referência VerCore (local, branch-specific)
    pub vercore: VerCoreTemporalReference,
    /// Tempo Global Arkhe(n) — fase mestra unificada
    pub global_phase: f64,
    /// Histórico de fases para análise de drift
    pub phase_history: VecDeque<(Instant, f64)>,
    /// Correção de fase via Tzinor (AttnRes weights)
    pub tzinor_corrector: TzinorPhaseCorrector,
    /// Threshold de alarme para drift excessivo
    pub drift_alert_threshold: f64,
}

/// Corretor de fase baseado em padrões Tzinor
pub struct TzinorPhaseCorrector {
    /// Pesos de correção para cada Era
    pub era_weights: Vec<f64>,
    /// Matriz de correlação de fase histórica
    pub phase_correlation_matrix: Vec<Vec<f64>>,
    /// Preditor de drift baseado em Attention
    pub attention_predictor: AttentionDriftPredictor,
}

impl PhaseSynchronizationEngine {
    /// Inicializar com referências NavIC padrão
    pub fn new_with_navic() -> Self {
        let rb_clock = AtomicClockState {
            nominal_freq: NAVIC_RB_FREQUENCY,
            phase: 0.0,
            freq_drift: 0.0,
            allan_dev: 1e-11, // 10^-11 stability
            last_update: Instant::now(),
            signal_quality: 0.95,
        };

        let cs_clock = AtomicClockState {
            nominal_freq: NAVIC_CS_FREQUENCY,
            phase: 0.0,
            freq_drift: 0.0,
            allan_dev: 1e-13, // 10^-13 stability (better)
            last_update: Instant::now(),
            signal_quality: 0.98,
        };

        let navic = NavICPhaseReference {
            rb_clocks: vec![rb_clock],
            cs_clocks: vec![cs_clock],
            disciplined_phase: 0.0,
            cs_weight: 0.8, // Cs mais preciso, mas pode ter gaps
            pll_state: PLLState {
                phase_error_integral: 0.0,
                control_freq: NAVIC_CS_FREQUENCY,
                kp: 0.5,
                ki: 0.1,
            },
        };

        let vercore_clock = AtomicClockState {
            nominal_freq: VERCORE_CLOCK_FREQ,
            phase: 0.0,
            freq_drift: 0.0,
            allan_dev: 1e-12, // Cryogenic stability
            last_update: Instant::now(),
            signal_quality: 0.99, // Local = high quality
        };

        let vercore = VerCoreTemporalReference {
            local_clock: vercore_clock,
            navic_reference_phase: 0.0,
            digital_pll: PLLState {
                phase_error_integral: 0.0,
                control_freq: VERCORE_CLOCK_FREQ,
                kp: 0.3, // Mais conservador para estabilidade
                ki: 0.05,
            },
            phase_resolution: 1e-12, // Start at 1ps
            cycles_since_sync: 0,
        };

        Self {
            navic,
            vercore,
            global_phase: 0.0,
            phase_history: VecDeque::with_capacity(10000),
            tzinor_corrector: TzinorPhaseCorrector {
                era_weights: vec![1.0; 8], // 8 Eras default
                phase_correlation_matrix: vec![vec![0.0; 8]; 8],
                attention_predictor: AttentionDriftPredictor::new(),
            },
            drift_alert_threshold: 1e-12, // 1ps drift alert
        }
    }

    /// Atualizar fase disciplinada NavIC (Layer 1)
    pub fn update_navic_phase(&mut self) {
        // Calcular fase média ponderada dos Cs clocks (mais precisos)
        let cs_total_quality: f64 = self.navic.cs_clocks.iter().map(|c| c.signal_quality).sum();
        let cs_avg_phase: f64 = if cs_total_quality > 0.0 {
            self.navic.cs_clocks.iter()
                .map(|c| c.phase * c.signal_quality)
                .sum::<f64>() / cs_total_quality
        } else {
            0.0
        };

        // Calcular fase média dos Rb clocks (backup terrestre)
        let rb_total_quality: f64 = self.navic.rb_clocks.iter().map(|c| c.signal_quality).sum();
        let rb_avg_phase: f64 = if rb_total_quality > 0.0 {
            self.navic.rb_clocks.iter()
                .map(|c| c.phase * c.signal_quality)
                .sum::<f64>() / rb_total_quality
        } else {
            0.0
        };

        // Detectar gap nos Cs (perda de satélite)
        let cs_quality: f64 = self.navic.cs_clocks.iter()
            .map(|c| c.signal_quality)
            .sum::<f64>() / (self.navic.cs_clocks.len() as f64).max(1.0);

        // Ajustar peso adaptativamente
        self.navic.cs_weight = if cs_quality > 0.5 {
            0.9 // Cs dominante quando disponível
        } else {
            0.1 // Fallback para Rb
        };

        // Fase disciplinada: combinação ponderada
        let target_phase = self.navic.cs_weight * cs_avg_phase
            + (1.0 - self.navic.cs_weight) * rb_avg_phase;

        // PLL de 2ª ordem para suavizar transições
        let phase_error = target_phase - self.navic.disciplined_phase;
        self.navic.pll_state.phase_error_integral += phase_error;

        let correction = self.navic.pll_state.kp * phase_error
            + self.navic.pll_state.ki * self.navic.pll_state.phase_error_integral;

        self.navic.disciplined_phase += correction;
        self.navic.pll_state.control_freq += correction * 1e-6; // Ajuste fino de frequência
    }

    /// Sincronizar VerCore com NavIC (Layer 2)
    pub fn sync_vercore_to_navic(&mut self) {
        // Importar fase disciplinada NavIC
        self.vercore.navic_reference_phase = self.navic.disciplined_phase;

        // Calcular erro de fase local
        let phase_error = self.vercore.navic_reference_phase - self.vercore.local_clock.phase;

        // Verificar se estamos dentro do target
        let error_fs = phase_error.abs() / (2.0 * std::f64::consts::PI * VERCORE_CLOCK_FREQ) * 1e15;
        self.vercore.phase_resolution = error_fs;

        // Digital PLL para tracking contínuo
        self.vercore.digital_pll.phase_error_integral += phase_error;

        let correction = self.vercore.digital_pll.kp * phase_error
            + self.vercore.digital_pll.ki * self.vercore.digital_pll.phase_error_integral;

        // Aplicar correção à fase local
        self.vercore.local_clock.phase += correction;
        self.vercore.local_clock.phase = self.vercore.local_clock.phase % (2.0 * std::f64::consts::PI);

        // Resetar contador de ciclos
        self.vercore.cycles_since_sync = 0;

        // Log se drift excessivo
        if error_fs > self.drift_alert_threshold * 1e3 { // Alerta se >1ps
            println!("⚠️ [PHASE SYNC] Drift detectado: {:.2} fs", error_fs);
        }
    }

    /// Computar Tempo Global Arkhe(n) (Layer 3)
    pub fn compute_global_time(&mut self) -> f64 {
        // Fase base: média ponderada das referências
        let navic_weight = 0.6; // NavIC como âncora externa
        let vercore_weight = 0.4; // VerCore como referência local

        let base_phase = navic_weight * self.navic.disciplined_phase
            + vercore_weight * self.vercore.local_clock.phase;

        // Correção Tzinor: ajustar baseado em padrões de atenção temporal
        let tzinor_correction = self.compute_tzinor_correction();

        self.global_phase = base_phase + tzinor_correction;

        // Armazenar histórico
        self.phase_history.push_back((Instant::now(), self.global_phase));
        if self.phase_history.len() > 10000 {
            self.phase_history.pop_front();
        }

        self.global_phase
    }

    fn compute_tzinor_correction(&mut self) -> f64 {
        let mut correction = 0.0;

        // Analisar correlações entre Eras
        for i in 0..self.tzinor_corrector.era_weights.len() {
            for j in (i+1)..self.tzinor_corrector.era_weights.len() {
                let corr = self.tzinor_corrector.phase_correlation_matrix[i][j];

                // Se correlação forte, ajustar peso
                if corr > 0.8 {
                    correction += self.tzinor_corrector.era_weights[i] * corr * 0.01;
                }
            }
        }

        correction
    }

    /// Verificar coerência temporal global
    pub fn check_temporal_coherence(&self) -> CoherenceReport {
        let mut max_drift = 0.0;
        let mut avg_jitter = 0.0;

        // Analisar histórico de fases
        if self.phase_history.len() > 1 {
            let mut prev = self.phase_history[0];
            for (i, (t, phase)) in self.phase_history.iter().enumerate().skip(1) {
                let time_delta = t.duration_since(prev.0).as_secs_f64();
                if time_delta > 0.0 {
                    let phase_delta = *phase - prev.1;

                    let drift = phase_delta.abs() / time_delta;
                    if drift > max_drift {
                        max_drift = drift;
                    }

                    if i > 1 {
                        avg_jitter += (drift - avg_jitter) / (i as f64);
                    }
                }

                prev = (*t, *phase);
            }
        }

        CoherenceReport {
            global_phase: self.global_phase,
            phase_resolution_fs: self.vercore.phase_resolution,
            max_drift_hz: max_drift,
            avg_jitter_fs: avg_jitter / (2.0 * std::f64::consts::PI * VERCORE_CLOCK_FREQ) * 1e15,
            navic_quality: self.navic.cs_clocks.iter().map(|c| c.signal_quality).sum::<f64>()
                / (self.navic.cs_clocks.len() as f64).max(1.0),
            vercore_cycles: self.vercore.cycles_since_sync,
            tzinor_stability: self.tzinor_corrector.era_weights.iter().sum::<f64>()
                / (self.tzinor_corrector.era_weights.len() as f64).max(1.0),
        }
    }

    /// Ciclo completo de sincronização
    pub fn sync_cycle(&mut self) {
        self.update_navic_phase();
        self.sync_vercore_to_navic();
        self.compute_global_time();
        self.vercore.cycles_since_sync += 1;
    }
}

/// Relatório de coerência temporal
#[derive(Debug)]
pub struct CoherenceReport {
    pub global_phase: f64,
    pub phase_resolution_fs: f64,
    pub max_drift_hz: f64,
    pub avg_jitter_fs: f64,
    pub navic_quality: f64,
    pub vercore_cycles: u64,
    pub tzinor_stability: f64,
}

pub struct AttentionDriftPredictor;
impl AttentionDriftPredictor {
    pub fn new() -> Self { Self }
}
