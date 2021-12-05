use std::fs;

#[derive(Debug)]
struct BingoCell {
    value: i32,
    marked: bool,
}

type BingoCard = Vec<Vec<BingoCell>>;

fn main() {
    let (cards, cage) = read_input("input/d04-example");
    let winners = bingo(cards, &cage);

    println!("P1: {}\nP2: {}", &winners[0], winners.last().unwrap());
}

fn bingo(mut cards: Vec<BingoCard>, cage: &Vec<i32>) -> Vec<i32> {
    let mut winners: Vec<i32> = vec![];

    for draw in cage.iter() {
        for card in cards.iter_mut() {
            fill_card(card, draw);
        }

        let (w, l): (Vec<BingoCard>, Vec<BingoCard>) =
            cards.into_iter().partition(|card| is_card_winner(card));

        cards = l;

        let mut scores: Vec<i32> = w.iter().map(|card| calculate_score(card, draw)).collect();
        winners.append(&mut scores);
    }

    winners
}

fn calculate_score(card: &BingoCard, latest_draw: &i32) -> i32 {
    let unmarked = card.iter().flatten().filter(|f| !f.marked).map(|f| f.value);
    unmarked.sum::<i32>() * latest_draw
}

fn is_card_winner(card: &BingoCard) -> bool {
    (0..5).any(|i| card[i].iter().all(|cell| cell.marked))
        || (0..5).any(|i| card.iter().all(|row| row[i].marked))
}

fn fill_card(card: &mut BingoCard, draw: &i32) {
    if let Some(cell) = find_cell(card, draw) {
        cell.marked = true
    }
}

fn find_cell<'a>(card: &'a mut BingoCard, draw: &i32) -> Option<&'a mut BingoCell> {
    for row in card {
        if let Some(found) = row.iter_mut().find(|cell| cell.value == *draw) {
            return Some(found);
        }
    }
    return None;
}

fn read_input(file_name: &str) -> (Vec<BingoCard>, Vec<i32>) {
    let input = fs::read_to_string(file_name).expect("Error while reading");

    let mut lines = input.lines().peekable();

    let cage: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut cards: Vec<BingoCard> = vec![];

    while lines.peek().is_some() {
        let card: BingoCard = lines
            .by_ref()
            .skip(1) // Skip the newline
            .take(5)
            .map(|l| {
                l.trim()
                    .replace("  ", " ")
                    .split(' ')
                    .map(|c| BingoCell {
                        value: c.parse().unwrap(),
                        marked: false,
                    })
                    .collect()
            })
            .collect();

        cards.push(card);
    }

    (cards, cage)
}
