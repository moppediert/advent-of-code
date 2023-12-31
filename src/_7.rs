use std::collections::HashMap;

pub fn _1(input: String) {
    let _input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let mut hands = vec![vec![]; 7];
    let mut bids = vec![];
    for line in input.lines() {
        let (hand, bid) = line.trim().split_once(" ").unwrap();
        let hand_type = hand_type(hand);
        hands[hand_type.0].push((hand, bid.parse::<usize>().unwrap()));
        bids.push(bid.parse::<usize>().unwrap());
    }

    let label_strength = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    for hand in hands.iter_mut() {
        hand.sort_by(|a, b| {
            for (_a, _b) in a.0.chars().zip(b.0.chars()) {
                let cmp = label_strength
                    .get(&_a)
                    .unwrap()
                    .partial_cmp(label_strength.get(&_b).unwrap());
                if cmp != Some(std::cmp::Ordering::Equal) {
                    return cmp.unwrap();
                }
            }
            std::cmp::Ordering::Equal
        })
    }

    let result = hands
        .iter()
        .flatten()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i + 1) * bid);

    println!("{:#?}", result);
    assert!(result == 251029473);
}

fn hand_type(hand: &str) -> (usize, usize) {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in hand.chars() {
        map.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }

    let score_map: &[usize] = &[15, 18, 21, 33, 36, 84, 243];

    let score = map
        .into_values()
        .fold(0, |acc, v| acc + 3_usize.pow(v.try_into().unwrap()));
    let index = score_map.iter().position(|x| *x == score).unwrap();
    (index, score)
}

pub fn _2(input: String) {
    let _input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let mut hands = vec![];
    for line in input.lines() {
        let (hand, bid) = line.trim().split_once(" ").unwrap();
        let score = hand_type_2(hand);
        hands.push((hand, score, bid.parse::<usize>().unwrap()));
    }

    let label_strength = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    hands.sort_by(|a, b| {
        if a.1.partial_cmp(&b.1) != Some(std::cmp::Ordering::Equal) {
            return a.1.partial_cmp(&b.1).unwrap();
        }
        for (_a, _b) in a.0.chars().zip(b.0.chars()) {
            let cmp = label_strength
                .get(&_a)
                .unwrap()
                .partial_cmp(label_strength.get(&_b).unwrap());
            if cmp != Some(std::cmp::Ordering::Equal) {
                return cmp.unwrap();
            }
        }
        println!("should not be here");
        std::cmp::Ordering::Equal
    });

    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, _, bid))| acc + (i + 1) * bid);

    println!("{:#?}", result);
    assert!(result == 251003917);
}

fn hand_type_2(hand: &str) -> usize {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in hand.chars() {
        map.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }

    let score_map: &[usize] = &[
        15,  // high card
        18,  // one pair
        21,  // two pair
        33,  // three of a kind
        36,  // full house
        84,  // four of a kind
        243, // five of a kind
    ];
    let num_j = hand.matches(|x| x == 'J').count();
    let score = map
        .into_values()
        .fold(0, |acc, v| acc + 3_usize.pow(v.try_into().unwrap()));
    let index = score_map.iter().position(|x| *x == score).unwrap();
    let index = match (index, num_j) {
        (0, 1) => index + 1,
        (1, 2) => index + 2,
        (3, 3) => index + 2,
        (5, 4) => index + 1,
        (6, 5) => index,
        (1, 1) => index + 2,
        (2, 2) => index + 3,
        (4, 3) => index + 2,
        (3, 1) => index + 2,
        (4, 2) => index + 2,
        (5, 1) => index + 1,
        (2, 1) => index + 2,
        _ => index,
    };
    let score = score_map[index];
    score
}
