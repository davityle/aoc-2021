use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let sequence_pairs: HashMap<(u8, u8), u8> = HashMap::from_iter(
        INPUT_PAIRS
            .into_iter()
            .map(|((c1, c2), c3)| ((c1 as u8, c2 as u8), c3 as u8)),
    );
    let iter_sequence: Vec<u8> = INPUT_SEQUENCE.chars().map(|c| c as u8).collect();

    let mut pair_counts: HashMap<(u8, u8), u64> =
        HashMap::from_iter(sequence_pairs.iter().map(|(pair, _)| ((pair.0, pair.1), 0)));
    let mut last_pair: (u8, u8) = (
        iter_sequence[iter_sequence.len() - 2],
        iter_sequence[iter_sequence.len() - 1],
    );

    for index in 0..(iter_sequence.len() - 1) {
        let pair = (iter_sequence[index], iter_sequence[index + 1]);
        if let Some(count) = pair_counts.get_mut(&pair) {
            *count += 1;
        }
    }

    for _ in 0..40 {
        let mut replaced = false;
        let mut new_counts: HashMap<(u8, u8), u64> =
            HashMap::from_iter(sequence_pairs.iter().map(|(pair, _)| ((pair.0, pair.1), 0)));
        for (pair, to_add_count) in pair_counts.into_iter() {
            if let Some(&replacement) = sequence_pairs.get(&pair) {
                let replacement_pair_0 = (pair.0, replacement);
                let replacement_pair_1 = (replacement, pair.1);
                if !replaced && pair.0 == last_pair.0 && pair.1 == last_pair.1 {
                    replaced = true;
                    last_pair = replacement_pair_1;
                }
                if let Some(count) = new_counts.get_mut(&replacement_pair_0) {
                    *count += to_add_count;
                }
                if let Some(count) = new_counts.get_mut(&replacement_pair_1) {
                    *count += to_add_count;
                }
            }
        }
        pair_counts = new_counts;
    }
    let character_counts = get_character_counts(&pair_counts, &last_pair);
    let most_frequent =
        character_counts
            .clone()
            .into_iter()
            .fold((0, 0), |p1, p2| if p1.1 > p2.1 { p1 } else { p2 });
    let least_frequent =
        character_counts
            .into_iter()
            .fold((0, u64::MAX), |p1, p2| if p1.1 < p2.1 { p1 } else { p2 });

    println!("{:?}", most_frequent.1 - least_frequent.1);
}

fn get_character_counts(
    pair_counts: &HashMap<(u8, u8), u64>,
    last_pair: &(u8, u8),
) -> HashMap<u8, u64> {
    let mut character_counts: HashMap<u8, u64> = HashMap::new();
    for (character, count) in pair_counts.iter().map(|((c1, _), count)| (*c1, *count)) {
        match character_counts.entry(character) {
            Entry::Vacant(e) => {
                e.insert(count);
            }
            Entry::Occupied(mut e) => *e.get_mut() += count,
        }
    }
    match character_counts.entry(last_pair.1) {
        Entry::Vacant(e) => {
            e.insert(1);
        }
        Entry::Occupied(mut e) => *e.get_mut() += 1,
    }
    character_counts
}

const _: &str = "NNCB";
const _: [((char, char), char); 16] = [
    (('C', 'H'), 'B'),
    (('H', 'H'), 'N'),
    (('C', 'B'), 'H'),
    (('N', 'H'), 'C'),
    (('H', 'B'), 'C'),
    (('H', 'C'), 'B'),
    (('H', 'N'), 'C'),
    (('N', 'N'), 'C'),
    (('B', 'H'), 'H'),
    (('N', 'C'), 'B'),
    (('N', 'B'), 'B'),
    (('B', 'N'), 'B'),
    (('B', 'B'), 'N'),
    (('B', 'C'), 'B'),
    (('C', 'C'), 'N'),
    (('C', 'N'), 'C'),
];

