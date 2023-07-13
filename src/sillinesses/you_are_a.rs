use std::collections::HashSet;

use once_cell::sync::Lazy;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serenity::{model::prelude::Message, prelude::Context};

use tracing::{error, info};

static NOUNS: &'static str = include_str!("nouns_lower.txt");
static CACHE: Lazy<HashSet<&str>> = Lazy::new(|| {
    let num = NOUNS.split("\n").count();
    let mut acc = HashSet::with_capacity(num);
    acc.extend(NOUNS.split("\n").map(|w| w.trim_end()));
    acc
});

const JOKE_PROB: f64 = 0.01;

pub async fn run(ctx: Context, msg: Message) {
    let mut rng = SmallRng::from_entropy();

    for word in msg.content.split_whitespace() {
        if !CACHE.contains(word) || word == "a" {
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
