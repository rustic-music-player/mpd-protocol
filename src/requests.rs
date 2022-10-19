#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Command(Command),
    CommandList(Vec<Command>, bool)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Idle(Vec<String>),
    NoIdle,
    CurrentSong,
    Status,
    Stats,
    Random(bool),
    Repeat(bool),
    GetVolume,
    SetVolume(u32),
    ChangeVolumeBy(i32),
    PlaylistChanges(String),
    Outputs,
    Decoders,
    ListPlaylists,
    ListPlaylist(String),
    ListPlaylistInfo(String),
    LoadPlaylist(String),
    /**
     * Displays a list of all songs in the playlist, or if the optional argument is given, displays information only for the song SONGPOS or the range of songs START:END
     */
    PlaylistInfo,
    /**
     * Lists the contents of the directory URI. The response contains records starting with file, directory or playlist, each followed by metadata (tags or other metadata).
     *
     * When listing the root directory, this currently returns the list of stored playlists. This behavior is deprecated; use “listplaylists” instead.
     *
     * This command may be used to list metadata of remote files (e.g. URI beginning with “http://” or “smb://”).
     *
     * Clients that are connected via local socket may use this command to read the tags of an arbitrary local file (URI is an absolute path).
     */
    ListInfo(Option<String>),
    Previous,
    Next,
    Pause(Option<bool>),
    Play(Option<u32>),
    PlayId(Option<u32>),
    Stop,
    List(String),
    Find(String),
    Add(String),
    AddId(String),
    Commands,
    TagTypes,
    /**
     * Closes the connection to MPD. MPD will try to send the remaining output buffer before it actually closes the connection, but that cannot be guaranteed. This command will not generate a response.
     *
     * Clients should not use this command; instead, they should just close the socket.
     */
    Close,
    ListPartitions,
    UrlHandlers,
    /**
     * Clears the queue.
     */
    Clear,
    /**
     * Locate album art for the given song and return a chunk of an album art image file at offset OFFSET.
     *
     * This is currently implemented by searching the directory the file resides in for a file called cover.png, cover.jpg, cover.tiff or cover.bmp.
     *
     * Returns the file size and actual number of bytes read at the requested offset, followed by the chunk requested as raw bytes (see Binary Responses), then a newline and the completion code.
     *
     * ## Example
     * ```mpd
     * albumart foo/bar.ogg 0
     * size: 1024768
     * binary: 8192
     * <8192 bytes>
     * OK
     * ```
     */
    AlbumArt(String, u32),
    ReplayGainMode(ReplayGainMode),
    ReplayGainStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsumeState {
    True,
    False,
    Oneshot
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayGainMode {
    Off,
    Track,
    Album,
    Auto,
}
