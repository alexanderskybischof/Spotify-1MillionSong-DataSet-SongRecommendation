//modules
mod cli;
mod loading_data;
mod tests;

//crates uploaded
use cli::recommend;
use loading_data::{calc_z_sc, feature_matrix, load_data};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let songs_loaded = load_data("song_data.csv")?;
    //loads songs

    let mut features = feature_matrix(&songs_loaded);
    //put loaded data into feature matrix

    calc_z_sc(&mut features);
    //z score!

    println!("{:?}", features.shape());
    //j checking the shape

    //run final code
    recommend(&songs_loaded, &features);

    //some()none()
    Ok(())
}
