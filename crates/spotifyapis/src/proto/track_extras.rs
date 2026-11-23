use anyhow::Result;
use bytes::Bytes;
use reqwest::{Method, header::HeaderMap};

use crate::{Session, http, id};

/// A track's optional looping visual, as returned by Spotify's Canvas service.
#[derive(prost::Message)]
pub struct Canvas {
    #[prost(string, tag = "2")]
    pub url: String,
    #[prost(int32, tag = "4")]
    pub kind: i32,
    #[prost(string, tag = "5")]
    entity_uri: String,
}

#[derive(prost::Message)]
struct CanvasRequest {
    #[prost(message, repeated, tag = "1")]
    tracks: Vec<CanvasTrack>,
}

#[derive(prost::Message)]
struct CanvasTrack {
    #[prost(string, tag = "1")]
    uri: String,
}

#[derive(prost::Message)]
struct CanvasResponse {
    #[prost(message, repeated, tag = "1")]
    canvases: Vec<Canvas>,
}

impl Session {
    /// Loads the Canvas for one track, if the service returns a matching visual.
    pub async fn track_canvas(&self, track_id: &str) -> Result<Option<Canvas>> {
        let uri = id::track_uri(track_id);
        let request = CanvasRequest {
            tracks: vec![CanvasTrack { uri: uri.clone() }],
        };
        let response: CanvasResponse =
            http::read_rpc(self, "/canvaz-cache/v0/canvases", &request).await?;
        Ok(response.canvases.into_iter().find(|canvas| {
            !canvas.url.is_empty() && (canvas.entity_uri.is_empty() || canvas.entity_uri == uri)
        }))
    }

    /// Reads the full credits response from the desktop track credits service.
    pub async fn track_credits(&self, track_id: &str) -> Result<Bytes> {
        anyhow::ensure!(
            !track_id.is_empty() && track_id.bytes().all(|byte| byte.is_ascii_alphanumeric()),
            "invalid Spotify track ID"
        );
        let path = format!("/track-credits-view/v0/experimental/{track_id}/credits");
        http::send(self, Method::GET, &path, HeaderMap::new(), Bytes::new()).await
    }
}
