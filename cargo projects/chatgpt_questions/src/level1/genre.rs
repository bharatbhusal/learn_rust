// Problem 2:
// You are building a music streaming application.
// Create an enum called Genre that represents different music genres, such as Rock, Pop, HipHop, etc.
// Implement a struct called Song that contains information about a song, including its title, artist, duration in seconds, and genre.
// Add a method to the Song struct that returns a string displaying the song's details.

#[derive(Debug)]
pub enum Genre {
    Rock,
    Pop,
    HipHop,
}

pub struct Song {
    pub title: String,
    pub artist: String,
    pub duration: u64,
    pub genre: Genre,
}

impl Song {
    pub fn details(&self) -> String {
        format!(
            "Title: {}\nArtist: {}\nDuration(in seconds): {}\nGenre: {:?}",
            self.title, self.artist, self.duration, self.genre
        )
    }
}
