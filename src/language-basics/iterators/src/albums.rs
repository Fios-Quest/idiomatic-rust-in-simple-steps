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

impl IntoIterator for Albums {
    type Item = Album;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Album> for Albums {
    fn from_iter<I: IntoIterator<Item = Album>>(iter: I) -> Self {
        let mut albums = Self::new();

        for album in iter {
            albums.buy(album);
        }

        albums
    }
}
