use std::rc::Rc;

use gpui::prelude::*;
use gpui::{App, Entity, SharedString, div};
use music::Track;
use state::Playback;
use ui::{ActiveTheme as _, Button, Card, Text, clock, tabular};

use crate::shared::cards;

pub(crate) type StartHandler = Rc<dyn Fn(usize, &mut App)>;
pub(crate) type RemoveHandler = Rc<dyn Fn(usize, &mut App)>;

pub(crate) struct TrackCard {
    id: &'static str,
    place: usize,
    tracks: Rc<Vec<Track>>,
    playback: Entity<Playback>,
    detailed: bool,
    start: Option<StartHandler>,
    /// The Fluent key naming the remove button, and what it does with the track's place.
    remove: Option<(&'static str, RemoveHandler)>,
}

impl TrackCard {
    pub(crate) fn new(
        id: &'static str,
        place: usize,
        tracks: Rc<Vec<Track>>,
        playback: Entity<Playback>,
    ) -> Self {
        Self {
            id,
            place,
            tracks,
            playback,
            detailed: false,
            start: None,
            remove: None,
        }
    }

    pub(crate) fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = detailed;
        self
    }

    pub(crate) fn start(mut self, start: Option<StartHandler>) -> Self {
        self.start = start;
        self
    }

    /// Reserves room for an X shown on hover so durations stay aligned. `tooltip` is its Fluent key.
    pub(crate) fn remove(mut self, tooltip: &'static str, remove: RemoveHandler) -> Self {
        self.remove = Some((tooltip, remove));
        self
    }

    pub(crate) fn render(self, cx: &App) -> Card {
        let theme = *cx.theme();
        let track = &self.tracks[self.place];
        let (current, playing) = cards::track_status(track, &self.playback, cx);
        let transport_playback = self.playback.clone();
        let pressed_start = self.start.unwrap_or_else(|| {
            let tracks = self.tracks.clone();
            let playback = self.playback.clone();
            Rc::new(move |place, cx| {
                playback.update(cx, |playback, cx| playback.play_radio(&tracks[place], cx));
            })
        });
        let transport_start = pressed_start.clone();
        let place = self.place;

        let artists = (!self.detailed || featured(track))
            .then(|| cards::track_artists(SharedString::new_static("pick-artist"), track, &theme));
        let length = self.detailed.then(|| {
            div()
                .text_size(theme.text(Text::Small))
                .text_color(theme.muted_foreground)
                .font_features(tabular())
                .when_some(track.duration, |this, duration| this.child(clock(duration)))
        });

        let remove = self.remove.map(|(tooltip, remove)| {
            Button::new("track-card-remove")
                .ghost()
                .small()
                .icon("icons/x.svg")
                .tooltip(tooltip)
                .tint(theme.muted_foreground)
                .on_click(move |_, _, cx| {
                    cx.stop_propagation();
                    remove(place, cx);
                })
        });

        let total = self.tracks.len();

        cards::track_card((self.id, place), track, &self.playback, cx)
            .aria_position_in_set(place + 1)
            .aria_size_of_set(total)
            .when_some(artists, Card::bare_meta)
            .when_some(length, Card::trailing)
            .when_some(remove, Card::collapsed_action)
            .play(playing, move |_, _, cx| match current {
                true => transport_playback.update(cx, |playback, cx| playback.toggle_play(cx)),
                false => transport_start(place, cx),
            })
            .press(move |_, _, cx| pressed_start(place, cx))
            .min_w_0()
    }
}

fn featured(track: &Track) -> bool {
    match track.artist_refs.is_empty() {
        false => track.artist_refs.len() > 1,
        true => track.artists.contains(','),
    }
}
