use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Index, IndexList, MultiIndex};

#[cw_serde]
pub struct SongInfo {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub genre: String,
    pub album: String
}

pub struct SongIndexes<'a> {
    pub artist: MultiIndex<'a, String, SongInfo, String>,
    pub genre: MultiIndex<'a, String, SongInfo, String>,
    pub album: MultiIndex<'a, String, SongInfo, String>
}

impl SongIndexes<'_> {
    fn song_artist_idx(_: &[u8], d: &SongInfo) -> String {
        return d.artist.clone()
    }
    fn song_genre_idx(_: &[u8], d: &SongInfo) -> String {
        return d.genre.clone()
    }
    fn song_album_idx(_: &[u8], d: &SongInfo) -> String {
        return d.album.clone()
    }
    pub fn new() -> Self {
        return Self {
            artist: MultiIndex::new(SongIndexes::song_artist_idx, "SONG", "SONG__ARTIST"),
            genre: MultiIndex::new(SongIndexes::song_genre_idx, "SONG", "SONG__GENRE"),
            album: MultiIndex::new(SongIndexes::song_album_idx, "SONG", "SONG__ALBUM"),
        }
    }
}

impl IndexList<SongInfo> for SongIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item=&'_ dyn Index<SongInfo>> + '_> {
        let v: Vec<&dyn Index<SongInfo>> = vec![&self.artist, &self.genre, &self.album];
        return Box::new(v.into_iter());
    }
}

