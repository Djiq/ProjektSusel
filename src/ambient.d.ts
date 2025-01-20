type Song = {
    uuid: number,
    name: string,
    path: string,
    album: string | null,
    author: string | null
};

type Playlist = {
    id: number,
    name: string,
    desc: string | null,
    current_song_index: number | null,
    songs: Uuid[]
};

type PlaylistQueue = {
    currentIndex: number,
    jumps: number,
    prime: number,
    playlist: Playlist
}

type QueueEntry = Song | PlaylistQueue;