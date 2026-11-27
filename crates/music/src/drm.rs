//! The Widevine module, as the rest of the app has to talk about it.
//!
//! The CDM is installed separately and used by providers through the shared playback engine.

pub use widevine::{
    CDM_PATH, Cdm, Found, Offer, Origin, available, cenc, fetch, find, host, hosted, offer,
    require, supported, uninstall,
};
