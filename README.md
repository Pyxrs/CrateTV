# CrateTV

A simple game streaming service built with `SvelteKit` and `Axum`

## Layout

| Directory       | Type         | Language            | Description                                                                                             |
| --------------- | ------------ | ------------------- | ------------------------------------------------------------------------------------------------------- |
| crate_tv        | Web Frontend | Svelte + Typescript | A client for watching streams using HLS                                                                 |
| crate_tv_app    | App Frontend | Rust                | A webview wrapper using `tao` and `wry` to display the website as a native app                          |
| crate_tv_server | Backend      | Rust                | A server for both RTMP (streamers) and HLS (viewers) that handles live transcoding and VOD distribution |

## Todo

### Client

- [x] Home page that displays all streams
- [x] Sidebar that displays followed streams
- [x] HLS client
- [ ] Better video player with native quality
- [ ] Account page
  - [ ] Log in / Sign up system
  - [ ] Stream key
- [ ] Search bar that displays best matches

### Server

- [x] Authentication and account system
  - [x] Login hash database
    - [ ] Permissions
    - [ ] Misc data blob using bitcode (don't update major version due to incompatibility)
  - [ ] Stream key generation
- [ ] RTMP server
  - [ ] Stream key authentication
- [ ] HLS server

And much more...
