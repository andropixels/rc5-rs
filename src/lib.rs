#![forbid(unsafe_code)]


pub struct RC5 {
    rounds: u32,
    sub_keys: Vec<u32>,
}






impl RC5 {
    pub fn new(key: &[u8], rounds: u32) -> Self {
        assert!(rounds > 0, "Number of rounds must be positive");
        assert!(!key.is_empty(), "Key must not be empty");
        
        let s = Self::expand_key(key, rounds);
        
        RC5 {
            rounds,
            sub_keys: s,
        }
    }

    fn expand_key(key: &[u8], rounds: u32) -> Vec<u32> {
        const P32: u32 = 0xB7E15163;
        const Q32: u32 = 0x9E3779B9;
        let t = 2 * (rounds + 1);
        let mut s = vec![0u32; t as usize];
        
        // Initialize S array
        s[0] = P32;
        for i in 1..t as usize {
            s[i] = s[i-1].wrapping_add(Q32);
        }
        
        // Mix in the key
        let mut l = vec![0u32; (key.len() + 3) / 4];
        for i in 0..key.len() {
            l[i / 4] |= (key[i] as u32) << ((i % 4) * 8);
        }
        
        let mut i = 0;
        let mut j = 0;
        let mut a = 0u32;
        let mut b = 0u32;
        let v = 3 * std::cmp::max(t as usize, l.len());
        
        for _ in 0..v {
            let rotated = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
            s[i] = rotated;
            a = rotated;
            
            let rotated_l = (l[j].wrapping_add(a).wrapping_add(b)).rotate_left((a.wrapping_add(b)) as u32);
            l[j] = rotated_l;
            b = rotated_l;
            i = (i + 1) % t as usize;
            j = (j + 1) % l.len();
        }
        
        s
    }

    pub fn encrypt_block(&self, block: &mut [u8; 8]) {
        let mut a = u32::from_le_bytes([block[0], block[1], block[2], block[3]]);
        let mut b = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);
        
        a = a.wrapping_add(self.sub_keys[0]);
        b = b.wrapping_add(self.sub_keys[1]);
        
        for i in 1..=self.rounds {
            a = (a ^ b).rotate_left(b as u32).wrapping_add(self.sub_keys[2*i as usize]);
            b = (b ^ a).rotate_left(a as u32).wrapping_add(self.sub_keys[2*i as usize + 1]);
        }
        
        block[0..4].copy_from_slice(&a.to_le_bytes());
        block[4..8].copy_from_slice(&b.to_le_bytes());
    }

    pub fn decrypt_block(&self, block: &mut [u8; 8]) {
        let mut a = u32::from_le_bytes([block[0], block[1], block[2], block[3]]);
        let mut b = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);
        
        for i in (1..=self.rounds).rev() {
            b = b.wrapping_sub(self.sub_keys[2*i as usize + 1]).rotate_right(a as u32) ^ a;
            a = a.wrapping_sub(self.sub_keys[2*i as usize]).rotate_right(b as u32) ^ b;
        }
        
        b = b.wrapping_sub(self.sub_keys[1]);
        a = a.wrapping_sub(self.sub_keys[0]);
        
        block[0..4].copy_from_slice(&a.to_le_bytes());
        block[4..8].copy_from_slice(&b.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let key = b"my secret key";
        let rounds = 12;
        let rc5 = RC5::new(key, rounds);
        
        let mut block = [0u8; 8];
        block.copy_from_slice(b"testtest");
        
        let original = block;
        rc5.encrypt_block(&mut block);
        assert_ne!(block, original); // Encrypted should be different
        
        rc5.decrypt_block(&mut block);
        assert_eq!(block, original); // Decrypted should match original
    }

    #[test]
    #[should_panic(expected = "Number of rounds must be positive")]
    fn test_invalid_rounds() {
        let key = b"test key";
        RC5::new(key, 0);
    }

    #[test]
    #[should_panic(expected = "Key must not be empty")]
    fn test_empty_key() {
        let key = b"";
        RC5::new(key, 12);
    }
}
