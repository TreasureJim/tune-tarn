mod yt;

use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, path::PathBuf};

use axum::Router;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    let libraries = yt_dlp::client::Libraries::new(PathBuf::from("libraries/yt"), PathBuf::from("libraries/ffmpeg"));
    let yt = yt_dlp::Youtube::new(libraries, PathBuf::from("media")).await.unwrap();

    let path = yt.get_video_by_id("mz6EU3Wdebw").await;
    yt_dlp::download::fetcher::Fetcher::
    dbg!(path);

    // let new_path = yt.download_audio_stream_from_url("https://www.youtube.com/watch?v=mz6EU3Wdebw".to_string(), "test/audio.mp3").await.unwrap();
    // dbg!(&new_path);

    return;

    let router = Router::new()
        .nest_service("/media", ServeDir::new("media"))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
    log::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
