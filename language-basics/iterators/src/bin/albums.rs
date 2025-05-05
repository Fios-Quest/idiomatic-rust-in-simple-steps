// ANCHOR: Albums
struct Album {
    name: String,
    artist: String,
}

struct Albums(Vec<Album>);

impl Albums {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn buy(&mut self, album: Album) {
        self.0.push(album);
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}
// ANCHOR_END: Albums

// ANCHOR: IntoIterator
impl IntoIterator for Albums {
    type Item = Album;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
// ANCHOR_END: IntoIterator

// ANCHOR: FromIterator
use std::iter::FromIterator;

impl FromIterator<Album> for Albums {
    fn from_iter<I: IntoIterator<Item = Album>>(iter: I) -> Self {
        let mut albums = Self::new();

        for album in iter {
            albums.buy(album);
        }

        albums
    }
}
// ANCHOR_END: FromIterator

fn main() {

    // ANCHOR: UseIntoIterator
    let mut albums = Albums::new();

    albums.buy(Album {
        name: "Sgt. Pepper's Lonely Hearts Club Band".into(),
        artist: "The Beatles".into(),
    });

    albums.buy(Album {
        name: "Back in Black".into(),
        artist: "AC/DC".into(),
    });

    albums.buy(Album {
        name: "Hotel California".into(),
        artist: "Eagles".into(),
    });

    // Get the artist names, the obviously consumes the collection:
    let artists: Vec<_> = albums
        .into_iter()
        .map(|Album { artist, .. }| artist)
        .collect();

    assert_eq!(
        artists,
        vec!["The Beatles".to_string(), "AC/DC".to_string(), "Eagles".to_string()],
    );
    // ANCHOR_END: UseIntoIterator

    // ANCHOR: UseFromIterator
    let raw_albums = vec![
        Album {
            name: "Sgt. Pepper's Lonely Hearts Club Band".into(),
            artist: "The Beatles".into(),
        },

        Album {
            name: "Back in Black".into(),
            artist: "AC/DC".into(),
        },

        Album {
            name: "Hotel California".into(),
            artist: "Eagles".into(),
        },
    ];

    let albums: Albums = raw_albums.into_iter().collect();
    assert_eq!(albums.count(), 3);
    // ANCHOR_END: UseFromIterator
}
