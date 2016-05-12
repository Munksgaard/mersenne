use std::u32;

/// Inspired by the pseudocode and python implementation of the mersenne twister
/// on https://en.wikipedia.org/wiki/Mersenne_Twister

pub struct Mersenne<T> {
    w: T, n: T, m: T, r: T,

    a: T,

    u: T, d: T,

    s: T, b: T,

    t: T, c: T,

    l: T,

    f: T,

    mt: Vec<T>,
    index: T,

    lower_mask: T,
    upper_mask: T,
}

pub fn seed_mt(seed: u32) -> Mersenne<u32> {
    let mut result = Mersenne {
        w: 32, n: 624, m: 397, r: 31,

        a: 0x9908B0DF,

        u: 11, d: 0xFFFFFFFF,

        s: 7, b: 0x9D2C5680,

        t: 15, c: 0xEFC60000,

        l: 18,

        f: 1812433253,

        mt: vec!(seed),
        index: 624,

        lower_mask: 0x7fffffff,
        upper_mask: 0x80000000,
    };

    for i in 1..result.n {
        let tmp =
            u32::wrapping_add(u32::wrapping_mul(result.f,
                                                result.mt[i as usize - 1] ^
                                                (result.mt[i as usize - 1]
                                                 >> (result.w - 2))),
                                    i);

        result.mt.push(tmp);
    }

    result
}

impl Mersenne<u32> {
    pub fn extract_number(&mut self) -> u32 {
        if self.index >= self.n {
            if self.index > self.n {
                unreachable!("Generator was never seeded");
            }
            self.twist();
        }

        let mut y = self.mt[self.index as usize];
        y = y ^ ((y >> self.u) & self.d);
        y = y ^ ((y << self.s) & self.b);
        y = y ^ ((y << self.t) & self.c);
        y = y ^ (y >> self.l);

        self.index += 1;
        y
    }

    fn twist(&mut self) {
        for i in 0..(self.n) {
            let x = u32::wrapping_add(self.mt[i as usize] & self.upper_mask,
                                      self.mt[((i+1) % self.n) as usize]
                                      & self.lower_mask);

            let mut xA = x >> 1;
            if (x % 2) != 0 {
                xA = xA ^ self.a
            }
            self.mt[i as usize] =
                self.mt[((i + self.m) % self.n) as usize] ^ xA;
        }
        self.index = 0;
    }
}
