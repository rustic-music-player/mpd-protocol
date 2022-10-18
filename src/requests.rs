#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    Command(Command),
    CommandList(Vec<Command>, bool)
}

#[derive(Debug, Clone, PartialEq)]
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
    ListInfo(String),
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConsumeState {
    True,
    False,
    Oneshot
}
