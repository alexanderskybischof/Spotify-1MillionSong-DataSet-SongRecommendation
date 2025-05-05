mod loading_data;
mod cli;
mod tests;

use loading_data::{load_data, feature_matrix, calc_z_sc};
use std::error::Error;
use cli::{recommend};




fn main() -> Result<(), Box<dyn Error>> {
    let songs_loaded = load_data("song_data.csv")?;
    //loads songs

    let mut features = feature_matrix(&songs_loaded);
    //put loaded data into feature matrix

    calc_z_sc(&mut features);
    //z score!

    println!("{:?}", features.shape());
    //j checking the shape

  recommend(&songs_loaded, &features);
    Ok(())

    //create a test
}

