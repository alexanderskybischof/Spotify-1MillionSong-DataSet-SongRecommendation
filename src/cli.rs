//This module is used to prompt the user, which applies filters on the matrix. Once filtered, 
// the code iterates through the matrix and takes the euclidean distance for each row, comparing it and storing it,
//  and outputs the top k nearest song recommendations
use std::io::{self, Write};
use ndarray::Array2;
use crate::loading_data::Song;

//for said input, iterate over the matrix (by row), compute euclidean distance to the input, store euclidean distances, associated with name and id. 
//with that done, print out top 5 songs based on closeness of euclidean distance. 

//indicates whether to filter songs by popularity
pub enum Popularity {
    Underground,
    //includes songs with a popularity < 50
    
    Popular,
    //includes songs greater or equal to 50
}
//indicates whether to filter based on genre
pub enum GenreFilter {
    //same genre
    Same,

    //different genre
    Different,
}


//import song struct 
// inputs a slice of the song struct,
//returns the index of the selected song in songs
//reads user input, matches against each songs id or name. 
pub fn prompt_song(songs: &[Song]) -> usize {
    loop {
        print!("Please enter the song ID or the song name: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input, try again.");
            continue;
        }
        let q = input.trim();

        //finds track id or name(case does not matter)
        if let Some(i) = songs.iter().position(|s|
            s.track_id.eq_ignore_ascii_case(q)
            || s.track_name.eq_ignore_ascii_case(q)
        ) {
            return i;//if found a valid index
        } else {
            println!("Song was not found. Please try again.");
        }
    }
}

//retursn an integer in the range of 1- =20
fn prompt_for_k() -> usize {
    loop {
        print!("How many song recommendations would you like (1–20)? ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read the input. Please try again.");
            continue;
        }
        //parse to a usize and then check if it is within the range 
        if let Ok(k) = input.trim().parse::<usize>() {
            if (1..=20).contains(&k) {
                return k;
            }
        }

        println!("Not a valid number. Please enter an integer between 1 and 20.");
    }
}
    
//outputs a none if there is no filter, 
//will output some popularity if the popular or underground is selected
pub fn prompt_for_popularity() -> Option<Popularity> {
    loop {
        print!("Would you like to filter by popularity? (y/n): ");
        io::stdout().flush().unwrap();

        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        match ans.trim().to_lowercase().as_str() {
            "n" => return None, //for no filter
            "y" => {
                print!("  Show only Underground (u) or Popular (p)? ");
                io::stdout().flush().unwrap();
                ans.clear();
                io::stdin().read_line(&mut ans).unwrap();
                //maps to input, case insensitive
                return match ans.trim().to_lowercase().as_str() {
                    "u" => Some(Popularity::Underground),
                    "p" => Some(Popularity::Popular),
                    _   => { println!("Enter ‘u’ or ‘p’."); continue }//in the case it's invalid
                };
            }
            _ => println!("Enter ‘y’ or ‘n’."),
        }
    }
}
//return none for no filter, a some(genrefilter::same) in same-genre
//some(different genre) for different genre
pub fn prompt_for_genre() -> Option<GenreFilter> {
    loop {
        print!("Filter by genre? (same/different/none): ");
        io::stdout().flush().unwrap();

        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        match ans.trim().to_lowercase().as_str() {
            "none"      => return None,
            "same"      => return Some(GenreFilter::Same),
            "different" => return Some(GenreFilter::Different),
            _ => {
                println!("Please enter 'same', 'different', or 'none'.");
                continue;
            }
        }
    }
}
//builds list of candidates after applying a popularity and genre filter.

//inputs include slice of songs, reference to population filter(if applied),
//genre filter if applicable
//query_idx, which stands for index of the query song to compare against

//outputs a indices of songs after passing the filters

pub fn build_candidates(
    songs: &[Song],
    pop_filter: &Option<Popularity>,
    genre_filter: &Option<GenreFilter>,
    query_idx: usize
) -> Vec<usize> {
    let query_genre = &songs[query_idx].genre;
    songs.iter().enumerate()
        .filter_map(|(i, s)| {
            // skip the query itself
            if i == query_idx { return None; }

            // popularity check
            let pop_ok = match pop_filter {
                None => true,
                Some(Popularity::Underground) => s.popularity < 50,
                Some(Popularity::Popular)     => s.popularity >= 50,
            };
            if !pop_ok { return None; }

            // genre check
            let genre_ok = match genre_filter {
                None => true,
                Some(GenreFilter::Same)      => &s.genre == query_genre,
                Some(GenreFilter::Different) => &s.genre != query_genre,
            };
            if !genre_ok { return None; }

            Some(i)
        })
        .collect()
}
//checks for euclidean distance between each candidate song, and query song
//inputs index of query song, candidates, and features
//outputs vec (distance, song_index) tuples
pub fn compute_distances(
    query_idx: usize,
    candidates: &[usize],
    features: &Array2<f32>
) -> Vec<(f32, usize)> {
    let qrow = features.row(query_idx);
    candidates.iter()
        .map(|&i| {
            let sum_sq = (&features.row(i) - &qrow).mapv(|x| x*x).sum();
            (sum_sq.sqrt(), i)
        })
        .collect()
}

//prompts users for song, k(to 20), and filters
//builds the candidates, computes the distances, then sorts
//prints out the top k song recs.

//inputs song struct, features for 2d array of feature vecs.
pub fn recommend(songs: &[Song], features: &Array2<f32>) {
    let idx = prompt_song(songs);
    let k = prompt_for_k();
    let pop_filter = prompt_for_popularity();
    let genre_filter = prompt_for_genre();

    //computes, sorts distances
    let mut dists = compute_distances(
        idx,
        &build_candidates(songs, &pop_filter, &genre_filter, idx),
        features,
    );
    dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    //prints the
    println!("\nTop {} recommendations for “{}”:", k, songs[idx].track_name);
    for (rank, &(_dist, si)) in dists.iter().take(k).enumerate() {
        println!(
            "{}. {} — {} [{}] (pop={})",
            rank + 1,
            songs[si].track_name,
            songs[si].artist_name,
            songs[si].genre,
            songs[si].popularity
        );
    }
}