use std::collections::HashSet;

use once_cell::sync::Lazy;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serenity::{model::prelude::Message, prelude::Context};

use tracing::{error, info};

pub static NOUNS: &'static str = include_str!("A:/_Coding/Rust/silly_bot/files/nouns_lower.txt");
pub static CACHE: Lazy<HashSet<&str>> = Lazy::new(|| {
    let num = NOUNS.split("\n").count();
    let mut acc = HashSet::with_capacity(num);
    // for noun in NOUNS.split("\n") {
    //     acc.insert(noun.trim());
    // }
    acc.extend(NOUNS.split("\n").map(|w| w.trim_end()));
    acc
});

// static MOBYPOS: &'static str = include!("A:/_Coding/Rust/silly_bot/files/mobypos_utf8.txt");
// pub static CACHE: Lazy<HashSet<&str>> = Lazy::new(|| {
//     let num = MOBYPOS.split("\n").count();
//     let mut acc = HashSet::new();
//     for noun in MOBYPOS.split("\n") {
//         acc.insert(noun.trim());
//     }
//     let nouns = MOBYPOS
//         .split("\n")
//         .map(|l| l.split_once(r"\"))
//         .filter(|o| o.is_some())
//         .map(|o| o.unwrap())
//         .filter(|s| p.1 == "N")
//         .map(|t| t.0);

//     info!("{}" nouns.take(20).collect::Vec<&str>());
//     acc
// });

const JOKE_PROB: f64 = 1.0;

pub async fn run(ctx: Context, msg: Message) {
    let mut rng = SmallRng::from_entropy();

    for word in msg.content.split(" ") {
        if !CACHE.contains(word) {
            continue;
        }
        if rng.gen_bool(JOKE_PROB) {
            std::mem::drop(rng);
            if let Err(why) = msg.reply(ctx, format!("You're a {}", word)).await {
                error!("Error replying: {}", why);
            }
            return;
        }
    }
}
