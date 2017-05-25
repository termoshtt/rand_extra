//! Random number generation

extern crate rand;
extern crate num_complex;

use std::marker::PhantomData;
use rand::Rng;
use rand::distributions::*;
use num_complex::Complex;

/// Normal distribution for real numbers
#[derive(Clone, Copy)]
pub struct RealNormal<A> {
    dist: Normal,
    phantom: PhantomData<A>,
}

impl<A> RealNormal<A> {
    pub fn new(center: f64, var: f64) -> Self {
        RealNormal {
            dist: Normal::new(center, var),
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_RealNormal {
    ($float:ty) => {
impl Sample<$float> for RealNormal<$float> {
    fn sample<R>(&mut self, rng: &mut R) -> $float
        where R: Rng
    {
        self.dist.sample(rng) as $float
    }
}

impl IndependentSample<$float> for RealNormal<$float> {
    fn ind_sample<R>(&self, rng: &mut R) -> $float
        where R: Rng
    {
        self.dist.ind_sample(rng) as $float
    }
}
}} // impl_RealNormal

impl_RealNormal!(f64);
impl_RealNormal!(f32);

/// Normal distribution for complex numbers
#[derive(Clone, Copy)]
pub struct ComplexNormal<A> {
    re_dist: Normal,
    im_dist: Normal,
    phantom: PhantomData<A>,
}

impl<A> ComplexNormal<A> {
    pub fn new(re0: f64, im0: f64, re_var: f64, im_var: f64) -> Self {
        ComplexNormal {
            re_dist: Normal::new(re0, re_var),
            im_dist: Normal::new(im0, im_var),
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_ComplexNormal {
    ($float:ty) => {
impl Sample<Complex<$float>> for ComplexNormal<$float> {
    fn sample<R>(&mut self, rng: &mut R) -> Complex<$float>
        where R: Rng
    {
        let re = self.re_dist.sample(rng) as $float;
        let im = self.im_dist.sample(rng) as $float;
        Complex::new(re, im)
    }
}

impl IndependentSample<Complex<$float>> for ComplexNormal<$float> {
    fn ind_sample<R>(&self, rng: &mut R) -> Complex<$float>
        where R: Rng
    {
        let re = self.re_dist.ind_sample(rng) as $float;
        let im = self.im_dist.ind_sample(rng) as $float;
        Complex::new(re, im)
    }
}
}} // impl_ComplexNormal

impl_ComplexNormal!(f32);
impl_ComplexNormal!(f64);
