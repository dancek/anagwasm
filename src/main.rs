// #![feature(conservative_impl_trait)]

extern crate fixedbitset;

mod anagrams;

mod charbag;
use charbag::CharBag;

use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufRead, Write};
use std::path::Path;

type CharMap = HashMap<char, u8>;

fn filter_alphabetic<'a>(input: &'a str) -> impl Iterator<Item = char> + 'a {
    input.chars().filter(|c| c.is_alphabetic())
}

fn generate_charmap(input: &str) -> (CharMap, Vec<char>) {
    let mut charmap = CharMap::new();
    let mut reverse_charmap = vec![];
    let mut i = 0;
    for c in filter_alphabetic(input) {
        if i == charbag::MAX_CHARIDX {
            panic!("More than {} different characters in input", charbag::MAX_CHARIDX as usize + 1);
        }
        if !charmap.contains_key(&c) {
            charmap.insert(c, i);
            i += 1;
            reverse_charmap.push(c);
        }
    }
    (charmap, reverse_charmap)
}

fn load_dictionary(fname: &Path, cset: &CharBag, cmap: &CharMap) -> (Vec<Vec<String>>, Vec<CharBag>) {
    let f = fs::File::open(fname).expect("Could not open a file");
    let reader = BufReader::new(f);

    let mut words: Vec<Vec<String>> = vec![];
    let mut charsets = vec![];
    let mut charset_map: HashMap<CharBag, usize> = HashMap::new();
    let mut count = 0;

    for line in reader.lines() {
        let line = line.expect("Invalid UTF-8");
        if line.is_empty() {
            continue;
        }
        if let Some(cs) = CharBag::from_str(&line[..], cmap) {
            if (cset - &cs).is_some() {
                if cs.empty() {
                    continue;
                }
                count += 1;
                if charset_map.contains_key(&cs) {
                    let c = charset_map.get(&cs).unwrap();
                    words[*c].push(line);
                } else {
                    words.push(vec![line]);
                    charsets.push(cs.clone());
                    charset_map.insert(cs, words.len() - 1);
                }
            }
        }
    }
    eprintln!("Loaded {} dictionary words, {} distinct.", count, words.len());
    (words, charsets)
}

fn output_words(word_idxs: &[usize], words: &[Vec<String>]) {
    let size = word_idxs.len();
    let mut idxs = vec![0; size];

    let stdout = ::std::io::stdout();
    let mut out_handle = stdout.lock();

    loop {
        let _ = out_handle.write(words[word_idxs[0]][idxs[0]].as_bytes());
        for i in 1..size {
            let _ = out_handle.write(b" ");
            let _ = out_handle.write(words[word_idxs[i]][idxs[i]].as_bytes());
        }
        let _ = out_handle.write(b"\n");

        let mut curr = (size - 1) as isize;
        while curr >= 0 {
            idxs[curr as usize] += 1;
            if idxs[curr as usize] == words[word_idxs[curr as usize]].len() {
                idxs[curr as usize] = 0;
                curr -= 1;
            } else {
                break;
            }
        }
        if curr < 0 {
            break;
        }
    }
}

fn main() {
    let dict_path = std::env::args().nth(1).unwrap();
    let input = std::env::args().nth(2).unwrap();
    let lowercased = input.to_lowercase();
    let (charmap, _reverse_charmap) = generate_charmap(&lowercased[..]);
    let input_charset = CharBag::from_str(&lowercased[..], &charmap).expect("input_charset");
    let (dict_words, dict_charsets) = load_dictionary(&Path::new(&dict_path), &input_charset, &charmap);

    anagrams::for_all_anagrams(&dict_charsets, &input_charset, 3 /* len */, move |word_idxs| {
        assert!(!word_idxs.is_empty());
        output_words(word_idxs, &dict_words);
    });
}
