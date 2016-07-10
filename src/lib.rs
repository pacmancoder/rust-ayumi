//! Bindings to *AY-3-8910* and *YM2149* emulation library
//! [ayumi](https://github.com/pacmancoder/ayumi-lib)
#![allow(dead_code)]
#![deny(missing_docs)]
use std::i32;

mod ffi;

/// Emulated chip type
pub enum ChipType {
    /// AY-3-8910
    AY = 0,
    /// YM2149
    YM = 1,
}

/// Tone channel id
pub enum ToneChannel {
    /// Channel A
    A = 0,
    /// Channel B
    B = 1,
    /// Channel C
    C = 2,
}

/// Represents tone channel control structure
pub struct ToneDescriptor<'a> {
    ayumi: &'a mut Ayumi,
    index: i32,
}

impl<'a> ToneDescriptor<'a> {
    /// Changes period of current tone channel
    /// # Panics
    /// When `period` < 0
    pub fn period(&mut self, period: i32) -> &mut Self {
        assert!(period >= 0);
        unsafe {
            ffi::ayumi_set_tone(&mut self.ayumi.c_ayumi, self.index, period);
        }
        self
    }

    /// Changes pan of current tone channel
    /// # Arguments
    /// - `pan` - value in range 0...1, which represents pan of channel
    /// - `equal_power` - flag, which used to enable "equal_power" panning
    pub fn pan(&mut self, pan: f64, equal_power: bool) -> &mut Self {
        let pan = if pan > 1.0 {
            1.0
        } else if pan < 0.0 {
            0.0
        } else {
            pan
        };
        unsafe {
            ffi::ayumi_set_pan(&mut self.ayumi.c_ayumi, self.index, pan, equal_power as i32);
        }
        self
    }

    /// Changes volume of tone channel
    /// # Arguments
    /// - `volume` - volume value of cahnnel in range [0...15]
    /// # Panics
    /// When value is bigger than 15
    pub fn volume(&mut self, volume: u8) -> &mut Self {
        assert!(volume <=0x0F);
        unsafe {
            ffi::ayumi_set_volume(&mut self.ayumi.c_ayumi, self.index, volume as i32);
        }
        self
    }

    /// Changes mixer flags of channel, use it to enable/disable sources mixing
    /// # Arguments
    /// - `tone` - tone enable flag
    /// - `noise` - noise enable flag
    /// - `envelope` - envelope enable flag
    pub fn mixer(&mut self, tone: bool, noise: bool, envelope: bool) -> &mut Self {
        unsafe {
            ffi::ayumi_set_mixer(&mut self.ayumi.c_ayumi,
                                 self.index,
                                 tone as i32,
                                 noise as i32,
                                 envelope as i32);
        }
        self
    }

}

/// Represents noise channel control structure
pub struct NoiseDescriptor<'a> {
    ayumi: &'a mut Ayumi,
}

impl<'a> NoiseDescriptor<'a> {
    /// Changes period of noise channel
    /// # Panics
    /// When `period` < 0
    /// # Arguments
    /// - `period` - noise period
    pub fn period(&mut self, period: i32) -> &mut Self {
        assert!(period >= 0);
        unsafe {
            ffi::ayumi_set_noise(&mut self.ayumi.c_ayumi, period);
        }
        self
    }
}

/// Represents envelope control structure
pub struct EnvelopeDescriptor<'a> {
    ayumi: &'a mut Ayumi,
}

impl<'a> EnvelopeDescriptor<'a> {
    /// Changes period of envelope
    /// # Panics
    /// When `period` < 0
    /// # Arguments
    /// - `period` - period of envelope
    pub fn period(&mut self, period: i32) -> &mut Self {
        assert!(period >= 0);
        unsafe {
            ffi::ayumi_set_envelope(&mut self.ayumi.c_ayumi, period);
        }
        self
    }
    /// Changes envelope shape
    /// # Panics
    /// When `shape` is bigger than 15 (0x0F)
    /// # Arguments
    /// - `shape` - value in range [0...15] which represents shape of envelope
    pub fn shape(&mut self, shape: u8) -> &mut Self {
        assert!(shape <= 0x0F);
        unsafe {
            ffi::ayumi_set_envelope_shape(&mut self.ayumi.c_ayumi, shape as i32);
        }
        self
    }
}

