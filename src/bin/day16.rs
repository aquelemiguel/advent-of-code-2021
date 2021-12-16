use itertools::Itertools;

#[derive(Debug)]
struct Packet {
    version: usize,
}

fn main() {
    let input = std::fs::read_to_string("input/d16-e").expect("Error while reading");
    let bits = hex_to_binary_str(&input);

    parse_packet(&bits);
}

fn parse_header(bits: &str) -> (u32, u32) {
    (
        u32::from_str_radix(&bits[0..3], 2).unwrap(),
        u32::from_str_radix(&bits[3..6], 2).unwrap(),
    )
}

fn parse_packet(bits: &str) -> &str {
    let (version, type_id) = parse_header(bits);
    println!("VERSION: {}", version);

    if type_id == 4 {
        parse_literal_packet(&bits[6..])
    } else {
        parse_operator_packet(&bits[6..])
    }
}

fn parse_operator_packet(bits: &str) -> &str {
    let mode = bits.chars().next().unwrap();
    let mut remaining;

    // println!("bits: {}", bits);

    if mode == '0' {
        let subpacket_len = u32::from_str_radix(&bits[1..=15], 2).unwrap() as usize;
        let mut processed_bits = 0;

        remaining = &bits[16..];

        while processed_bits < subpacket_len {
            // println!("REMAINING = {}", remaining);
            let curr_remaining = parse_packet(remaining);
            processed_bits += remaining.len() - curr_remaining.len();
            remaining = curr_remaining;
        }
    } else {
        let subpackets = u32::from_str_radix(&bits[1..=11], 2).unwrap();

        remaining = &bits[12..];

        for _ in 0..subpackets {
            // println!("REMAINING = {}", remaining);
            remaining = parse_packet(remaining);
        }
    }

    remaining
}

fn parse_literal_packet(bits: &str) -> &str {
    // println!("PROCESSING = {}", bits);

    let chunks = &bits
        .chars()
        .chunks(5)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect_vec();

    // println!("{:?}", chunks);

    let mut packet_size = 0;
    let mut value = String::new();

    for (i, chunk) in chunks.iter().enumerate() {
        // println!("{} {}", i, chunk);
        value.push_str(&chunk[1..5]);
        packet_size += 5;

        if chunk.starts_with('0') {
            break;
        }
    }

    // println!("PACKET SIZE: {}", packet_size);

    // let value = u32::from_str_radix(&value, 2).unwrap();
    // println!("VALUE: {}", value);

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
