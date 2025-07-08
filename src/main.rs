use std::collections::HashMap;
use std::io::{self, Write};
use rand::seq::IteratorRandom;
use rand::thread_rng;

struct BoxInfo {
    score_reward: u32,
    money_reward: u32,
}

struct GameStats {
    chance: u32,
    money: u32,
    score: u32,
    boxes: HashMap<String, BoxInfo>
}

fn menu(game: &mut GameStats) {
    loop {
        print!("\n**menu**: luckygame, store, status, quit: ");
        io::stdout()
            .flush()
            .unwrap();
        
        let mut opt = String::new();
        io::stdin()
            .read_line(&mut opt)
            .unwrap();

        let opt = opt.trim();

        if opt == "luckygame" || opt.is_empty() {
            luckygame(game);
        } else if opt == "store" {
            store(game);
        } else if opt == "status" {
            status(game);
        } else if opt == "quit" {
           println!("\nYour final score: {}", game.score); 
           println!("Thanks for playing!");
           break;
        }

    }
}

fn luckygame(game: &mut GameStats) {
    loop {
        print!("\n**luckygame**: box => -1 chance, back => menu: ");
        io::stdout()
            .flush()
            .unwrap();

        let mut opt = String::new();

        io::stdin()
            .read_line(&mut opt)
            .unwrap();

        let opt = opt.trim();

        if opt == "box" || opt.is_empty() {

            if game.chance == 0 {
                println!("**out of chance**");
                continue;
            }

            let mut rng = thread_rng();

            let (name, info) = game.boxes
                                   .iter()
                                   .choose(&mut rng)
                                   .unwrap();

            game.chance -= 1;
            game.score += info.score_reward;
            game.money += info.money_reward;

            println!("You got a {} box!", name);
            println!(
                "chance: {} (-1) score: {} (+{}) money: {} (+{})",
                game.chance, 
                game.score, info.score_reward, 
                game.money, info.money_reward,
            )
            
        } else if opt == "back" {
            break;
        } else {
            continue;
        }
    }
}

fn store(game: &mut GameStats) {
    loop {
        print!("\n**store**: buy => chance (+1) money (-30), back => menu: ");
        io::stdout()
            .flush()
            .unwrap();

        let mut opt = String::new();

        io::stdin()
            .read_line(&mut opt)
            .unwrap();

        let opt = opt.trim();

        if opt == "buy" || opt.is_empty() {

            if game.money < 30 {
                println!("**out of money**");
                continue;
            }

            game.chance += 1;
            game.money -= 30;

            println!(
                "chance: {} (+1) score: {} money: {} (-30)",
                game.chance, game.score,game.money
            )
            
        } else if opt == "back" {
            break;
        } else {
            continue;
        }
    }
}

fn status(game: &GameStats) {
    println!(
        "chance: {} score: {} money: {}",
        game.chance, game.score, game.money
    )
}

fn main() {

    let mut boxes = HashMap::new();
    boxes.insert("normal".to_string(), 
    BoxInfo { score_reward: 5, money_reward: 15});
    boxes.insert("rare".to_string(), 
    BoxInfo { score_reward: 10, money_reward: 30});
    boxes.insert("epic".to_string(), 
    BoxInfo { score_reward: 15, money_reward: 60});
    boxes.insert("legendary".to_string(), 
    BoxInfo { score_reward: 20, money_reward: 120});

    let mut game = GameStats {
        chance: 3,
        money: 0,
        score: 0,
        boxes,
    };

    menu(&mut game);

}