/// Represents stereo sample
pub struct StereoSample {
    /// right channel sample
    pub right: f64,
    /// left cahnnel sample
    pub left: f64,
}

/// Main library structure
/// use `new` function for instance creation
pub struct Ayumi {
    c_ayumi: ffi::Ayumi,
}
impl Ayumi {
    /// Constructs new Ayumi instance
    ///
    /// # Arguments
    /// - `chip` - `ChipType` of emulator
    /// - `freq` - clock frequency of chip
    /// - `sample_rate` - samples count per second
    ///
    /// # Panics
    /// - `freq` <= 0
    /// - `sample_rate` <= 0`
    /// - ffi function call error
    pub fn new(chip: ChipType, freq: f64, sample_rate: i32) -> Ayumi {
        assert!(freq > 0f64);
        assert!(sample_rate > 0i32);
        let mut c_ayumi = Default::default();
        unsafe {
            ffi::ayumi_configure(&mut c_ayumi, chip as i32, freq, sample_rate);
        }
        Ayumi {
            c_ayumi: c_ayumi,
        }
    }

    /// Returns tone channel descriptor of type `ToneDescriptor` for configuring it.
    pub fn tone<'a>(&'a mut self, channel: ToneChannel) -> ToneDescriptor<'a> {
        ToneDescriptor {
            ayumi: self,
            index: channel as i32,
        }
    }

    /// Returns noise channel descriptor of type `NoiseDescriptor` for configuring it.
    pub fn noise<'a>(&'a mut self) -> NoiseDescriptor<'a> {
        NoiseDescriptor {
            ayumi: self,
        }
    }

    /// Returns envelope descriptor of type `EnvelopeDescriptor` for configuring it.
    pub fn envelope<'a>(&'a mut self) -> EnvelopeDescriptor<'a> {
        EnvelopeDescriptor {
            ayumi: self,
        }
    }

    /// Renders next sample
    pub fn process(&mut self) -> &mut Self {
        unsafe {
            ffi::ayumi_process(&mut self.c_ayumi);
        }
        self
    }

    /// Removes the DC offset from the current sample
    pub fn remove_dc(&mut self) -> &mut Self {
        unsafe {
            ffi::ayumi_remove_dc(&mut self.c_ayumi);
        }
        self
    }

    /// Returns sound sample of type StereoSample
    pub fn sample(&self) -> StereoSample {
        StereoSample {
            right: self.c_ayumi.right,
            left: self.c_ayumi.left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn api_test() {
        let mut ayumi = Ayumi::new(ChipType::AY, 1_000_000f64, 44100i32);
        ayumi.tone(ToneChannel::A).period(440).pan(0.5, false);
        ayumi.noise().period(500);
        ayumi.envelope().period(600).shape(12);
        ayumi.remove_dc();
        ayumi.process();
        let _ = ayumi.sample();
    }

    #[test]
    #[should_panic]
    fn boundaries_test1() {
        let mut ayumi = Ayumi::new(ChipType::AY, -10f64, 10);
    }

    #[test]
    #[should_panic]
    fn boundaries_test2() {
        let mut ayumi = Ayumi::new(ChipType::AY, 10f64, -10);
    }

    #[test]
    #[should_panic]
    fn boundaries_test3() {
        let mut ayumi = Ayumi::new(ChipType::AY, 1_000_000f64, 44100i32);
        ayumi.tone(ToneChannel::A).period(600).volume(16);
    }

    #[test]
    #[should_panic]
    fn boundaries_test4() {
        let mut ayumi = Ayumi::new(ChipType::AY, 1_000_000f64, 44100i32);
        ayumi.noise().period(-10);
    }

    #[test]
    #[should_panic]
    fn boundaries_test5() {
        let mut ayumi = Ayumi::new(ChipType::AY, 1_000_000f64, 44100i32);
        ayumi.envelope().period(10).shape(16);
    }
}
