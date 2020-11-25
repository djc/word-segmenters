#![cfg(feature = "__test_data")]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use super::Segmenter;

pub fn segmenter() -> Segmenter {
    let dir = PathBuf::from(format!("{}/data", env!("CARGO_MANIFEST_DIR")));

    let uni_file = dir.join("unigrams.txt");
    let reader = BufReader::new(File::open(&uni_file).unwrap());
    let unigrams = reader.lines().enumerate().map(move |(i, ln)| {
        let ln = ln?;
        let split = ln
            .find('\t')
            .ok_or_else(|| format!("no tab found in {:?}:{}", uni_file, i))?;

        let word = ln[..split].into();
        let p = usize::from_str(&ln[split + 1..])
            .map_err(|e| format!("error at {:?}:{}: {}", uni_file, i, e))?;
        Ok((word, p as f64))
    });

    let bi_file = dir.join("bigrams.txt");
    let reader = BufReader::new(File::open(&bi_file).unwrap());
    let bigrams = reader.lines().enumerate().map(move |(i, ln)| {
        let ln = ln?;
        let word_split = ln
            .find(' ')
            .ok_or_else(|| format!("no space found in {:?}:{}", bi_file, i))?;
        let score_split = ln[word_split + 1..]
            .find('\t')
            .ok_or_else(|| format!("no tab found in {:?}:{}", bi_file, i))?
            + word_split
            + 1;

        let word1 = ln[..word_split].into();
        let word2 = ln[word_split + 1..score_split].into();
        let p = usize::from_str(&ln[score_split + 1..])
            .map_err(|e| format!("error at {:?}:{}: {}", bi_file, i, e))?;

        Ok(((word1, word2), p as f64))
    });

    Segmenter::from_iters(unigrams, bigrams).unwrap()
}