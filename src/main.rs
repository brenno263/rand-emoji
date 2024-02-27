
use std::{env, error::Error, fs::{self, File}, io::{BufRead, BufReader}, iter, path::Path};

use rand::{
    distributions::*, seq::IteratorRandom, Rng
};


struct Emojichar {
    pub character: char,
}

impl Distribution<Emojichar> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Emojichar {
        // A valid `char` is either in the interval `[0, 0xD800)` or
        // `(0xDFFF, 0x11_0000)`. All `char`s must therefore be in
        // `[0, 0x11_0000)` but not in the "gap" `[0xD800, 0xDFFF]` which is
        // reserved for surrogates. This is the size of that gap.
        const GAP_SIZE: u32 = 0xDFFF - 0xD800 + 1;

        // We want to generate chars only within the gap [RANGE_MIN, RANGE_MAX)
        const RANGE_MIN: u32 = 0x231A;
        const RANGE_MAX: u32 = 0x1FAF8;

        const RANGES: [(u32,u32); 1]= [
            (2, 3),
        ];

        // Uniform::new(0, 0x11_0000 - GAP_SIZE) can also be used but it
        // seemed slower.
        let range = Uniform::new(RANGE_MIN, RANGE_MAX + 1);

        let mut n = range.sample(rng);
        if n <= 0xDFFF {
            n -= GAP_SIZE;
        }
        Emojichar {
            character: unsafe { char::from_u32_unchecked(n) }
        }
    }
}

struct EmojiIter<R> {
    rng: R,
}

impl<R> EmojiIter<R>
where
    R: Rng,
{
    pub fn new(rng: R) -> Self {
        Self {
            rng
        }
    }
}

impl<R> Iterator for EmojiIter<R>
where
    R: Rng,
{
    type Item = char;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        // Here, self.rng may be a reference, but we must take &mut anyway.
        // Even if sample could take an R: Rng by value, we would need to do this
        // since Rng is not copyable and we cannot enforce that this is "reborrowable".
        Some('d')
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::max_value(), None)
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum EmojiType {
    Basic,
    KeycapSequence,
    FlagSequence,
    ModifierSequence,
    Unknown
}

#[derive(Debug)]
struct EmojiData {
    character_raw: u32,
    emoji_type: EmojiType,
}

impl EmojiData {
    fn to_char(self) -> Option<char> {
        char::from_u32(self.character_raw)
    }
}

fn parse_emoji_data(lines: impl Iterator<Item = String>) -> impl Iterator<Item = EmojiData> {
    lines
        .filter_map(|line| {
            let first_char = line.chars().nth(0).unwrap_or(' ');
            if first_char.is_alphanumeric() {
                Some(line)
            } else {
                None
            }
        })
        .flat_map(|line| {
            let mut emoji_datas = Vec::new();
            let segments = line.split(';').map(str::trim).collect::<Vec<_>>();
            let hex_string = segments[0];
            let emoji_type = match segments[1] {
                "Basic_Emoji" => EmojiType::Basic, 
                _ => EmojiType::Unknown
            };
            // Don't accept non-basic types or multi-char emoji
            if emoji_type != EmojiType::Basic || hex_string.contains(' ') {
                return emoji_datas;
            }
            let hex_strings: Vec<_> = hex_string.split("..").collect();
            if hex_strings.len() == 1 {
                let character_raw = u32::from_str_radix(hex_string, 16).unwrap();
                emoji_datas.push(
                    EmojiData {
                        character_raw,
                        emoji_type,
                    }
                );
            } else {
                let lower_bound = u32::from_str_radix(hex_strings[0], 16).unwrap();
                let upper_bound = u32::from_str_radix(hex_strings[1], 16).unwrap();
                (lower_bound..=upper_bound)
                    .for_each(|n| {
                        emoji_datas.push(
                            EmojiData { character_raw: n, emoji_type}
                        )
                    })
            }
            return emoji_datas;
        })
}

fn load_from_file<P>(path: P) -> Option<impl Iterator<Item = EmojiData>>
where
    P: AsRef<Path>
{
    let file = File::open(path).ok()?;
    let file_lines = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok());

    Some(parse_emoji_data(file_lines))
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("No file path given");

    let emoji_data = load_from_file(path).expect("Could not read file");


    let mut rng = rand::thread_rng();
    let emoji = emoji_data.choose(&mut rng).unwrap();
    println!("{}", emoji.to_char().unwrap())
}
