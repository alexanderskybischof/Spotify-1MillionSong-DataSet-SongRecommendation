use ndarray::{Array, Array1, Array2, Axis, stack};
use serde::Deserialize;
use std::error::Error;
use csv::ReaderBuilder;



//create a mod where it loads all the data, z score normalizes it, and spits out a usable matrix(with sensible numbers) for the code. 

//struct holding song data. 
#[derive(Debug, Deserialize)]
pub struct Song {
    #[serde(rename = "artist_name")]
    pub artist_name: String,
    #[serde(rename = "track_name")]
    pub track_name: String,
    #[serde(rename = "track_id")]
    pub track_id: String,
    #[serde(rename = "popularity")]
    pub popularity: u8,
    #[serde(rename = "year")]
    pub year: u16,
    #[serde(rename = "genre")]
    pub genre: String,
    #[serde(rename = "danceability")]
    pub danceability: f32,
    #[serde(rename = "energy")]
    pub energy: f32,
    #[serde(rename = "key")]
    pub key: u8,
    #[serde(rename = "loudness")]
    pub loudness: f32,
    #[serde(rename = "mode")]
    pub mode: u8,
    #[serde(rename = "speechiness")]
    pub speechiness: f32,
    #[serde(rename = "acousticness")]
    pub acousticness: f32,
    #[serde(rename = "instrumentalness")]
    pub instrumentalness: f32,
    #[serde(rename = "liveness")]
    pub liveness: f32,
    #[serde(rename = "valence")]
    pub valence: f32,
    #[serde(rename = "tempo")]
    pub tempo: f32,
    #[serde(rename = "duration_ms")]
    pub duration_ms: u32,
    #[serde(rename = "time_signature")]
    pub time_signature: u8,
}

//Loads data into a vector of songs. 
pub fn load_data(path: &str) -> Result<Vec<Song>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(path)?;
    let mut songs = Vec::new();
    for song in rdr.deserialize(){
    let s: Song = song?; // parse into song vec
        songs.push(s);
}//pushes all into one vector
Ok(songs)
}

//function creates a usable matrix from the feature vectors calculated before. 
pub fn feature_matrix(songs: &[Song]) -> Array2<f32>{
    let mut feats:Vec<Array1<f32>> = songs.iter().map(|s|{//iterates over and maps 
        let log_dur = (s.duration_ms as f32).ln();
        let age = (2025 - s.year) as f32; //current year - year it came out
        let time_signature_f = s.time_signature as f32;
        let theta = 2.0 * std::f32::consts::PI * (s.key as f32) / 12.0;
        let key_cos = theta.cos() as f32;
        let key_sin = theta.sin() as f32;
        let tempo_clipped = s.tempo.clamp(40.0, 200.0);//clipping for outliers
        //redefining values to numeric values before getting z scored. 
        // Lookedup ways for this to make it so one field does not dominate
        //tempo, key, mode, 
        Array1::from(vec![age,
        s.danceability,
        s.energy,
        key_cos,
        key_sin,//
        s.loudness,
        s.mode as f32,
        s.speechiness,
        s.acousticness,
        s.instrumentalness,
        s.liveness,
        s.valence,
        tempo_clipped,
        log_dur,//lower weights/ drop if this is affecting too much
        time_signature_f,
        ])
    }
    //now that i created matrix, need to z score normalization 
    ).collect();//mapv, collect bc retursn iterators
    //stack into a array2 matrix
    let views = feats.iter().map(|r| r.view()).collect::<Vec<_>>();
    stack(Axis(0), &views).expect("all rows must be same length")//relook
}

//creating a vec<vec<f32>> for matrix of numerics for each. 
//make sure to convert all to f32


//z score calculation to minimize outlier dependency
//for a vector in the matrix features, calculate the mean for each of those vectors. 
//then calculate the st. deviation of that
//then calculate zscore.
//then create a new vector matrix where z score is applied 
//add all to a new matrix. 

pub fn calc_z_sc(feats: &mut Array2<f32>){
for mut vec in feats.axis_iter_mut(Axis(0)){//for vector in features . Make sure iterate over rows, not just elements
    let mut mean = vec.mean().expect("must have a mean");
    let mut st_dv = vec.std(0.0);
    for x in vec.iter_mut(){
        *x = (*x - mean)/st_dv;//dereference to change values
    };
}
}


//iterates over then map  
