const LN2: f64 = 0.693147180559945;
const SQUARE_LN2: f64 = 0.480453013918201;

pub struct Bloom {
    bits: u32,
    hashes: i32,
    bf: Vec<u8>,
    ready: bool
}

impl Bloom {
    pub fn new<'a>(entries: i32, error_rate: f64) -> Result<Bloom, &'a str> {
        if entries < 1 || error_rate <= 0.0 || error_rate >= 1.0 {
            return Err("entries should greater than or equal 1, error_rate should between 0 and 1");
        }

        let bpe = -error_rate.ln() / SQUARE_LN2;
        let bits = (entries as f64 * bpe) as u32;

        let bytes = (if bits % 8 != 0 { bits / 8 + 1 } else { bits / 8 }) as usize;
        let hashes = (LN2 * bpe).ceil() as i32;

        Ok(Bloom {
            bits: bits,
            hashes: hashes,
            bf: vec![0; bytes],
            ready: true
        })
    }

    fn check_add(&mut self, buffer: &[u8], adding: bool) -> bool {
        assert!(self.ready);

        let a = murmurhash2(buffer, 0x9747b28c);
        let b = murmurhash2(buffer, a);

        let mut hits = 0;

        for i in 0..self.hashes {
            let x = (a as u64 + i as u64 * b as u64) as u32 % self.bits;
            let byte = (x >> 3) as usize;
            let c = self.bf[byte];
            let mask = 1 << (x % 8);

            if c & mask != 0 {
                hits += 1;
            } else if adding {
                self.bf[byte] = c | mask;
            }
        }

        if hits == self.hashes { true } else { false }
    }

    pub fn check(&mut self, buffer: &str) -> bool {
        self.check_add(buffer.as_bytes(), false)
    }

    pub fn add(&mut self, buffer: &str) -> bool {
        self.check_add(buffer.as_bytes(), true)
    }
}

pub fn murmurhash2(key: &[u8], seed: u32) -> u32 {
    let m: u32 = 0x5bd1e995;
    let r = 24;
    let mut len = key.len() as u32;
    let mut h: u32 = seed ^ len;
    let mut i = 0;

    while len / 4 > 0 {
        let mut k = (key[i] as u32) | ((key[i + 1] as u32) << 8) |
            ((key[i + 2] as u32) << 16) | ((key[i + 3] as u32) << 24);
        k = ((k as u64) * (m as u64)) as u32;
        k ^= k >> r;
        k = ((k as u64) * (m as u64)) as u32;

        h = ((h as u64) * (m as u64)) as u32;
        h ^= k;
        len -= 4;
        i += 4;
    }

    while len > 0 {
        match len {
            3 => h ^= (key[i + 2] as u32) << 16,
            2 => h ^= (key[i + 1] as u32) << 8,
            1 => {
                h ^= key[i] as u32;
                h = ((h as u64) * (m as u64)) as u32;
            },
            _ => unreachable!()
        }
        len -= 1;
    }

    h ^= h >> 13;
    h = ((h as u64) * (m as u64)) as u32;
    h ^= h >> 15;
    return h;
}
