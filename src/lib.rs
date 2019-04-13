extern crate wasm_bindgen;
extern crate fixedbitset;

mod anagrams;
mod charbag;

use charbag::CharBag;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

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

fn load_dictionary(raw_wordlist: Vec<&str>, cset: &CharBag, cmap: &CharMap) -> (Vec<Vec<String>>, Vec<CharBag>) {
    let mut words: Vec<Vec<String>> = vec![];
    let mut charsets = vec![];
    let mut charset_map: HashMap<CharBag, usize> = HashMap::new();
    let mut count = 0;

    let mut wordlist: Vec<String> = vec![];

    for word in raw_wordlist {
        let word = word.to_string();
        if word.is_empty() {
            continue;
        }
        wordlist.push(word);
    }

    wordlist.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

    for word in wordlist {
        if let Some(cs) = CharBag::from_str(&word[..], cmap) {
            if (cset - &cs).is_some() {
                if cs.empty() {
                    continue;
                }
                count += 1;
                if charset_map.contains_key(&cs) {
                    let c = charset_map.get(&cs).unwrap();
                    words[*c].push(word);
                } else {
                    words.push(vec![word]);
                    charsets.push(cs.clone());
                    charset_map.insert(cs, words.len() - 1);
                }
            }
        }
    }
    eprintln!("Loaded {} dictionary words, {} distinct.", count, words.len());
    (words, charsets)
}

fn get_anagrams(word_idxs: &[usize], words: &[Vec<String>]) -> Vec<JsValue> {
    let size = word_idxs.len();
    let mut idxs = vec![0; size];
    let mut anagrams: Vec<JsValue> = Vec::new();

    loop {
        let mut tmp = vec![];
        for i in 0..size {
            tmp.push(words[word_idxs[i]][idxs[i]].clone());
        }
        anagrams.push(JsValue::from(tmp.join(" ")));

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

    anagrams
}

#[wasm_bindgen]
pub fn create_anagrams(input: &str) -> Vec<JsValue> {
    let wordlist: Vec<&str> = include_str!("../resources/kotus_sanat.txt").lines().collect();

    let lowercased = input.to_lowercase();
    let (charmap, _reverse_charmap) = generate_charmap(&lowercased[..]);
    let input_charset = CharBag::from_str(&lowercased[..], &charmap).expect("input_charset");
    let (dict_words, dict_charsets) = load_dictionary(wordlist, &input_charset, &charmap);

    let mut anagrams: Vec<JsValue> = Vec::new();
    anagrams::for_all_anagrams(&dict_charsets, &input_charset, 3 /* len */, |word_idxs| {
        assert!(!word_idxs.is_empty());
        anagrams.append(&mut get_anagrams(word_idxs, &dict_words));
    });

    anagrams
}
