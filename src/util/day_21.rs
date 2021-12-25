use regex::Regex;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


#[derive(Debug)]
struct Player {
    number:u64,
    position:u64,
    score:u64,
}


impl Player {

    fn parse(line:&str) -> Player {
        //Player 1 starting position: 4
        let re = Regex::new(r"Player (\d*) starting position: (\d*)").unwrap();
        for cap in re.captures_iter(line) {
            let (number, position) =
                (cap[1].parse::<u64>().unwrap(), cap[2].parse::<u64>().unwrap());
            return Player{number, position, score:0}
        }
        panic!("...");
    }

    fn roll(&mut self, dice:&mut u64, rolls:&mut u64) -> bool {
        for _ in 0..3 {
            self.advance(*dice+1);
            *dice = (*dice + 1) % 100;
        }

        self.score += self.position;
        *rolls +=3;

        self.score >= 1000
    }

    fn advance(&mut self, dice_value:u64) {
        self.position += dice_value;
        while self.position > 10 {
            self.position -= 10;
        }
    }
}




fn part1(lines:Vec<&str>) -> String {
    let mut game_context:Vec<Player> = lines.iter().map( |line| Player::parse(line)).collect();
    let mut player2 = game_context.pop().unwrap();
    let mut player1 = game_context.pop().unwrap();

    let mut rolls = 0;
    let mut dice = 0;
    loop {
        if player1.roll(&mut dice, &mut rolls) {
            // Player 1 wins
            println!("player 2 score:{}, rolls={}",player2.score, rolls);
            return (player2.score * rolls).to_string()
        }
        if player2.roll(&mut dice, &mut rolls) {
            println!("player 1 score:{}, rolls={}",player1.score, rolls);
            return (player1.score * rolls).to_string()
        }
    }
}

fn get_number_of_outcomes(dice_value:u64) -> u64 {
    return match dice_value {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => panic!("..."),
    }
}

fn play_part2(state:&mut Vec<Vec<Option<(u64, u64)>>>, player1_score:u64, player1_pos:u64, player2_score:u64, player2_pos:u64, player1_turn:bool) -> (u64, u64){
    let mut player1_acc_wins:u64 = 0;
    let mut player2_acc_wins:u64 = 0;

    // Check if we already have the count for this combo
    let pos_index = ((player1_pos << 5) + player2_pos) as usize;
    let points_index = ((player1_score << 5) + player2_score) as usize;

    //println!("play => player1:{},{}, player2:{},{}", player1_score, player1_pos, player2_score, player2_pos);
    if let Some(state_element) = state.get(pos_index).unwrap().get(points_index).unwrap() {
        //println!("    => Cached res {:?}", state_element);
        return *state_element;
    }


    // Check if any player reached 21?
    if player1_score > 20 {
        // Player 1 wins...
        player1_acc_wins += 1;
    } else if player2_score > 20 {
        // Player 2 wins...
        player2_acc_wins += 1;
    } else {
        // Roll dices
        let rolls: Vec<(u64, u64)> = (3..=9).into_iter().map(|v| (v, get_number_of_outcomes(v))).collect();

        for (dice_value, outcomes) in rolls {
            if player1_turn {
                let mut player1_next_pos = player1_pos + dice_value;
                while player1_next_pos > 10 {
                    player1_next_pos -= 10;
                }
                let player1_next_score = player1_score + player1_next_pos;

                // Call next game
                let (player1_wins, player2_wins) = play_part2(state, player1_next_score, player1_next_pos, player2_score, player2_pos, !player1_turn);

                // Accumulate number of outcomes for this branch
                player1_acc_wins += player1_wins * outcomes;
                player2_acc_wins += player2_wins * outcomes;

            } else {
                let mut player2_next_pos = player2_pos + dice_value;
                while player2_next_pos > 10 {
                    player2_next_pos -= 10;
                }
                let player2_next_score = player2_score + player2_next_pos;

                // Call next game
                let (player1_wins, player2_wins) = play_part2(state, player1_score, player1_pos, player2_next_score, player2_next_pos, !player1_turn);

                // Accumulate number of outcomes for this branch
                player1_acc_wins += player1_wins * outcomes;
                player2_acc_wins += player2_wins * outcomes;

            }
        }
    }

    // Update cached state
    let update_state_element = state.get_mut(pos_index).unwrap().get_mut(points_index).unwrap();
    *update_state_element = Some((player1_acc_wins, player2_acc_wins));

    //println!("    => CALCULATE res {:?}", update_state_element);

    return (player1_acc_wins, player2_acc_wins)
}

fn part2(lines:Vec<&str>) -> String {
    let mut game_context:Vec<Player> = lines.iter().map( |line| Player::parse(line)).collect();
    println!("{:?}",game_context);
    let player2 = game_context.pop().unwrap();
    let player1 = game_context.pop().unwrap();

    let mut state = vec![];
    for _ in 0..1024 {
        let mut sub_vec = vec![];
        for _ in 0..1024 {
            sub_vec.push(Option::None);
        }
        state.push(sub_vec);
    }

    let outcomes = play_part2(&mut state, 0, player1.position, 20, player2.position, true);

    //307571216519574
    //89974745806771

    //444356092776315
    //341960390180808
    println!("{:?}", outcomes);
    outcomes.0.to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "Player 1 starting position: 4
Player 2 starting position: 8";

        assert_eq!("739785", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_21.txt");

        assert_eq!("675024", solve(input.to_string(), Part1));
    }

    //#[test]
    fn test2() {

        let input = "Player 1 starting position: 4
Player 2 starting position: 8";
        assert_eq!("444356092776315", solve(input.to_string(), Part2));
    }

    //#[test]
    fn test_part2() {
        let input = include_str!("../../input_21.txt");

        assert_eq!("1", solve(input.to_string(), Part2));
    }

}
