# CrateTV

A simple game streaming service built with `SvelteKit` and `Axum`

## Layout

| Directory       | Type         | Language            | Description                                                                                             |
| --------------- | ------------ | ------------------- | ------------------------------------------------------------------------------------------------------- |
| crate_tv        | Web Frontend | Svelte + Typescript | A client for watching streams using HLS                                                                 |
| crate_tv_app    | App Frontend | Rust                | A webview wrapper using `tao` and `wry` to display the website as a native app                          |
| crate_tv_server | Backend      | Rust                | A server for both RTMP (streamers) and HLS (viewers) that handles live transcoding and VOD distribution |
