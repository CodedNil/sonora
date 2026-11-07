use std::collections::{HashMap, HashSet};

use gpui::{Context, Entity, Task};
use music::{Album, ListeningStats, Ranked, SavedArtist, StatsPeriod, Track, scrobble};
use storage::Database;

use crate::history::Store;
use crate::{AppSettings, History, Io, Session, SessionEvent};

/// Provider rankings or timestamped plays recorded by Sonora on this account.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatsSource {
    Provider,
    Recorded,
    Scrobbler(&'static str),
}

/// Owns stats requests, retained history and date filtering, independently of the view.
pub struct Stats {
    session: Entity<Session>,
    history: Entity<History>,
    settings: Entity<AppSettings>,
    store: Store,
    source: StatsSource,
    period: StatsPeriod,
    recorded: Vec<Track>,
    recorded_provider: Option<String>,
    bounds: Option<(i64, i64)>,
    dates: Option<(i64, i64)>,
    data: ListeningStats,
    loading: bool,
    failed: bool,
    task: Option<Task<()>>,
}

impl Stats {
    pub fn new(
        session: Entity<Session>,
        history: Entity<History>,
        settings: Entity<AppSettings>,
        database: Database,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(&session, |this, _, event, cx| match event {
            SessionEvent::SignedOut | SessionEvent::SignedIn => {
                this.task = None;
                this.recorded.clear();
                this.recorded_provider = None;
                this.bounds = None;
                this.dates = None;
                this.data = ListeningStats::default();
                this.loading = false;
                this.failed = false;
                if let Some(period) = this
                    .session
                    .read(cx)
                    .client()
                    .and_then(|client| client.stats_periods().first().copied())
                {
                    this.period = period;
                }
                this.source = match this
                    .session
                    .read(cx)
                    .client()
                    .is_some_and(|client| !client.stats_periods().is_empty())
                {
                    true => StatsSource::Provider,
                    false => StatsSource::Recorded,
                };
                this.refresh(cx);
            }
            _ => {}
        })
        .detach();
        cx.observe(&history, |this, _, cx| {
            if this.source == StatsSource::Recorded {
                this.refresh(cx);
            }
        })
        .detach();
        cx.observe(&settings, |this, _, cx| {
            if matches!(this.source, StatsSource::Scrobbler(_)) {
                this.refresh(cx);
            }
        })
        .detach();
        let provider_period = session
            .read(cx)
            .client()
            .and_then(|client| client.stats_periods().first().copied());
        let source = match provider_period.is_some() {
            true => StatsSource::Provider,
            false => StatsSource::Recorded,
        };
        Self {
            session,
            history,
            settings,
            store: Store::new(database),
            source,
            period: provider_period.unwrap_or(StatsPeriod::Weeks),
            recorded: Vec::new(),
            recorded_provider: None,
            bounds: None,
            dates: None,
            data: ListeningStats::default(),
            loading: false,
            failed: false,
            task: None,
        }
    }

    pub fn source(&self) -> StatsSource {
        self.source
    }

