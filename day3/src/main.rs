use std::slice::Iter;

fn main() {
    let bit_masks: [u16; 12] = [
        0b100000000000,
        0b10000000000,
        0b1000000000,
        0b100000000,
        0b10000000,
        0b1000000,
        0b100000,
        0b10000,
        0b1000,
        0b100,
        0b10,
        0b1,
    ];

    let mut ones: [bool; 12] = [false; 12];
    let mut oxygen = DIAGNOSTICS.to_vec();
    let mut c02_scrub = DIAGNOSTICS.to_vec();

    for (i, mask) in bit_masks.iter().enumerate() {
        ones[i] = test_mask(*mask, DIAGNOSTICS.iter());
        
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
        }
    }
    let compare = |i: usize, v: u16| if ones[i] { v } else { 0 };

    let gamma: u16 = bit_masks
        .iter()
        .enumerate()
        .map(|(i, mask)| compare(i, *mask))
        .fold(0, |a, b| a | b);
    let epsilon: u16 = !gamma & 0b0000111111111111;

    let life_support = oxygen[0] as u32 * c02_scrub[0] as u32;

    println!("{}", gamma as u32 * epsilon as u32);
    println!("{}", life_support);
}

fn test_mask(mask: u16, iter: Iter<u16>) -> bool {
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

const DIAGNOSTICS: [u16; 1000] = [
    0b0000111110110111,
    0b0000100111000111,
    0b0000011101111101,
    0b0000011011010010,
    0b0000001010001010,
    0b0000111101001001,
    0b0000010001110011,
    0b0000100001001111,
    0b0000110101010000,
    0b0000100001100010,
    0b0000000011000100,
    0b0000111101000101,
    0b0000100100100110,
    0b0000100000011111,
    0b0000101101111101,
    0b0000100000011110,
    0b0000001110101111,
    0b0000101011101011,
    0b0000110011000001,
    0b0000010100011110,
    0b0000100011000011,
    0b0000001000101001,
    0b0000101110010110,
    0b0000101110010100,
    0b0000110000001101,
    0b0000010001010101,
    0b0000110000010010,
    0b0000111011101100,
    0b0000101111011100,
    0b0000101000010111,
    0b0000011000011001,
    0b0000110011110010,
    0b0000110100011010,
    0b0000110110010011,
    0b0000001110001100,
    0b0000011011001100,
    0b0000000011101111,
    0b0000111010101001,
    0b0000000101001000,
    0b0000000010111111,
    0b0000110100100111,
    0b0000011000101000,
    0b0000101011001101,
    0b0000000110110110,
    0b0000010101001111,
    0b0000010011010110,
    0b0000101111100001,
    0b0000000111101100,
    0b0000000000101110,
    0b0000101110001111,
    0b0000010000001101,
    0b0000011111010011,
    0b0000101100011111,
    0b0000011011000000,
    0b0000110000010111,
    0b0000001011010010,
    0b0000101111100000,
    0b0000101010000010,
    0b0000011101101010,
    0b0000110010000111,
    0b0000011110011111,
    0b0000110000010110,
    0b0000011110101000,
    0b0000111001011000,
    0b0000111010011100,
    0b0000011111000101,
    0b0000011111110100,
    0b0000111101101010,
    0b0000111101101001,
    0b0000011011110011,
    0b0000100110100010,
    0b0000100000110000,
    0b0000010001010011,
    0b0000100110111000,
    0b0000101010110100,
    0b0000011011011010,
    0b0000010100100000,
    0b0000011110011100,
    0b0000110101000001,
    0b0000001010111101,
    0b0000011110011000,
    0b0000001101101001,
    0b0000010011001110,
    0b0000100000100111,
    0b0000000100101011,
    0b0000110000000010,
    0b0000100011101011,
    0b0000100101010010,
    0b0000001010100111,
    0b0000001111011100,
    0b0000010111100101,
    0b0000000111000000,
    0b0000010001011101,
    0b0000100111001001,
    0b0000111100010101,
    0b0000000110011000,
    0b0000000010101001,
    0b0000110001110101,
    0b0000011101000110,
    0b0000011101111011,
    0b0000110100011101,
    0b0000011001100010,
    0b0000011011010100,
    0b0000001110111101,
    0b0000000100110001,
    0b0000100100000000,
    0b0000010001110010,
    0b0000110010010100,
    0b0000010100111011,
    0b0000111010010100,
    0b0000010010111010,
    0b0000011001100000,
    0b0000110101110111,
    0b0000001111100101,
    0b0000001011100000,
    0b0000010011001000,
    0b0000010011011100,
    0b0000000000111001,
    0b0000010101110000,
    0b0000001100011100,
    0b0000000010110011,
    0b0000111000101010,
    0b0000100000010010,
    0b0000001001011101,
    0b0000011100100001,
    0b0000110111101000,
    0b0000101110001001,
    0b0000001011000110,
    0b0000100110011010,
    0b0000101010000111,
    0b0000110110000110,
    0b0000001100101101,
    0b0000000000001011,
    0b0000110001110011,
    0b0000010001100111,
    0b0000010100011111,
    0b0000000000010001,
    0b0000001100110011,
    0b0000011010011110,
    0b0000101100111101,
    0b0000000111000110,
    0b0000101000111010,
    0b0000011011001011,
    0b0000011111000000,
    0b0000111011010110,
    0b0000000011001111,
    0b0000101100100000,
    0b0000100111010000,
    0b0000010000001010,
    0b0000001010001011,
    0b0000000011000010,
    0b0000111001110100,
    0b0000000011000011,
    0b0000000001101001,
    0b0000000000010010,
    0b0000100100010001,
    0b0000011010010111,
    0b0000110111011010,
    0b0000101101001110,
    0b0000001110010110,
    0b0000010100110001,
    0b0000101101010100,
    0b0000000000111100,
    0b0000110011111110,
    0b0000000010100101,
    0b0000100111010111,
    0b0000110111101111,
    0b0000111010011000,
    0b0000110000111100,
    0b0000101001011100,
    0b0000101000100101,
    0b0000100010001110,
    0b0000101111001100,
    0b0000101100111000,
    0b0000000100000110,
    0b0000001100100100,
    0b0000101011011011,
    0b0000111010010101,
    0b0000100011100101,
    0b0000001100000111,
    0b0000100111000001,
    0b0000010001011010,
    0b0000101101110100,
    0b0000100011011110,
    0b0000110001110000,
    0b0000111100101000,
    0b0000100111111011,
    0b0000110000101010,
    0b0000101100110100,
    0b0000001001000001,
    0b0000110100011100,
    0b0000010111110001,
    0b0000110111101010,
    0b0000100001110010,
    0b0000100000000011,
    0b0000000111100001,
    0b0000111010011011,
    0b0000010101011110,
    0b0000001000001101,
    0b0000010111010101,
    0b0000100100011000,
    0b0000111000110010,
    0b0000110001110100,
    0b0000111001101000,
    0b0000110000111110,
    0b0000010111001100,
    0b0000011101110011,
    0b0000101001101011,
    0b0000011011110000,
    0b0000000111010010,
    0b0000110101010111,
    0b0000011101000101,
    0b0000000100100101,
    0b0000011100011101,
    0b0000010000100100,
    0b0000000101111011,
    0b0000110111010010,
    0b0000110111100010,
    0b0000000110111100,
    0b0000011011010011,
    0b0000111111111011,
    0b0000001111000101,
    0b0000111110010110,
    0b0000010000011011,
    0b0000111110011101,
    0b0000100111101100,
    0b0000001000100011,
    0b0000111010000010,
    0b0000001001001100,
    0b0000010000010111,
    0b0000101010100011,
    0b0000111110010000,
    0b0000110100100001,
    0b0000001111000010,
    0b0000110010000000,
    0b0000111110110001,
    0b0000100001101110,
    0b0000101011010101,
    0b0000111100100011,
    0b0000001111000000,
    0b0000101111111011,
    0b0000001001010101,
    0b0000001110111000,
    0b0000101010001001,
    0b0000110111000000,
    0b0000010101110011,
    0b0000001110101110,
    0b0000000011010101,
    0b0000000110111011,
    0b0000000010001000,
    0b0000100000100011,
    0b0000001001001001,
    0b0000011101111111,
    0b0000100101100101,
    0b0000010011101110,
    0b0000101000101100,
    0b0000110011000101,
    0b0000001101111000,
    0b0000000001011000,
    0b0000010100100110,
    0b0000111011001000,
    0b0000110110010111,
    0b0000001000001011,
    0b0000111000101001,
    0b0000011010011100,
    0b0000111011010011,
    0b0000011001101111,
    0b0000011000111111,
    0b0000010100000000,
    0b0000000111010101,
    0b0000101111011010,
    0b0000000100011111,
    0b0000011000000100,
    0b0000101111110010,
    0b0000111110001100,
    0b0000101111111000,
    0b0000111000001101,
    0b0000100000001101,
    0b0000100011110001,
    0b0000100111110100,
    0b0000010100000111,
    0b0000000000010000,
    0b0000010110011001,
    0b0000111001100001,
    0b0000110001011011,
    0b0000010101100011,
    0b0000110011011010,
    0b0000110001011000,
    0b0000110111010011,
    0b0000101000011110,
    0b0000111100011111,
    0b0000101011000111,
    0b0000101000011001,
    0b0000000100000000,
    0b0000010111000011,
    0b0000110001000100,
    0b0000001010011010,
    0b0000010010011000,
    0b0000110100001000,
    0b0000010000101010,
    0b0000111110000001,
    0b0000110101000000,
    0b0000000001110101,
    0b0000000010101101,
    0b0000010100001111,
    0b0000010110100010,
    0b0000011111110111,
    0b0000111000001000,
    0b0000000110001010,
    0b0000000010100011,
    0b0000011110101001,
    0b0000000101000100,
    0b0000100101110011,
    0b0000111110011100,
    0b0000011101001010,
    0b0000001001001110,
    0b0000110101111111,
    0b0000000101011100,
    0b0000001100110111,
    0b0000010010000010,
    0b0000000111010111,
    0b0000111100001110,
    0b0000011110110011,
    0b0000110110000010,
    0b0000101110011111,
    0b0000100101011001,
    0b0000111001111111,
    0b0000111010110110,
    0b0000110110011000,
    0b0000110110111010,
    0b0000100100111010,
    0b0000001010101011,
    0b0000100011001000,
    0b0000101011101010,
    0b0000111100111110,
    0b0000100010101000,
    0b0000011011111001,
    0b0000010111111110,
    0b0000111111011010,
    0b0000100111110000,
    0b0000100100100011,
    0b0000011101011100,
    0b0000011101011011,
    0b0000010001000110,
    0b0000011100010100,
    0b0000110111110000,
    0b0000110110101101,
    0b0000110001001001,
    0b0000110101011011,
    0b0000101101100011,
    0b0000101101010000,
    0b0000111011001111,
    0b0000100111100100,
    0b0000100110000000,
    0b0000100011011010,
    0b0000010100000001,
    0b0000001100011101,
    0b0000010011100011,
    0b0000000101011111,
    0b0000000111010011,
    0b0000001100110101,
    0b0000011011001110,
    0b0000101111000100,
    0b0000111110111101,
    0b0000111111000100,
    0b0000001101111101,
    0b0000001111111111,
    0b0000110101001101,
    0b0000011111110001,
    0b0000111001010001,
    0b0000001000110001,
    0b0000010010110111,
    0b0000000111111111,
    0b0000000001011010,
    0b0000100101101110,
    0b0000100001111100,
    0b0000110110010000,
    0b0000110110101100,
    0b0000010110101011,
    0b0000011111101010,
    0b0000001110101010,
    0b0000001111001101,
    0b0000000001101010,
    0b0000000100111100,
    0b0000101111100100,
    0b0000000010011110,
    0b0000100110010100,
    0b0000111101101110,
    0b0000100011111101,
    0b0000100111100001,
    0b0000011110100101,
    0b0000111101100110,
    0b0000101001111010,
    0b0000101001001010,
    0b0000001000101111,
    0b0000001111001011,
    0b0000101000100011,
    0b0000110100100110,
    0b0000110010101111,
    0b0000000001000001,
    0b0000001011010111,
    0b0000111101011011,
    0b0000000100000101,
    0b0000111001001101,
    0b0000110001101001,
    0b0000010111000001,
    0b0000101110101110,
    0b0000110100010011,
    0b0000001011110000,
    0b0000101000001011,
    0b0000011110111101,
    0b0000001010011111,
    0b0000001101110101,
    0b0000100001110110,
    0b0000111111111111,
    0b0000010011011011,
    0b0000000111101000,
    0b0000011011011000,
    0b0000000000001101,
    0b0000101111100101,
    0b0000000101101010,
    0b0000010111001000,
    0b0000011110110100,
    0b0000110100110010,
    0b0000000110011110,
    0b0000001011110100,
    0b0000001111101010,
    0b0000110100111101,
    0b0000010101011011,
    0b0000010101101011,
    0b0000101011100011,
    0b0000011110011010,
    0b0000111111001111,
    0b0000001100110010,
    0b0000101111101111,
    0b0000110010001001,
    0b0000101101100101,
    0b0000000111000101,
    0b0000000101010001,
    0b0000011100111110,
    0b0000001000001001,
    0b0000101100100010,
    0b0000001110010000,
    0b0000111110000010,
    0b0000110110001010,
    0b0000010001010110,
    0b0000001101010011,
    0b0000100011010111,
    0b0000100011000001,
    0b0000001111011011,
    0b0000001100001110,
    0b0000000110001100,
    0b0000000001101000,
    0b0000111101010111,
    0b0000010011100101,
    0b0000010010111001,
    0b0000011100111001,
    0b0000001000000111,
    0b0000010001000100,
    0b0000000110010000,
    0b0000000100011110,
    0b0000110110101111,
    0b0000100111111110,
    0b0000011111110101,
    0b0000100101100000,
    0b0000110101100011,
    0b0000010100100100,
    0b0000110011110100,
    0b0000001001101010,
    0b0000111110100111,
    0b0000000010101000,
    0b0000001010011001,
    0b0000101111100011,
    0b0000010110100001,
    0b0000100101001111,
    0b0000001010010010,
    0b0000101010101110,
    0b0000101101101010,
    0b0000011100010011,
    0b0000100001001101,
    0b0000001010011100,
    0b0000000010111101,
    0b0000001110100011,
    0b0000101000110000,
    0b0000111011001101,
    0b0000100100001010,
    0b0000111011111000,
    0b0000001100111111,
    0b0000010001111000,
    0b0000001010101111,
    0b0000101001110001,
    0b0000010111111101,
    0b0000010000011101,
    0b0000000011101010,
    0b0000001100111100,
    0b0000100110001110,
    0b0000001101001010,
    0b0000000001100001,
    0b0000011001001111,
    0b0000101011000011,
    0b0000110111101101,
    0b0000111101101100,
    0b0000001010110001,
    0b0000110101000110,
    0b0000010111010110,
    0b0000110100111010,
    0b0000101010011101,
    0b0000001000101110,
    0b0000111111011101,
    0b0000111100011100,
    0b0000010101110101,
    0b0000011100011011,
    0b0000111101100000,
    0b0000011100101111,
    0b0000110110100001,
    0b0000101010010111,
    0b0000100101101111,
    0b0000000001001000,
    0b0000111001000110,
    0b0000001101110001,
    0b0000100001000101,
    0b0000110110011100,
    0b0000100101110100,
    0b0000111010101110,
    0b0000100111110101,
    0b0000000010101011,
    0b0000011100110110,
    0b0000111110100110,
    0b0000111100000111,
    0b0000101111010111,
    0b0000100001000011,
    0b0000001011000000,
    0b0000100010010101,
    0b0000111000101101,
    0b0000101010100010,
    0b0000111110111000,
    0b0000000101000001,
    0b0000101001110110,
    0b0000110001101010,
    0b0000100011100111,
    0b0000100111001101,
    0b0000001100001001,
    0b0000010111110011,
    0b0000000011111101,
    0b0000010011100100,
    0b0000110101110011,
    0b0000101110101100,
    0b0000111100111000,
    0b0000101100011000,
    0b0000010101010000,
    0b0000001110000000,
    0b0000000000001100,
    0b0000010011111010,
    0b0000100101010100,
    0b0000010011110000,
    0b0000100011110000,
    0b0000000110011101,
    0b0000111111110010,
    0b0000100111100000,
    0b0000100001011110,
    0b0000111011000110,
    0b0000111001011011,
    0b0000110110100111,
    0b0000100101110001,
    0b0000111010100001,
    0b0000101101011000,
    0b0000100000100000,
    0b0000011000100110,
    0b0000100000100010,
    0b0000100010011100,
    0b0000001000011100,
    0b0000111111010010,
    0b0000000011011101,
    0b0000011001011110,
    0b0000000101100010,
    0b0000000100010000,
    0b0000110100111100,
    0b0000111011011001,
    0b0000100011010101,
    0b0000110010000100,
    0b0000001101100101,
    0b0000001001110100,
    0b0000101011011111,
    0b0000010000111000,
    0b0000111011100101,
    0b0000000010000100,
    0b0000010011110100,
    0b0000111000010101,
    0b0000000011111010,
    0b0000111100000001,
    0b0000011110010010,
    0b0000101111101100,
    0b0000000101011001,
    0b0000100110101100,
    0b0000000111110011,
    0b0000101110000011,
    0b0000011000101100,
    0b0000010001101011,
    0b0000111101000100,
    0b0000001010000110,
    0b0000111101111100,
    0b0000101000110001,
    0b0000010011010011,
    0b0000011111100111,
    0b0000110001011010,
    0b0000111110110011,
    0b0000110001011100,
    0b0000110010001011,
    0b0000110101101100,
    0b0000001001011110,
    0b0000110000010101,
    0b0000001100000010,
    0b0000111001010111,
    0b0000101001010010,
    0b0000101011101100,
    0b0000000110101011,
    0b0000101101101011,
    0b0000111100111011,
    0b0000111111000001,
    0b0000110100101010,
    0b0000010010110101,
    0b0000111101010101,
    0b0000000011100011,
    0b0000111000100100,
    0b0000000101001100,
    0b0000111100001100,
    0b0000110010001111,
    0b0000110010011001,
    0b0000110011111011,
    0b0000100101110010,
    0b0000100010010110,
    0b0000111001001000,
    0b0000010010010110,
    0b0000001000000110,
    0b0000111110011010,
    0b0000010100001100,
    0b0000111011011000,
    0b0000111011001110,
    0b0000100111010001,
    0b0000100110101000,
    0b0000000010100001,
    0b0000101111101001,
    0b0000100001100100,
    0b0000100000100100,
    0b0000010000100000,
    0b0000011111111000,
    0b0000111011111101,
    0b0000000001011001,
    0b0000000001011101,
    0b0000011000011111,
    0b0000111101111000,
    0b0000110000011110,
    0b0000011100110001,
    0b0000101001000001,
    0b0000111000111110,
    0b0000110111111101,
    0b0000101010001011,
    0b0000110101011001,
    0b0000001111100000,
    0b0000100100110000,
    0b0000100111000101,
    0b0000111011101111,
    0b0000010000010011,
    0b0000100101100111,
    0b0000100000110111,
    0b0000010101011000,
    0b0000111110110101,
    0b0000010000101111,
    0b0000110011001100,
    0b0000011010011011,
    0b0000000001100100,
    0b0000101110110110,
    0b0000010000101011,
    0b0000110101100001,
    0b0000000001011100,
    0b0000000100011101,
    0b0000100001000100,
    0b0000011110100001,
    0b0000010111001010,
    0b0000011000110111,
    0b0000011010001000,
    0b0000011000111001,
    0b0000010000010000,
    0b0000010001101100,
    0b0000010110111010,
    0b0000000000010110,
    0b0000101101111001,
    0b0000110000111111,
    0b0000111110010100,
    0b0000110110100010,
    0b0000101100001001,
    0b0000001000010011,
    0b0000011101101001,
    0b0000010111011010,
    0b0000001110110001,
    0b0000011110110111,
    0b0000111101100100,
    0b0000101011010111,
    0b0000011111011101,
    0b0000010000010101,
    0b0000111101001011,
    0b0000000110001000,
    0b0000010110101110,
    0b0000101001100100,
    0b0000101010010010,
    0b0000110010010000,
    0b0000101101001011,
    0b0000100110111001,
    0b0000010000000001,
    0b0000000111001011,
    0b0000011011101000,
    0b0000110110010100,
    0b0000110011111001,
    0b0000101110111010,
    0b0000111010001010,
    0b0000000010110010,
    0b0000000011000111,
    0b0000101000100111,
    0b0000101000000100,
    0b0000111110100000,
    0b0000110100110011,
    0b0000011100010000,
    0b0000100000001010,
    0b0000100000111101,
    0b0000111011010000,
    0b0000000001110001,
    0b0000111011000000,
    0b0000110100100100,
    0b0000010001001101,
    0b0000011100100100,
    0b0000011000101001,
    0b0000101011010110,
    0b0000001011001101,
    0b0000110100001101,
    0b0000011110001101,
    0b0000001010101110,
    0b0000011001000100,
    0b0000000011111000,
    0b0000111100001011,
    0b0000001111011001,
    0b0000010100000110,
    0b0000111001101110,
    0b0000001100011000,
    0b0000100001101001,
    0b0000101000101111,
    0b0000101100111011,
    0b0000100011111111,
    0b0000011100000100,
    0b0000000000001000,
    0b0000100100100010,
    0b0000111000110001,
    0b0000001100111110,
    0b0000001110011100,
    0b0000100010010111,
    0b0000100001000010,
    0b0000000111110100,
    0b0000010001010010,
    0b0000110111001011,
    0b0000010111110101,
    0b0000010110000010,
    0b0000110100010111,
    0b0000011000001110,
    0b0000011011011100,
    0b0000110010011010,
    0b0000101010101101,
    0b0000100111011111,
    0b0000111111101101,
    0b0000000010001001,
    0b0000000000110001,
    0b0000000100111011,
    0b0000111100110101,
    0b0000001011000100,
    0b0000111111110100,
    0b0000011001010111,
    0b0000000000000000,
    0b0000011100000011,
    0b0000000001000100,
    0b0000101000011100,
    0b0000101010011001,
    0b0000101011111000,
    0b0000011111001010,
    0b0000111110010101,
    0b0000001010110110,
    0b0000001011110110,
    0b0000110110110001,
    0b0000110100111111,
    0b0000000100110000,
    0b0000101000100001,
    0b0000001101000011,
    0b0000111110011110,
    0b0000000101101100,
    0b0000000011111011,
    0b0000100101100110,
    0b0000011101010111,
    0b0000101101000100,
    0b0000110000101110,
    0b0000110111111000,
    0b0000101111001011,
    0b0000111001101010,
    0b0000000010000110,
    0b0000001011011001,
    0b0000011010110010,
    0b0000010001110100,
    0b0000001011010000,
    0b0000101101000011,
    0b0000111111010100,
    0b0000011001110101,
    0b0000011011111000,
    0b0000010101010100,
    0b0000001000101100,
    0b0000111000000010,
    0b0000100000111011,
    0b0000100000000100,
    0b0000000000000110,
    0b0000100100111110,
    0b0000011110000110,
    0b0000100111001100,
    0b0000111010111000,
    0b0000001011010011,
    0b0000101100001101,
    0b0000110000001010,
    0b0000110101111100,
    0b0000101010111011,
    0b0000001100111101,
    0b0000101100111010,
    0b0000101010000011,
    0b0000011111001100,
    0b0000100011111110,
    0b0000010010011110,
    0b0000110001110110,
    0b0000010011011000,
    0b0000001001010100,
    0b0000101110001011,
    0b0000000101110101,
    0b0000101000000011,
    0b0000001011011110,
    0b0000101010000100,
    0b0000111111111001,
    0b0000101111001010,
    0b0000011010100101,
    0b0000011001001011,
    0b0000110110110101,
    0b0000010000111011,
    0b0000100001010011,
    0b0000010111100000,
    0b0000011000010100,
    0b0000001010011110,
    0b0000100101011101,
    0b0000001010000011,
    0b0000111011010010,
    0b0000000101111001,
    0b0000011011001000,
    0b0000101000010011,
    0b0000000111111001,
    0b0000110111010111,
    0b0000001100001011,
    0b0000001111001010,
    0b0000001110100110,
    0b0000101010011100,
    0b0000001100010110,
    0b0000101110111100,
    0b0000010110000011,
    0b0000000100101001,
    0b0000111111000010,
    0b0000000010110110,
    0b0000001111000110,
    0b0000010001011110,
    0b0000000101001010,
    0b0000011001100110,
    0b0000101010100000,
    0b0000010110110111,
    0b0000100010110010,
    0b0000101100001100,
    0b0000110010000001,
    0b0000101111101011,
    0b0000011100011001,
    0b0000010101000000,
    0b0000101010111010,
    0b0000110101110010,
    0b0000111101100001,
    0b0000111000100110,
    0b0000110101001001,
    0b0000110100110100,
    0b0000001110100010,
    0b0000001011011011,
    0b0000111011100110,
    0b0000010100111000,
    0b0000100100100111,
    0b0000011000000111,
    0b0000101110111000,
    0b0000011110000011,
    0b0000010011101011,
    0b0000101000001001,
    0b0000011101100100,
    0b0000000110110000,
    0b0000010100001010,
    0b0000011110010011,
    0b0000101010100110,
    0b0000101100110000,
    0b0000011110000100,
    0b0000111000010111,
    0b0000100100001000,
    0b0000000110001001,
    0b0000010000101100,
    0b0000010111101100,
    0b0000110101100100,
    0b0000100000111001,
    0b0000010101101001,
    0b0000101000001100,
    0b0000010100110011,
    0b0000100101000111,
    0b0000101001111111,
    0b0000000110011100,
    0b0000000101000110,
    0b0000001101000000,
    0b0000010101011101,
    0b0000110111011100,
    0b0000111010010110,
    0b0000111111000101,
    0b0000100111000110,
    0b0000100011001100,
    0b0000011000110100,
    0b0000011011011001,
    0b0000001000010111,
    0b0000100011101110,
    0b0000110011101111,
    0b0000001000110010,
    0b0000110010101110,
    0b0000001110011110,
    0b0000010110110100,
    0b0000000010011000,
    0b0000011111101000,
    0b0000100000001110,
    0b0000110100101110,
    0b0000001110110111,
    0b0000100110111010,
    0b0000101110101011,
    0b0000000110000001,
    0b0000010111110100,
    0b0000011111111011,
    0b0000110001100011,
    0b0000000101100100,
    0b0000100110011110,
    0b0000011000101010,
    0b0000011111100001,
    0b0000000000111110,
    0b0000010010110110,
    0b0000110101111110,
    0b0000101110011011,
    0b0000101001011000,
    0b0000000101010011,
    0b0000011110100000,
    0b0000001100010100,
    0b0000000011010010,
    0b0000001110100111,
    0b0000011010000000,
    0b0000001011100010,
    0b0000111100011000,
    0b0000100100100001,
    0b0000101010110101,
    0b0000100101010111,
    0b0000100010010100,
    0b0000111011011110,
    0b0000100001010101,
    0b0000011111111110,
    0b0000001011001001,
    0b0000100111010101,
    0b0000001000011010,
    0b0000100110111101,
    0b0000001111011110,
    0b0000101011100010,
    0b0000010110001111,
    0b0000100000101110,
    0b0000001000000011,
    0b0000111101100010,
    0b0000100110110011,
    0b0000100011001011,
    0b0000010101110110,
    0b0000000111111000,
    0b0000011110111111,
    0b0000100000010001,
    0b0000011001101101,
    0b0000100111001011,
    0b0000000010000111,
    0b0000110000101011,
    0b0000010000110101,
    0b0000101010111111,
    0b0000111110001110,
    0b0000111101110001,
    0b0000100100100000,
    0b0000000001110110,
    0b0000001101010000,
    0b0000111011100010,
    0b0000110001111110,
    0b0000100011001110,
    0b0000001001100001,
    0b0000000111111100,
    0b0000000100100000,
    0b0000000000101001,
    0b0000101011000001,
];
