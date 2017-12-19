use charbag::CharBag;

pub fn for_all_anagrams<F>(dict_charsets: &[CharBag], charset: &CharBag, max_len: usize, f: F)
where
    F: Fn(&[usize]),
{
    let mut words = vec![];
    for_all_anagrams_iter(dict_charsets, charset, &f, &mut words, 0, 0, max_len);
}

fn for_all_anagrams_iter<F>(
    dict_charsets: &[CharBag],
    charset: &CharBag,
    f: &F,
    words: &mut Vec<usize>,
    start_idx: usize,
    curr_len: usize,
    max_len: usize,
) where
    F: Fn(&[usize]),
{
    if curr_len + 1 >= max_len {
        for_all_anagrams_iter_last(dict_charsets, charset, f, words, start_idx);
        return;
    }
    for i in start_idx..dict_charsets.len() {
        if let Some(cs) = charset - &dict_charsets[i] {
            words.push(i);
            if cs.empty() {
                f(&words[..])
            } else {
                for_all_anagrams_iter(dict_charsets, &cs, f, words, i, curr_len + 1, max_len);
            }
            words.pop();
        }
    }
}


fn for_all_anagrams_iter_last<F>(
    dict_charsets: &[CharBag],
    charset: &CharBag,
    f: &F,
    words: &mut Vec<usize>,
    start_idx: usize,
) where
    F: Fn(&[usize]),
{
    for i in start_idx..dict_charsets.len() {
        if charset == &dict_charsets[i] {
            words.push(i);
            f(words);
            words.pop();
        }
    }
}