    /// Linked scrobblers with a read-side stats endpoint, in provider order.
    pub fn scrobblers(&self, cx: &gpui::App) -> Vec<&'static str> {
        let accounts = self.settings.read(cx).scrobbling();
        scrobble::services()
            .into_iter()
            .filter(|service| {
                service.stats_periods().len() > 0
                    && accounts.get(service.id()).is_some_and(|account| account.linked())
            })
            .map(|service| service.id())
            .collect()
    }

    /// Ranking periods for the selected source.
    pub fn periods(&self, cx: &gpui::App) -> Vec<StatsPeriod> {
        match self.source {
            StatsSource::Provider => self.session.read(cx).client()
                .map(|client| client.stats_periods().to_vec()).unwrap_or_default(),
            StatsSource::Scrobbler(id) => scrobble::services().into_iter()
                .find(|service| service.id() == id)
                .map(|service| service.stats_periods().to_vec()).unwrap_or_default(),
            StatsSource::Recorded => vec![StatsPeriod::Weeks, StatsPeriod::Months, StatsPeriod::Year],
        }
    }
    pub fn period(&self) -> StatsPeriod {
        self.period
    }
    pub fn data(&self) -> &ListeningStats {
        &self.data
    }
    pub fn is_loading(&self) -> bool {
        self.loading
    }
    pub fn failed(&self) -> bool {
        self.failed
    }
    pub fn bounds(&self) -> Option<(i64, i64)> {
        self.bounds
    }
    pub fn dates(&self) -> Option<(i64, i64)> {
        self.dates
    }

    pub fn select(&mut self, source: StatsSource, period: StatsPeriod, cx: &mut Context<Self>) {
        self.source = source;
        self.period = match source {
            StatsSource::Provider => self
                .session
                .read(cx)
                .client()
                .filter(|client| client.stats_periods().contains(&period))
                .map_or_else(
                    || {
                        self.session
                            .read(cx)
                            .client()
                            .and_then(|client| client.stats_periods().first().copied())
                            .unwrap_or(period)
                    },
                    |_| period,
                ),
            StatsSource::Recorded => period,
            StatsSource::Scrobbler(id) => scrobble::services().into_iter()
                .find(|service| service.id() == id)
                .map_or(period, |service| {
                    if service.stats_periods().contains(&period) {
                        period
                    } else {
                        service.stats_periods().first().copied().unwrap_or(period)
                    }
                }),
        };
        self.refresh(cx);
    }

    /// Sets an inclusive range of Unix seconds, used only for recorded plays.
    pub fn set_dates(&mut self, dates: (i64, i64), cx: &mut Context<Self>) {
        self.dates = Some(dates);
        self.data = aggregate(
            &self.recorded,
            self.dates,
            self.recorded_provider.as_deref(),
        );
        cx.notify();
    }

    /// Restores the whole recorded log, including plays recorded by later refreshes.
    pub fn all_dates(&mut self, cx: &mut Context<Self>) {
        self.dates = None;
        self.data = aggregate(&self.recorded, None, self.recorded_provider.as_deref());
        cx.notify();
    }

    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        self.task = None;
        self.failed = false;
        self.loading = false;
        self.data = ListeningStats::default();
        let io = Io::global(cx);
        match self.source {
            StatsSource::Provider => {
                let Some(client) = self
                    .session
                    .read(cx)
                    .client()
                    .filter(|client| client.stats_periods().contains(&self.period))
                else {
                    self.source = StatsSource::Recorded;
                    return self.refresh(cx);
                };
                self.loading = true;
                let period = self.period;
                self.task = Some(io.load(
                    async move { client.stats(period).await },
                    |this, result, cx| {
                        this.loading = false;
                        this.task = None;
                        match result {
                            Ok(data) => this.data = data,
                            Err(error) => {
                                log::warn!("stats: cannot load provider rankings: {error:#}");
                                this.failed = true;
                            }
                        }
                        cx.notify();
                    },
                    cx,
                ));
            }
            StatsSource::Scrobbler(id) => {
                let Some(service) = scrobble::services().into_iter().find(|s| s.id() == id) else {
                    self.failed = true;
                    return;
                };
                let account = self.settings.read(cx).account(id);
                if !account.linked() || !service.stats_periods().contains(&self.period) {
                    self.failed = true;
                    return;
                }
                let period = self.period;
                self.loading = true;
                self.task = Some(io.load(
                    async move { service.stats(&account, period).await },
                    |this, result, cx| {
                        this.loading = false;
                        this.task = None;
                        match result {
                            Ok(data) => this.data = data,
                            Err(error) => {
                                log::warn!("stats: cannot load scrobbler rankings: {error:#}");
                                this.failed = true;
                            }
                        }
                        cx.notify();
                    },
                    cx,
                ));
            }
            StatsSource::Recorded => {
                let Some(scope) = self.history.read(cx).scope(cx) else {
                    return;
                };
                let store = self.store.clone();
                self.loading = true;
                self.task = Some(io.load(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            Ok((store.recorded(&scope)?, store.recorded_provider(&scope)?))
                        })
                        .await?
                    },
                    |this, result, cx| {
                        this.loading = false;
                        this.task = None;
                        match result {
                            Ok((tracks, provider)) => {
                                this.recorded = tracks;
                                this.recorded_provider = provider;
                                let low = this.recorded.iter().filter_map(|t| t.added_at).min();
                                let high = this.recorded.iter().filter_map(|t| t.added_at).max();
                                this.bounds = low.zip(high);
                                this.data = aggregate(
                                    &this.recorded,
                                    this.dates,
                                    this.recorded_provider.as_deref(),
                                );
                            }
                            Err(error) => {
                                log::warn!("stats: cannot read recorded plays: {error:#}");
                                this.failed = true;
                            }
                        }
                        cx.notify();
                    },
                    cx,
                ));
            }
        }
        cx.notify();
    }
}

