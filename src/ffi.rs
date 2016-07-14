// The MIT License (MIT)
//
// Copyright (c) 2016 Vladislav Nikonov
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Module contains raw c-fucntions and types
//! Generated with rust-bindgen and some moments
//! were fixed after it
#![allow(dead_code)]

use std::os::raw::*;
use std::mem;

// in original c-library enum-constants were used.
// changed to ordinary constants
const TONE_CHANNELS: usize = 3;
const DECIMATE_FACTOR: usize = 8;
const FIR_SIZE: usize = 192;
const DC_FILTER_SIZE: usize = 1024;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ToneChannel {
    pub tone_period: c_int,
    pub tone_counter: c_int,
    pub tone: c_int,
    pub t_off: c_int,
    pub n_off: c_int,
    pub e_on: c_int,
    pub volume: c_int,
    pub pan_left: c_double,
    pub pan_right: c_double,
}
impl Default for ToneChannel {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Interpolator {
    pub c: [c_double; 4],
    pub y: [c_double; 4],
}
impl Default for Interpolator {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct DCFilter {
    pub sum: c_double,
    pub delay: [c_double; DC_FILTER_SIZE],
}
impl Clone for DCFilter {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for DCFilter {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Ayumi {
    pub channels: [ToneChannel; TONE_CHANNELS],
    pub noise_period: c_int,
    pub noise_counter: c_int,
    pub noise: c_int,
    pub envelope_counter: c_int,
    pub envelope_period: c_int,
    pub envelope_shape: c_int,
    pub envelope_segment: c_int,
    pub envelope: c_int,
    pub dac_table: *const c_double,
    pub step: c_double,
    pub x: c_double,
    pub interpolator_left: Interpolator,
    pub interpolator_right: Interpolator,
    pub fir_left: [c_double; FIR_SIZE * 2],
    pub fir_right: [c_double; FIR_SIZE * 2],
    pub fir_index: c_int,
    pub dc_left: DCFilter,
    pub dc_right: DCFilter,
    pub dc_index: c_int,
    pub left: c_double,
    pub right: c_double,
}
impl Clone for Ayumi {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for Ayumi {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
extern "C" {
    pub fn ayumi_configure(ay: *mut Ayumi, is_ym: c_int, clock_rate: c_double, sr: c_int);
    pub fn ayumi_set_pan(ay: *mut Ayumi, index: c_int, pan: c_double, is_eqp: c_int);
    pub fn ayumi_set_tone(ay: *mut Ayumi, index: c_int, period: c_int);
    pub fn ayumi_set_noise(ay: *mut Ayumi, period: c_int);
    pub fn ayumi_set_mixer(ay: *mut Ayumi, index: c_int, t_off: c_int, n_off: c_int, e_on: c_int);
    pub fn ayumi_set_volume(ay: *mut Ayumi, index: c_int, volume: c_int);
    pub fn ayumi_set_envelope(ay: *mut Ayumi, period: c_int);
    pub fn ayumi_set_envelope_shape(ay: *mut Ayumi, shape: c_int);
    pub fn ayumi_process(ay: *mut Ayumi);
    pub fn ayumi_remove_dc(ay: *mut Ayumi);
}
