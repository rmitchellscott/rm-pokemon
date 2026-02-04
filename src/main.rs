use memchr::memmem;
use memmap2::Mmap;
use rust_embed::Embed;
use std::fs::File;
use std::time::Instant;

const DEFAULT_XOCHITL_PATH: &str = "/usr/bin/xochitl";
const VERSION: &str = env!("RM_POKEMON_VERSION");
const MARKER: &[u8] = b"h\0e\0a\0d\0s\0/\0r\0e\0l\0e\0a\0s\0e\0s\0/\0";

#[derive(Embed)]
#[folder = "assets/sprites/"]
struct Sprites;

fn find_pokemon_name(path: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let mmap = unsafe { Mmap::map(&file).ok()? };

    let search_start = mmap.len().saturating_sub(mmap.len() / 4);
    let search_region = &mmap[search_start..];

    let pos = memmem::find(search_region, MARKER)?;

    let start = pos + MARKER.len();
    let remaining = &search_region[start..];

    let mut end = 0;
    while end + 1 < remaining.len() {
        if remaining[end] == 0 && remaining[end + 1] == 0 {
            break;
        }
        end += 2;
    }

    let utf16_bytes = &remaining[..end];
    let utf16: Vec<u16> = utf16_bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();

    let version_str = String::from_utf16(&utf16).ok()?;

    let after_dash = version_str.split('-').nth(1)?;
    let pokemon = after_dash.split_whitespace().next()?;

    Some(pokemon.to_lowercase())
}

fn print_sprite(pokemon: &str) -> bool {
    if let Some(sprite) = Sprites::get(pokemon) {
        if let Ok(art) = std::str::from_utf8(&sprite.data) {
            print!("{art}");
            return true;
        }
    }
    false
}

fn main() {
    let arg = std::env::args().nth(1);
    if arg.as_deref() == Some("--version") || arg.as_deref() == Some("-V") {
        println!("rm-pokemon {VERSION}");
        return;
    }

    let debug = std::env::var("DEBUG").is_ok();
    let xochitl_path = arg.as_deref().unwrap_or(DEFAULT_XOCHITL_PATH);

    let start = Instant::now();
    let pokemon = find_pokemon_name(xochitl_path);
    let parse_time = start.elapsed();

    match pokemon {
        Some(pokemon) => {
            let start = Instant::now();
            if !print_sprite(&pokemon) {
                eprintln!("No sprite found for {pokemon}");
                println!("{pokemon}");
            }
            let sprite_time = start.elapsed();

            if debug {
                eprintln!("parse: {parse_time:?}, sprite: {sprite_time:?}");
            }
        }
        None => eprintln!("No pokemon found in {xochitl_path}"),
    }
}
