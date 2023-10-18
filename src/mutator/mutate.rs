use crate::utils::rng::Rng;

pub struct DynFile {
    pub data: Vec<u8>,
    max_input_sz: usize
}

pub struct Run {
    pub dyn_file: DynFile,
    offset: usize,
    printable: bool,
}

pub struct Mutator {
    pub run: Run,
    rng: Rng,
}

impl Mutator {
    pub fn new() -> Self {
        Mutator {
            run: Run {
                dyn_file: DynFile { data: Vec::new(), max_input_sz: 0 },
                offset: 0,
                printable: false,
            },
            rng: Rng {
                seed: 0,
                exp_disabled: false,
            },
        }
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.run.offset = offset;
        self
    }

    pub fn max_sz(mut self, sz: usize) -> Self {
        self.run.dyn_file.max_input_sz = sz;
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.rng.seed = seed;
        self
    }

    pub fn exp_disabled(mut self, exp: bool) -> Self {
        self.rng.exp_disabled = exp;
        self
    }

    pub fn printable(mut self, printable: bool) -> Self {
        self.run.printable = printable;
        self
    }

    pub fn offset_int(&mut self) -> usize {
        if !self.run.dyn_file.data.is_empty() {
            self.rng.rand(0, self.run.dyn_file.data.len() - 1)
        } else {
            0
        }
    }

    pub fn dyn_file(&mut self, data: &Vec<u8>) -> &mut Mutator {
        self.run.dyn_file.data = data.clone();
        self
    }

    pub fn mutate(&mut self, mutations: usize) {
        let algo = [
            Self::mangle_inc_byte,
            Self::mangle_neg_byte,
            Self::mangle_dec_byte,
            Self::mangle_ins_byte,
            Self::mangle_ovw_byte,
            Self::mangle_del_byte,
            Self::mangle_repeat_insert,
            Self::mangle_repeat_ovw,
            Self::mangle_splice_insert,
            Self::mangle_splice_overwrite,
            // TODO
        ];
        for _ in 0..mutations {
            let sel = self.rng.rand(0, algo.len() - 1);
            let choose = algo[sel];
            choose(self);
        }
    }

    fn mangle_splice_insert(&mut self) {
        let donor = self.run.dyn_file.data.clone();
        let insert_offset = self.rng.rand(0, self.run.dyn_file.data.len());

        // Ensure that donor_offset is within bounds
        let donor_offset = self.rng.rand(0, donor.len().saturating_sub(1));
        // Ensure that donor_length does not exceed the remaining elements in the donor slice
        let max_length = donor.len() - donor_offset;
        let donor_length = self.rng.rand(1, max_length);

        let splice = &donor[donor_offset..(donor_offset + donor_length)];

        if insert_offset <= self.run.dyn_file.data.len() {
            self.run.dyn_file.data.splice(insert_offset..insert_offset, splice.iter().cloned());
        } else {
            self.run.dyn_file.data.extend(splice.iter().cloned());
        }
    }


    fn mangle_repeat_ovw(&mut self) {
        let mut chn_data = Vec::new();
        chn_data.extend_from_slice(&self.run.dyn_file.data);
        let off = self.offset_int();
        for (index, _) in self.run.dyn_file.data.iter().enumerate() {
            let select_val = chn_data[off];
            if off == index {
                let rnd = self.rng.rand(0, self.run.dyn_file.data.len());
                for item in 0..rnd {
                    chn_data[item] = select_val;
                }
            }
        }
        self.run.dyn_file.data = chn_data;
    }

    fn mangle_splice_overwrite(&mut self) {
        if self.run.dyn_file.data.is_empty() {
            return;
        }
        let donor = self.run.dyn_file.data.clone();
        let overwrite_offset = self.rng.rand_exp(0,self.run.dyn_file.data.len());
        let donor_offset = self.rng.rand_exp(0,donor.len());
        let donor_length = self.rng.rand_exp(1,donor.len() - donor_offset + 1);
        let splice;
        if (donor_offset + donor_length) > donor.len() {
            splice = &donor[donor_offset..donor.len()];
        } else {
            splice = &donor[donor_offset..(donor_offset + donor_length)];
        }
        let start = overwrite_offset;
        let end = start + splice.len();

        if end <= self.run.dyn_file.data.len() {
            self.run.dyn_file.data[start..end].copy_from_slice(splice);
        }
    }

    fn mangle_repeat_insert(&mut self) {
        let off = self.offset_int();
        let mut modified_data = Vec::new();
        for (index, val) in self.run.dyn_file.data.iter().enumerate() {
            modified_data.push(*val);
            if index == off {
                let rnd = self.rng.rand(0, 9);
                for _ in 1..rnd {
                    modified_data.push(*val);
                }
            }
        }
        self.run.dyn_file.data = modified_data;
    }

    pub fn mangle_random_insert(&mut self) {
        let offset = self.offset_int();
        // let amount = self.rng.gen_range(1..=self.max_input_size);
        let insert_index = offset.min(self.run.dyn_file.data.len());
        let random_bytes: Vec<u8> = (0..self.run.dyn_file.max_input_sz).map(|_| {
            if self.run.printable {
                self.rng.rand(32, 126) as u8
            } else {
                self.rng.rand(0, 255) as u8
            }
        }).collect();
        self.run.dyn_file.data.splice(insert_index..insert_index, random_bytes);
    }

    fn mangle_neg_byte(&mut self) {
        let off = self.offset_int();
        if self.run.printable {
            let val = self.run.dyn_file.data[off] as i32;
            let new_val = 94 - (val - 32) + 32;
            self.run.dyn_file.data[off] = new_val as u8;
        } else {
            self.run.dyn_file.data[off] = !self.run.dyn_file.data[off];
        }
    }

    fn mangle_ins_byte(&mut self) {
        let off = self.offset_int();
        if off <= self.run.dyn_file.data.len() {
            let mut bytes_to_insert = vec![0u8; 1];
            for byte in &mut bytes_to_insert {
                *byte = (self.rng.next() & 0xFF) as u8;
            }
            self.run.dyn_file.data.splice(off..off, bytes_to_insert);
        } else {
            println!("Offset is out of bounds, skipping byte insertion in mangle_ins_byte.");
        }
    }

    fn mangle_ovw_byte(&mut self) {
        let off = self.offset_int();
        let random_byte = (self.rng.next() & 0xFF) as u8;
        self.run.dyn_file.data[off] = random_byte;
    }

    fn mangle_del_byte(&mut self) {
        let off = self.offset_int();
        self.run.dyn_file.data.remove(off);
    }

    fn mangle_dec_byte(&mut self) {
        let offset = self.offset_int();
        if self.run.printable {
            self.run.dyn_file.data[offset] =
                ((self.run.dyn_file.data[offset] as i32 - 32 + 94) % 95 + 32) as u8;
        } else {
            if self.run.dyn_file.data[offset] > 0 {
                self.run.dyn_file.data[offset] -= 1;
            }
        }
    }

    fn mangle_inc_byte(&mut self) {
        let off = self.offset_int();
        if self.run.printable {
            self.run.dyn_file.data[off] = ((self.run.dyn_file.data[off] as i32 - 32 + 1).rem_euclid(95) + 32) as u8;
        } else {
            self.run.dyn_file.data[off] = self.run.dyn_file.data[off].wrapping_add(1);
        }
    }

}