const INPUT_SEQUENCE: &str = "PPFCHPFNCKOKOSBVCFPP";
const INPUT_PAIRS: [((char, char), char); 100] = [
    (('V', 'C'), 'N'),
    (('S', 'C'), 'H'),
    (('C', 'K'), 'P'),
    (('O', 'K'), 'O'),
    (('K', 'V'), 'O'),
    (('H', 'S'), 'B'),
    (('O', 'H'), 'O'),
    (('V', 'N'), 'F'),
    (('F', 'S'), 'S'),
    (('O', 'N'), 'B'),
    (('O', 'S'), 'H'),
    (('P', 'C'), 'B'),
    (('B', 'P'), 'O'),
    (('O', 'O'), 'N'),
    (('B', 'F'), 'K'),
    (('C', 'N'), 'B'),
    (('F', 'K'), 'F'),
    (('N', 'P'), 'K'),
    (('K', 'K'), 'H'),
    (('C', 'B'), 'S'),
    (('C', 'V'), 'K'),
    (('V', 'S'), 'F'),
    (('S', 'F'), 'N'),
    (('K', 'B'), 'H'),
    (('K', 'N'), 'F'),
    (('C', 'P'), 'V'),
    (('B', 'O'), 'N'),
    (('S', 'S'), 'O'),
    (('H', 'F'), 'H'),
    (('N', 'N'), 'F'),
    (('P', 'P'), 'O'),
    (('V', 'P'), 'H'),
    (('B', 'B'), 'K'),
    (('V', 'B'), 'N'),
    (('O', 'F'), 'N'),
    (('S', 'H'), 'S'),
    (('P', 'O'), 'F'),
    (('O', 'C'), 'S'),
    (('N', 'S'), 'C'),
    (('F', 'H'), 'N'),
    (('F', 'P'), 'C'),
    (('S', 'O'), 'P'),
    (('V', 'K'), 'C'),
    (('H', 'P'), 'O'),
    (('P', 'V'), 'S'),
    (('H', 'N'), 'K'),
    (('N', 'B'), 'C'),
    (('N', 'V'), 'K'),
    (('N', 'K'), 'B'),
    (('F', 'N'), 'C'),
    (('V', 'V'), 'N'),
    (('B', 'N'), 'N'),
    (('B', 'H'), 'S'),
    (('F', 'O'), 'V'),
    (('P', 'K'), 'N'),
    (('P', 'S'), 'O'),
    (('C', 'O'), 'K'),
    (('N', 'O'), 'K'),
    (('S', 'V'), 'C'),
    (('K', 'O'), 'V'),
    (('H', 'C'), 'B'),
    (('B', 'C'), 'N'),
    (('P', 'B'), 'C'),
    (('S', 'K'), 'S'),
    (('F', 'V'), 'K'),
    (('H', 'O'), 'O'),
    (('C', 'F'), 'O'),
    (('H', 'B'), 'P'),
    (('S', 'P'), 'N'),
    (('V', 'H'), 'P'),
    (('N', 'C'), 'K'),
    (('K', 'C'), 'B'),
    (('O', 'V'), 'P'),
    (('B', 'K'), 'F'),
    (('F', 'B'), 'F'),
    (('F', 'F'), 'V'),
    (('C', 'S'), 'F'),
    (('C', 'C'), 'H'),
    (('S', 'B'), 'C'),
    (('V', 'O'), 'V'),
    (('V', 'F'), 'O'),
    (('K', 'P'), 'N'),
    (('H', 'V'), 'H'),
    (('P', 'F'), 'H'),
    (('K', 'H'), 'P'),
    (('K', 'S'), 'S'),
    (('B', 'S'), 'H'),
    (('P', 'H'), 'S'),
    (('S', 'N'), 'K'),
    (('H', 'K'), 'P'),
    (('F', 'C'), 'N'),
    (('P', 'N'), 'S'),
    (('H', 'H'), 'N'),
    (('O', 'B'), 'P'),
    (('B', 'V'), 'S'),
    (('K', 'F'), 'N'),
    (('O', 'P'), 'H'),
    (('N', 'F'), 'V'),
    (('C', 'H'), 'K'),
    (('N', 'H'), 'P'),
];