/// Counts starts in the recorded log; each credited artist and each genre receives one hit per play.
fn aggregate(
    tracks: &[Track],
    dates: Option<(i64, i64)>,
    provider: Option<&str>,
) -> ListeningStats {
    let mut songs: HashMap<String, Ranked<Track>> = HashMap::new();
    let mut artists: HashMap<String, Ranked<SavedArtist>> = HashMap::new();
    let mut albums: HashMap<String, Ranked<Album>> = HashMap::new();
    let mut genres: HashMap<String, Ranked<String>> = HashMap::new();
    let mut descriptors: HashMap<String, Ranked<String>> = HashMap::new();
    let mut hours = [0usize; 24];
    let mut days = HashSet::new();
    let mut first_artist: HashMap<String, i64> = HashMap::new();
    for track in tracks {
        let Some(at) = track.added_at else { continue };
        let mut credited = HashSet::new();
        for artist in &track.artist_refs {
            let key = artist.id.clone().unwrap_or_else(|| artist.name.clone());
            if credited.insert(key.clone()) {
                first_artist
                    .entry(key)
                    .and_modify(|first| *first = (*first).min(at))
                    .or_insert(at);
            }
        }
        if credited.is_empty() && !track.artists.is_empty() {
            first_artist
                .entry(track.artists.clone())
                .and_modify(|first| *first = (*first).min(at))
                .or_insert(at);
        }
    }
    let mut new_artists = HashSet::new();
    let mut plays = 0;
    for track in tracks {
        let Some(at) = track.added_at else {
            continue;
        };
        if dates.is_some_and(|(low, high)| at < low || at > high) {
            continue;
        }
        let Some(id) = track.id.as_ref() else {
            continue;
        };
        plays += 1;
        let timestamp = jiff::Timestamp::from_second(at).ok();
        if let Some(local) = timestamp.map(|stamp| stamp.to_zoned(jiff::tz::TimeZone::system())) {
            hours[local.hour() as usize] += 1;
            days.insert(
                local
                    .date()
                    .at(0, 0, 0, 0)
                    .to_zoned(jiff::tz::TimeZone::UTC)
                    .ok()
                    .map(|day| day.timestamp().as_second() / 86_400)
                    .unwrap_or_default(),
            );
        }
        let entry = songs.entry(id.clone()).or_insert_with(|| Ranked {
            item: track.clone(),
            count: Some(0),
        });
        *entry.count.as_mut().unwrap() += 1;
        if !track.album.is_empty() {
            let key = track
                .album_id
                .clone()
                .unwrap_or_else(|| track.album.clone());
            let entry = albums.entry(key.clone()).or_insert_with(|| Ranked {
                item: Album {
                    id: key,
                    name: track.album.clone(),
                    artists: track.artists.clone(),
                    artist_refs: track.artist_refs.clone(),
                    cover: track.cover.clone(),
                    ..Default::default()
                },
                count: Some(0),
            });
            *entry.count.as_mut().unwrap() += 1;
        }
        let mut credited = HashSet::new();
        for artist in &track.artist_refs {
            let key = artist.id.clone().unwrap_or_else(|| artist.name.clone());
            if !credited.insert(key.clone()) {
                continue;
            }
            if first_artist.get(&key) == Some(&at) {
                new_artists.insert(key.clone());
            }
            let entry = artists.entry(key).or_insert_with(|| Ranked {
                item: SavedArtist {
                    id: artist.id.clone().unwrap_or_default(),
                    name: artist.name.clone(),
                    ..Default::default()
                },
                count: Some(0),
            });
            *entry.count.as_mut().unwrap() += 1;
        }
        if track.artist_refs.is_empty() && !track.artists.is_empty() {
            if first_artist.get(&track.artists) == Some(&at) {
                new_artists.insert(track.artists.clone());
            }
            let entry = artists
                .entry(track.artists.clone())
                .or_insert_with(|| Ranked {
                    item: SavedArtist {
                        name: track.artists.clone(),
                        ..Default::default()
                    },
                    count: Some(0),
                });
            *entry.count.as_mut().unwrap() += 1;
        }
        let mut tagged = HashSet::new();
        let is_descriptor_source = provider == Some("spotify");
        for genre in &track.tags {
            let key = genre.trim().to_lowercase();
            if key.is_empty() || key == "music" || !tagged.insert(key.clone()) {
                continue;
            }
            let target = if is_descriptor_source {
                &mut descriptors
            } else {
                &mut genres
            };
            let entry = target.entry(key).or_insert_with(|| Ranked {
                item: genre.trim().to_owned(),
                count: Some(0),
            });
            *entry.count.as_mut().unwrap() += 1;
        }
    }
    let unique_tracks = songs.len();
    let unique_artists = artists.len();
    let mut data = ListeningStats {
        tracks: songs.into_values().collect(),
        artists: artists.into_values().collect(),
        albums: albums.into_values().collect(),
        genres: genres.into_values().collect(),
        descriptors: descriptors.into_values().collect(),
        plays: Some(plays),
        unique_tracks: Some(unique_tracks),
        unique_artists: Some(unique_artists),
        new_artists: Some(new_artists.len()),
        active_days: Some(days.len()),
        longest_streak: Some(longest_streak(&days)),
        peak_hour: hours
            .iter()
            .enumerate()
            .max_by_key(|(_, count)| *count)
            .filter(|(_, count)| **count > 0)
            .map(|(hour, _)| hour as u8),
        hourly: hours.to_vec(),
    };
    data.tracks.sort_by(|a, b| {
        b.count
            .cmp(&a.count)
            .then_with(|| a.item.name.cmp(&b.item.name))
            .then_with(|| a.item.id.cmp(&b.item.id))
    });
    data.artists.sort_by(|a, b| {
        b.count
            .cmp(&a.count)
            .then_with(|| a.item.name.cmp(&b.item.name))
            .then_with(|| a.item.id.cmp(&b.item.id))
    });
    data.albums.sort_by(|a, b| {
        b.count
            .cmp(&a.count)
            .then_with(|| a.item.name.cmp(&b.item.name))
    });
    data.genres
        .sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.item.cmp(&b.item)));
    data.descriptors
        .sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.item.cmp(&b.item)));
    data
}

/// Finds the longest run of consecutive local calendar days with a recorded start.
fn longest_streak(days: &HashSet<i64>) -> usize {
    let mut days = days.iter().copied().collect::<Vec<_>>();
    days.sort_unstable();
    let mut longest = 0;
    let mut current = 0;
    let mut previous = None;
    for day in days {
        current = match previous {
            Some(previous) if day == previous + 1 => current + 1,
            _ => 1,
        };
        longest = longest.max(current);
        previous = Some(day);
    }
    longest
}
