#![feature(test)]
extern crate test as std_test;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
const WORD: &'static str = r#"
Once upon a time, in a vibrant forest filled with chirping birds, rustling leaves, and the sound of a bubbling brook,
lived a hare known for his speed and a tortoise renowned for his perseverance. The hare, a lean and agile creature, often boasted about how he was
the fastest in the forest. The tortoise, slow but steadfast, listened to these boasts without comment, his calm demeanor unshaken.
One sunny morning, the hare was in his usual place, holding court among the forest animals. He stood atop a rock, demonstrating
his impressive sprints between two tall oaks. The crowd cheered, except for the tortoise, who simply smiled and continued munching on a leaf.
The hare noticed and called out, “What’s so funny, old timer? Surely, you don’t think you could ever beat me in a race!”
The tortoise replied, “I wasn’t laughing at you, Hare. But since you bring it up, why not prove your speed in a race against me?”
The forest erupted in laughter. A race between the speedy hare and the plodding tortoise seemed absurd. But the hare, eager to flaunt his superiority, agreed immediately.
“Fine! Let’s race tomorrow morning. Everyone will see just how foolish your challenge is.Once upon a time, in a vibrant forest filled with chirping birds, rustling leaves, and the sound of a bubbling brook,
lived a hare known for his speed and a tortoise renowned for his perseverance. The hare, a lean and agile creature, often boasted about how he was
the fastest in the forest. The tortoise, slow but steadfast, listened to these boasts without comment, his calm demeanor unshaken.
One sunny morning, the hare was in his usual place, holding court among the forest animals. He stood atop a rock, demonstrating
his impressive sprints between two tall oaks. The crowd cheered, except for the tortoise, who simply smiled and continued munching on a leaf.
The hare noticed and called out, “What’s so funny, old timer? Surely, you don’t think you could ever beat me in a race!”
The tortoise replied, “I wasn’t laughing at you, Hare. But since you bring it up, why not prove your speed in a race against me?”
The forest erupted in laughter. A race between the speedy hare and the plodding tortoise seemed absurd. But the hare, eager to flaunt his superiority, agreed immediately.
“Fine! Let’s race tomorrow morning. Everyone will see just how foolish your challenge is.Once upon a time, in a vibrant forest filled with chirping birds, rustling leaves, and the sound of a bubbling brook,
lived a hare known for his speed and a tortoise renowned for his perseverance. The hare, a lean and agile creature, often boasted about how he was
the fastest in the forest. The tortoise, slow but steadfast, listened to these boasts without comment, his calm demeanor unshaken.
One sunny morning, the hare was in his usual place, holding court among the forest animals. He stood atop a rock, demonstrating
his impressive sprints between two tall oaks. The crowd cheered, except for the tortoise, who simply smiled and continued munching on a leaf.
The hare noticed and called out, “What’s so funny, old timer? Surely, you don’t think you could ever beat me in a race!”
The tortoise replied, “I wasn’t laughing at you, Hare. But since you bring it up, why not prove your speed in a race against me?”
The forest erupted in laughter. A race between the speedy hare and the plodding tortoise seemed absurd. But the hare, eager to flaunt his superiority, agreed immediately.
“Fine! Let’s race tomorrow morning. Everyone will see just how foolish your challenge is.
"#;

#[cfg(test)]
mod test {
    use crate::WORD;
    use std::borrow::Cow;
    use std_test::{black_box, Bencher};

    #[bench]
    fn bench_creating_cow(b: &mut Bencher) {
        b.iter(|| black_box(Cow::Borrowed(WORD)));
    }

    #[bench]
    fn bench_creating_string(b: &mut Bencher) {
        b.iter(|| black_box(String::from(WORD)));
    }

    #[bench]
    fn mutating_cow(b: &mut Bencher) {
        b.iter(|| {
            let cow_value = Cow::Borrowed(WORD);
            let mut owned_cow_value = cow_value.into_owned();

            owned_cow_value.push_str(&WORD);
            black_box(owned_cow_value)
        });
    }

    #[bench]
    fn mutating_owned_string(b: &mut Bencher) {
        b.iter(|| {
            let mut owned_value = String::from(WORD);
            owned_value.push_str(&WORD);

            black_box(owned_value)
        });
    }
}
