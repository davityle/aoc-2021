use itertools::Itertools;
use num::bigint::ToBigUint;
use num::{BigUint, One, Zero};
use std::collections::HashMap;

fn main() {
    let replacement_map: HashMap<char, &str> = HashMap::from_iter(HEX_REPLACEMENT.into_iter());

    let binary_chars = INPUT
        .chars()
        .into_iter()
        .flat_map(|c| replacement_map.get(&c))
        .join("")
        .chars()
        .into_iter()
        .collect::<Vec<char>>();
    let mut index = 0;

    let packet_result = evaluate_packet(&binary_chars, &mut index);

    println!("{:?}", packet_result);
}

fn evaluate_packet(chars: &Vec<char>, mut index: &mut usize) -> BigUint {
    let _version: u8 = take(3, chars, &mut index);
    let id: u8 = take(3, chars, &mut index);
    let packet = match id {
        4 => {
            let mut literal_val: BigUint = BigUint::zero();
            let mut continue_bit: u8 = 1;
            while continue_bit == 1 {
                continue_bit = take(1, chars, &mut index);
                literal_val = literal_val << 4;
                let bits: u8 = take(4, chars, &mut index);
                literal_val = literal_val | bits.to_biguint().unwrap();
            }
            literal_val
        }
        0 => evaluate_sub_packets(chars, &mut index)
            .into_iter()
            .fold(BigUint::zero(), |a, b| a + b),
        1 => evaluate_sub_packets(chars, &mut index)
            .into_iter()
            .fold(BigUint::one(), |a, b| a * b),
        2 => evaluate_sub_packets(chars, &mut index).into_iter().fold(
            u32::MAX.to_biguint().unwrap(),
            |a, b| if a < b { a } else { b },
        ),
        3 => evaluate_sub_packets(chars, &mut index)
            .into_iter()
            .fold(BigUint::zero(), |a, b| if a > b { a } else { b }),
        5 => {
            let packets = evaluate_sub_packets(chars, &mut index);
            if packets[0] > packets[1] {
                BigUint::one()
            } else {
                BigUint::zero()
            }
        }
        6 => {
            let packets = evaluate_sub_packets(chars, &mut index);
            if packets[0] < packets[1] {
                BigUint::one()
            } else {
                BigUint::zero()
            }
        }
        7 => {
            let packets = evaluate_sub_packets(chars, &mut index);
            if packets[0] == packets[1] {
                BigUint::one()
            } else {
                BigUint::zero()
            }
        }
        _ => 0.to_biguint().unwrap(),
    };
    packet
}

fn evaluate_sub_packets(chars: &Vec<char>, mut index: &mut usize) -> Vec<BigUint> {
    let packet_mode: u8 = take(1, chars, &mut index);
    if packet_mode == 0 {
        let packet_bit_length: usize = take(15, chars, &mut index);
        let final_index_size = *index + packet_bit_length;
        let mut sub_packets: Vec<BigUint> = Vec::new();
        while *index < final_index_size {
            sub_packets.push(evaluate_packet(chars, &mut index));
        }
        sub_packets
    } else {
        let sub_packet_length: usize = take(11, chars, &mut index);
        let mut sub_packets: Vec<BigUint> = Vec::new();
        while sub_packets.len() < sub_packet_length {
            sub_packets.push(evaluate_packet(chars, &mut index));
        }
        sub_packets
    }
}

fn take<T>(count: usize, chars: &Vec<char>, index: &mut usize) -> T
where
    T: num::PrimInt + num::Unsigned + std::fmt::Binary,
{
    let mut num = T::zero();
    for i in (0..count).rev() {
        num = num | to_num::<T>(chars[*index + (count - (i + 1))]) << i;
    }
    *index += count;
    num
}

fn to_num<T: num::PrimInt>(c: char) -> T {
    match c {
        '0' => T::zero(),
        '1' => T::one(),
        _ => T::one(),
    }
}

const HEX_REPLACEMENT: [(char, &str); 16] = [
    ('0', "0000"),
    ('1', "0001"),
    ('2', "0010"),
    ('3', "0011"),
    ('4', "0100"),
    ('5', "0101"),
    ('6', "0110"),
    ('7', "0111"),
    ('8', "1000"),
    ('9', "1001"),
    ('A', "1010"),
    ('B', "1011"),
    ('C', "1100"),
    ('D', "1101"),
    ('E', "1110"),
    ('F', "1111"),
];

const _: &str = "D2FE28";
const _: &str = "38006F45291200";
const _: &str = "EE00D40C823060";
const _: &str = "8A004A801A8002F478";
const _: &str = "620080001611562C8802118E34";
const _: &str = "C0015000016115A2E0802F182340";
const _: &str = "A0016C880162017C3686B18A3D4780";

const _: &str = "C200B40A82";
const _: &str = "04005AC33890";
const _: &str = "880086C3E88112";
const _: &str = "CE00C43D881120";
const _: &str = "D8005AC2A8F0";
const _: &str = "F600BC2D8F";
const _: &str = "9C005AC2F8F0";
const _: &str = "9C0141080250320F1802104A08";

const INPUT: &str = "A20D5CECBD6C061006E7801224AF251AEA06D2319904921880113A931A1402A9D83D43C9FFCC1E56FF29890E00C42984337BF22C502008C26982801009426937320124E602BC01192F4A74FD7B70692F4A74FD7B700403170400F7002DC00E7003C400B0023700082C601DF8C00D30038005AA0013F40044E7002D400D10030C008000574000AB958B4B8011074C0249769913893469A72200B42673F26A005567FCC13FE673004F003341006615421830200F4608E7142629294F92861A840118F1184C0129637C007C24B19AA2C96335400013B0C0198F716213180370AE39C7620043E0D4788B440232CB34D80260008645C86D16C401B85D0BA2D18025A00ACE7F275324137FD73428200ECDFBEFF2BDCDA70D5FE5339D95B3B6C98C1DA006772F9DC9025B057331BF7D8B65108018092599C669B4B201356763475D00480010E89709E090002130CA28C62300265C188034BA007CA58EA6FB4CDA12799FD8098021400F94A6F95E3ECC73A77359A4EFCB09CEF799A35280433D1BCCA666D5EFD6A5A389542A7DCCC010958D85EC0119EED04A73F69703669466A048C01E14FFEFD229ADD052466ED37BD8B4E1D10074B3FF8CF2BBE0094D56D7E38CADA0FA80123C8F75F9C764D29DA814E4693C4854C0118AD3C0A60144E364D944D02C99F4F82100607600AC8F6365C91EC6CBB3A072C404011CE8025221D2A0337158200C97001F6978A1CE4FFBE7C4A5050402E9ECEE709D3FE7296A894F4C6A75467EB8959F4C013815C00FACEF38A7297F42AD2600B7006A0200EC538D51500010B88919624CE694C0027B91951125AFF7B9B1682040253D006E8000844138F105C0010D84D1D2304B213007213900D95B73FE914CC9FCBFA9EEA81802FA0094A34CA3649F019800B48890C2382002E727DF7293C1B900A160008642B87312C0010F8DB08610080331720FC580";
