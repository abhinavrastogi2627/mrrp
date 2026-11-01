use std::f32::consts::TAU;

use mrrp_core::{
    sample::Complex,
    signal::{
        Frequency,
        GetSampleRate,
    },
};

use crate::signal::SignalGenerator;

#[inline]
fn step_from_frequency_and_sample_rate(frequency: Frequency, sample_rate: Frequency) -> f32 {
    // <https://github.com/rust-lang/rust/issues/67057>
    (TAU * (frequency / sample_rate) as f32).rem_euclid(TAU)
}

#[derive(Clone, Copy, Debug)]
pub struct SineWave {
    frequency: Frequency,
    sample_rate: Frequency,
    phase: f32,
    step: f32,
}

impl SineWave {
    #[inline]
    pub fn new(frequency: Frequency, sample_rate: Frequency) -> Self {
        Self {
            frequency,
            sample_rate,
            phase: 0.0,
            step: step_from_frequency_and_sample_rate(frequency, sample_rate),
        }
    }

    #[inline]
    pub fn with_phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }

    #[inline]
    pub fn phase(&self) -> f32 {
        self.phase
    }

    #[inline]
    pub fn set_phase(&mut self, phase: f32) {
        self.phase = phase
    }

    #[inline]
    pub fn set_frequency(&mut self, frequency: Frequency) {
        self.step = step_from_frequency_and_sample_rate(frequency, self.sample_rate);
    }

    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: Frequency) {
        self.step = step_from_frequency_and_sample_rate(self.frequency, sample_rate);
    }
}

impl SignalGenerator for SineWave {
    type Sample = f32;

    fn next(&mut self) -> Self::Sample {
        let output = self.phase.sin();
        self.phase += self.step;
        if self.phase > TAU {
            self.phase -= TAU;
        }
        output
    }
}

impl GetSampleRate for SineWave {
    #[inline]
    fn sample_rate(&self) -> Frequency {
        self.sample_rate
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ComplexSinusoid {
    frequency: Frequency,
    sample_rate: Frequency,
    phase: f32,
    step: f32,
}

impl ComplexSinusoid {
    #[inline]
    pub fn new(frequency: Frequency, sample_rate: Frequency) -> Self {
        Self {
            frequency,
            sample_rate,
            phase: 0.0,
            step: step_from_frequency_and_sample_rate(frequency, sample_rate),
        }
    }

    #[inline]
    pub fn with_phase(mut self, phase: f32) -> Self {
        self.set_phase(phase);
        self
    }

    #[inline]
    pub fn phase(&self) -> f32 {
        self.phase
    }

    #[inline]
    pub fn set_phase(&mut self, phase: f32) {
        self.phase = phase
    }

    #[inline]
    pub fn set_frequency(&mut self, frequency: Frequency) {
        self.step = step_from_frequency_and_sample_rate(frequency, self.sample_rate);
    }

    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: Frequency) {
        self.step = step_from_frequency_and_sample_rate(self.frequency, sample_rate);
    }
}

impl SignalGenerator for ComplexSinusoid {
    type Sample = Complex<f32>;

    fn next(&mut self) -> Self::Sample {
        let output = Complex {
            im: self.phase,
            re: 0.0,
        }
        .exp();
        self.phase += self.step;
        if self.phase > TAU {
            self.phase -= TAU;
        }
        output
    }
}

impl GetSampleRate for ComplexSinusoid {
    #[inline]
    fn sample_rate(&self) -> Frequency {
        self.sample_rate
    }
}

#[inline]
pub fn sine(frequency: Frequency, sample_rate: Frequency) -> SineWave {
    SineWave::new(frequency, sample_rate)
}

#[inline]
pub fn complex_sinusoid(frequency: Frequency, sample_rate: Frequency) -> ComplexSinusoid {
    ComplexSinusoid::new(frequency, sample_rate)
}
