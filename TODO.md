# TODO

The current broad goal is to get streams working end-to-end. This involves setting up the RTMP server, transcoding live feeds into HLS segments, saving them, then streaming them from disk or live to clients depending on whether the user is watching live or not.

## Server

- [ ] RTMP server with stream key authentication
- [ ] Live transcoding from RTMP to HLS
- [ ] Streaming of live HLS feed to live cleints
- [ ] HLS stream live saving to m3u8 VOD on disk
- [ ] VOD database with VOD information and path to VOD disk location
- [ ] HLS seeking from disk for live streams and VODs
- [x] Authentication and account system
  - [x] Login hash database
  - [x] Session-based auth (replaced JWT with actix-session)
  - [x] Automatic expired session cleanup (background task, runs hourly)
  - [x] Permissions wired up on stream-key endpoints
  - [ ] Permissions wired up on remaining endpoints
  - [x] Stream key generation (GET /stream-key, POST /stream-key/regenerate)

## Client

- [x] Home page that displays all streams
- [x] Sidebar that displays followed streams
- [x] HLS client
- [x] Account page — login / sign up / logout
  - [x] Stream key display (reveal, show/hide, copy, regenerate)
- [ ] Better video player with native quality switching and seeking
- [ ] Search bar with fuzzy searching

## Misc

- [ ] `permissions_to_add.md` can be deleted once the permission system is wired up to endpoints
