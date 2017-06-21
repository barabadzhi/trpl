use std::ops::Deref;

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

fn main() {
    let my_favorite_song = Mp3 {
        audio: vec![1, 2, 3],
        artist: Some(String::from("Nirvana")),
        title: Some(String::from("Smells like Teen Spirit")),
    };

    assert_eq!(vec![1, 2, 3], *my_favorite_song);

    let result = compress_mp3(&my_favorite_song);

    assert_eq!(vec![1, 2, 3], result);
}

fn compress_mp3(audio: &[u8]) -> Vec<u8> {
    Vec::from(audio)
}