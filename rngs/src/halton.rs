use itertools::Itertools;

use crate::traits::SimpleRng;

pub struct HaltonSequence {
    pub bases: Vec<u32>,
    pub nums: Vec<u32>,
    pub dens: Vec<u32>,
}

impl Default for HaltonSequence {
    fn default() -> Self {
        Self {
            bases: vec![2, 3],
            nums: vec![0, 0],
            dens: vec![1, 1],
        }
    }
}

impl HaltonSequence {
    pub fn ratios(&self) -> Vec<(&u32, &u32)> {
        self.nums.iter().zip(self.dens.iter()).collect_vec()
    }

    pub fn ratio_strings(&self) -> Vec<String> {
        self.nums
            .iter()
            .zip(self.dens.iter())
            .map(|(n, d)| format!("{n}/{d}"))
            .collect_vec()
    }
}

impl SimpleRng for HaltonSequence {
    fn next_u32(&mut self) -> u32 {
        for ((num, den), base) in self
            .nums
            .iter_mut()
            .zip(self.dens.iter_mut())
            .zip(self.bases.iter())
        {
            let x = *den - *num;

            if x == 1 {
                *num = 1;
                *den = *den * base;
            } else {
                let mut y = *den / *base;
                while x <= y {
                    y /= base;
                }
                *num = (*base + 1) * y - x;
            }
        }
        0
    }
}

// #[cfg(test)]
// mod halton_tests {
//     use super::*;

//     #[test]
//     fn test_sequence() {
//         let mut rng = HaltonSequence::default();
//         for _ in 0..10 {
//             rng.step();
//             println!("{:?}", rng.ratio_strings());
//         }
//     }
// }
