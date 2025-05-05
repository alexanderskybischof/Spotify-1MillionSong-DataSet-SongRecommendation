//this module holds the tests for my recommendation system
#[cfg(test)]

    use crate::cli::GenreFilter;
    use super::*;
    use crate::loading_data::Song;
    use crate::cli::compute_distances;
    use crate::cli::build_candidates;
    use crate::cli::Popularity;
    use ndarray::array;
//creating song data on certain parameters, then using function build candidates, compute distances on them to check 
#[cfg(test)]
mod tests {
    use super::*;
    use crate::loading_data::Song;
    use ndarray::array;


    fn songs() -> Vec<Song> {
        vec![
            Song {
                track_id:        "1".into(),
                track_name:      "A".into(),
                artist_name:     "X".into(),
                popularity:      40,
                year:            2020,
                genre:           "rock".into(),
                danceability:    0.5,
                energy:          0.5,
                key:             5,
                loudness:       -5.0,
                mode:            1,
                speechiness:     0.05,
                acousticness:    0.10,
                instrumentalness:0.00,
                liveness:        0.10,
                valence:         0.50,
                tempo:          120.0,
                duration_ms:   200_000,
                time_signature:  4,
            },
            Song {
                track_id:        "2".into(),
                track_name:      "B".into(),
                artist_name:     "Y".into(),
                popularity:      60,
                year:            2021,
                genre:           "pop".into(),
                danceability:    0.6,
                energy:          0.6,
                key:             6,
                loudness:       -4.0,
                mode:            1,
                speechiness:     0.04,
                acousticness:    0.20,
                instrumentalness:0.00,
                liveness:        0.20,
                valence:         0.60,
                tempo:          130.0,
                duration_ms:   210_000,
                time_signature:  4,
            },
            Song {
                track_id:        "3".into(),
                track_name:      "C".into(),
                artist_name:     "Z".into(),
                popularity:      70,
                year:            2022,
                genre:           "rock".into(),
                danceability:    0.7,
                energy:          0.7,
                key:             7,
                loudness:       -3.0,
                mode:            1,
                speechiness:     0.03,
                acousticness:    0.30,
                instrumentalness:0.00,
                liveness:        0.30,
                valence:         0.70,
                tempo:          140.0,
                duration_ms:   220_000,
                time_signature:  4,
            },
        ]
    }
//tests the build candidates test
    #[test]
    fn build_candidates_basic() {
        let s = songs();
        assert_eq!(build_candidates(&s, &None, &None, 0), vec![1, 2]);
        assert_eq!(
            build_candidates(&s, &Some(Popularity::Popular), &None, 0),
            vec![1, 2]
        );
        assert_eq!(
            build_candidates(&s, &None, &Some(GenreFilter::Different), 0),
            vec![1]
        );
    }

    #[test]
    fn compute_distances_basic() {
        let f = array![[0., 0.], [3., 4.], [6., 8.]];
        let d = compute_distances(0, &[1, 2], &f);
        assert!((d[0].0 - 5.).abs() < 1e-6 && d[0].1 == 1);
        assert!((d[1].0 - 10.).abs() < 1e-6 && d[1].1 == 2);
    }
}