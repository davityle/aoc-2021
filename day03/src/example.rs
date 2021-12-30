use std::slice::Iter;

fn main() {
    let bit_masks: [u8; 5] = [0b10000, 0b1000, 0b100, 0b10, 0b1];

    let mut ones: [bool; 5] = [false; 5];
    let mut oxygen = TEST_DIAGNOSTICS.to_vec();
    let mut c02_scrub = TEST_DIAGNOSTICS.to_vec();

    for (i, mask) in bit_masks.iter().enumerate() {
        ones[i] = test_mask(*mask, TEST_DIAGNOSTICS.iter());
        
        if oxygen.len() > 1 {
            let oxygen_mask_res = test_mask(*mask, oxygen.iter());
            if oxygen_mask_res {
                oxygen = oxygen.into_iter().filter(|n| (*n & *mask) > 0).collect();
            } else {
                oxygen = oxygen.into_iter().filter(|n| (*n & *mask) == 0).collect();
            }
        }

        if c02_scrub.len() > 1 {
            let c02_scrub_res = test_mask(*mask, c02_scrub.iter());
            if c02_scrub_res {
                c02_scrub = c02_scrub.into_iter().filter(|n| (*n & *mask) == 0).collect();
            } else {
                c02_scrub = c02_scrub.into_iter().filter(|n| (*n & *mask) > 0).collect();
            }
            println!("{}, {:?}", c02_scrub_res, c02_scrub);
        }
    }
    let compare = |i: usize, v: u8| if ones[i] { v } else { 0 };

    let gamma: u8 = bit_masks
        .iter()
        .enumerate()
        .map(|(i, mask)| compare(i, *mask))
        .fold(0, |a, b| a | b);
    let epsilon: u8 = !gamma & 0b00011111;

    let life_support = oxygen[0] as u32 * c02_scrub[0] as u32;

    println!("{}", gamma as u32 * epsilon as u32);
    println!("{}", life_support);
}

fn test_mask(mask: u8, iter: Iter<u8>) -> bool {
    let mut ones = 0;
    let mut zeros = 0;
    for n in iter {
        if n & mask > 0 {
            ones = ones + 1;
        } else {
            zeros = zeros + 1;
        }
    }
    ones >= zeros
}

const TEST_DIAGNOSTICS: [u8; 12] = [
    0b00000100, 0b00011110, 0b00010110, 0b00010111, 0b00010101, 0b00001111, 0b00000111, 0b00011100,
    0b00010000, 0b00011001, 0b00000010, 0b00001010,
];
