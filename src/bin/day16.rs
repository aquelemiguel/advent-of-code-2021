use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/d16-f").expect("Error while reading");
    let bits = hex_to_binary_str(&input);

    let mut packets: Vec<u32> = vec![];
    parse_packet(&bits, &mut packets);

    println!("P1: {}", packets.iter().sum::<u32>());
}

fn parse_packet<'a, 'b>(bits: &'a str, packets: &'b mut Vec<u32>) -> &'a str {
    let version = u32::from_str_radix(&bits[0..3], 2).unwrap();
    let type_id = u32::from_str_radix(&bits[3..6], 2).unwrap();

    packets.push(version);

    if type_id == 4 {
        parse_literal_packet(&bits[6..])
    } else {
        parse_operator_packet(&bits[6..], packets)
    }
}

fn parse_operator_packet<'a, 'b>(bits: &'a str, packets: &'b mut Vec<u32>) -> &'a str {
    let mode = bits.chars().next().unwrap();
    let mut remaining;

    if mode == '0' {
        let subpacket_len = u32::from_str_radix(&bits[1..=15], 2).unwrap() as usize;
        let mut processed_bits = 0;

        remaining = &bits[16..];

        while processed_bits < subpacket_len {
            let curr_remaining = parse_packet(remaining, packets);
            processed_bits += remaining.len() - curr_remaining.len();
            remaining = &curr_remaining;
        }
    } else {
        let subpackets = u32::from_str_radix(&bits[1..=11], 2).unwrap();
        remaining = &bits[12..];

        for _ in 0..subpackets {
            remaining = &parse_packet(remaining, packets);
        }
    }

    remaining
}

fn parse_literal_packet(bits: &str) -> &str {
    let chunks = &bits
        .chars()
        .chunks(5)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect_vec();

    let mut packet_size = 0;
    let mut value = String::new();

    for chunk in chunks.iter() {
        value.push_str(&chunk[1..5]);
        packet_size += 5;

        if chunk.starts_with('0') {
            break;
        }
    }

    &bits[packet_size..]
}

fn hex_to_binary_str(hex: &str) -> String {
    hex.chars()
        .map(|c| {
            let num = u64::from_str_radix(&c.to_string(), 16).unwrap();
            format!("{:04b}", num)
        })
        .collect_vec()
        .join("")
}
