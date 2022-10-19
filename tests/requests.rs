use test_case::test_case;
use mpd_protocol::{Command, Request, parse, ReplayGainMode};

#[test_case("idle\n", Default::default())]
#[test_case("idle \"database\"\n", vec!["database"])]
#[test_case("idle \"mixer\" \"update\"\n", vec!["mixer", "update"])]
fn idle_command(input: &str, systems: Vec<&str>) {
    let expected = Command::Idle(systems.into_iter().map(|system| system.to_string()).collect());

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn noidle_command() {
    let input = "noidle\n";
    let expected = Command::NoIdle;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn current_song_command() {
    let input = "currentsong\n";
    let expected = Command::CurrentSong;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn status_command() {
    let input = "status\n";
    let expected = Command::Status;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn stats_command() {
    let input = "stats\n";
    let expected = Command::Stats;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("random 1\n", true)]
#[test_case("random 0\n", false)]
fn random_command(input: &str, random: bool) {
    let expected = Command::Random(random);

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("repeat 1\n", true)]
#[test_case("repeat 0\n", false)]
fn repeat_command(input: &str, repeat: bool) {
    let expected = Command::Repeat(repeat);

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn get_volume_command() {
    let input = "getvol\n";
    let expected = Command::GetVolume;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("setvol 0\n", 0)]
#[test_case("setvol 100\n", 100)]
fn set_volume_command(input: &str, volume: u32) {
    let expected = Command::SetVolume(volume);

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("volume 0\n", 0)]
#[test_case("volume 100\n", 100; "positive volume")]
#[test_case("volume \"100\"\n", 100; "positive volume with quotes")]
#[test_case("volume -5\n", -5; "negative volume")]
#[test_case("volume \"-5\"\n", -5; "negative volume with quotes")]
fn change_volume_command(input: &str, volume: i32) {
    let expected = Command::ChangeVolumeBy(volume);

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn stop_command() {
    let input = "stop\n";
    let expected = Command::Stop;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn next_command() {
    let input = "next\n";
    let expected = Command::Next;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn previous_command() {
    let input = "previous\n";
    let expected = Command::Previous;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("pause\n", None)]
#[test_case("pause 1\n", Some(true))]
#[test_case("pause 0\n", Some(false))]
fn pause_command(input: &str, expected: Option<bool>) {
    let expected = Command::Pause(expected);

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("play\n", None)]
#[test_case("play 1\n", Some(1))]
#[test_case("play 50\n", Some(50))]
fn play_command(input: &str, expected: Option<u32>) {
    let expected = Command::Play(expected);

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("playid\n", None)]
#[test_case("playid 1\n", Some(1))]
#[test_case("playid 50\n", Some(50))]
fn play_id_command(input: &str, expected: Option<u32>) {
    let expected = Command::PlayId(expected);

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn commands_command() {
    let input = "commands\n";
    let expected = Command::Commands;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn tag_types_command() {
    let input = "tagtypes\n";
    let expected = Command::TagTypes;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn close_command() {
    let input = "close\n";
    let expected = Command::Close;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn clear_command() {
    let input = "clear\n";
    let expected = Command::Clear;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn list_playlists_command() {
    let input = "listplaylists\n";
    let expected = Command::ListPlaylists;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn outputs_command() {
    let input = "outputs\n";
    let expected = Command::Outputs;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn decoders_command() {
    let input = "decoders\n";
    let expected = Command::Decoders;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn playlist_info_command() {
    let input = "playlistinfo\n";
    let expected = Command::PlaylistInfo;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("plchanges \"0\"\n", "0")]
#[test_case("plchanges \"1\"\n", "1")]
fn playlist_changes_command(input: &str, version: &str) {
    let expected = Command::PlaylistChanges(version.to_string());

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("lsinfo\n", None)]
#[test_case("lsinfo \"\"\n", Some(""))]
#[test_case("lsinfo \"some path\"\n", Some("some path"))]
fn list_info_command(input: &str, uri: Option<&str>) {
    let expected = Command::ListInfo(uri.map(|uri| uri.to_string()));

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("load \"playlist\"\n", "playlist")]
#[test_case("load \"name\"\n", "name")]
fn load_playlist_command(input: &str, name: &str) {
    let expected = Command::LoadPlaylist(name.to_string());

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("list \"Artist\"\n", "Artist"; "list_artists_with_quotes")]
#[test_case("list Artist\n", "Artist"; "list_artists_without_quotes")]
#[test_case("list album\n", "album")]
fn list_command(input: &str, r#type: &str) {
    let expected = Command::List(r#type.to_string());

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("find \"Artist\"\n", "Artist"; "artists with quotes")]
#[test_case("find Artist\n", "Artist"; "artists without quotes")]
#[test_case("find album\n", "album")]
fn find_command(input: &str, filter: &str) {
    let expected = Command::Find(filter.to_string());

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}


#[test_case("listplaylistinfo \"Playlist\"\n", "Playlist"; "Playlist with quotes")]
#[test_case("listplaylistinfo Playlist\n", "Playlist"; "Playlist without quotes")]
#[test_case("listplaylistinfo \"Liked Songs\"\n", "Liked Songs")]
fn list_playlist_info_command(input: &str, playlist: &str) {
    let expected = Command::ListPlaylistInfo(playlist.to_string());

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("addid \"uri\"\n", "uri")]
#[test_case("addid \"id\"\n", "id")]
fn add_id_command(input: &str, uri: &str) {
    let expected = Command::AddId(uri.to_string());

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("albumart \"uri\" \"0\"\n", "uri", 0)]
#[test_case("albumart \"file\" 50\n", "file", 50)]
fn albumart_command(input: &str, uri: &str, offset: u32) {
    let expected = Command::AlbumArt(uri.to_string(), offset);

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn list_partitions_command() {
    let input = "listpartitions\n";
    let expected = Command::ListPartitions;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn url_handlers_command() {
    let input = "urlhandlers\n";
    let expected = Command::UrlHandlers;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test]
fn replay_gain_status_command() {
    let input = "replay_gain_status\n";
    let expected = Command::ReplayGainStatus;

    let result = parse(input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}

#[test_case("off", ReplayGainMode::Off)]
#[test_case("track", ReplayGainMode::Track)]
#[test_case("album", ReplayGainMode::Album)]
#[test_case("auto", ReplayGainMode::Auto)]
fn replay_gain_mode_command(mode: &str, expected: ReplayGainMode) {
    let input = format!("replay_gain_mode {mode}\n");
    let expected = Command::ReplayGainMode(expected);

    let result = parse(&input);

    assert_eq!(Ok(("", Request::Command(expected))), result)
}
