mod link;
mod navigation;

pub use link::Link;
pub use navigation::{Navigation, NavigationEvent};

use gpui::{App, AppContext as _, Entity, Global, SharedString};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LibraryTab {
    Songs,
    Albums,
    Playlists,
    Artists,
}

impl LibraryTab {
    /// The glyph of what the tab lists, the same one a pin of that kind falls back to.
    pub const fn icon(self) -> &'static str {
        match self {
            Self::Songs => ui::PinKind::Song.icon(),
            Self::Albums => ui::PinKind::Album.icon(),
            Self::Playlists => ui::PinKind::Playlist.icon(),
            Self::Artists => ui::PinKind::Artist.icon(),
        }
    }
}

/// A navigation entry or toolbar control group that can be placed in app chrome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavEntry {
    Home,
    Search,
    Library,
    Local,
    History,
    Pins,
    Settings,
    Transport,
}

impl NavEntry {
    pub const ALL: [Self; 8] = [
        Self::Transport,
        Self::Home,
        Self::Search,
        Self::Library,
        Self::Local,
        Self::History,
        Self::Pins,
        Self::Settings,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Search => "search",
            Self::Library => "library",
            Self::Local => "local",
            Self::History => "history",
            Self::Pins => "sidebar-pins",
            Self::Settings => "settings",
            Self::Transport => "transport",
        }
    }

    /// Whether the row is available as an optional sidebar shortcut.
    pub fn optional(self) -> bool {
        matches!(self, Self::Settings)
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Home => "nav-home",
            Self::Search => "common-search",
            Self::Library => "nav-library",
            Self::Local => "common-local-files",
            Self::History => "nav-history",
            Self::Settings => "nav-settings",
            Self::Pins => "nav-pinned",
            Self::Transport => "nav-transport",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Home => "icons/house.svg",
            Self::Search => "icons/search.svg",
            Self::Library => "icons/library-big.svg",
            Self::History => "icons/rotate-ccw-clock.svg",
            Self::Local => "icons/file-music.svg",
            Self::Pins => "icons/pin.svg",
            Self::Settings => "icons/settings.svg",
            Self::Transport => "icons/chevron-left.svg",
        }
    }

    /// Where the entry leads, or `None` for the pins, which are a list rather than a page.
    pub fn destination(self) -> Option<Destination> {
        match self {
            Self::Home => Some(Destination::Home),
            Self::Search => Some(Destination::Search),
            Self::Library => Some(Destination::Library(LibraryTab::Songs)),
            Self::History => Some(Destination::History),
            Self::Local => Some(Destination::Local(LibraryTab::Songs)),
            Self::Pins | Self::Transport => None,
            Self::Settings => Some(Destination::Settings(SettingsPage::General)),
        }
    }

    /// Where the entry sits until the user moves it.
    pub fn default_place(self) -> Place {
        match self {
            Self::Home | Self::Search => Place::Top,
            Self::Settings => Place::TopRight,
            Self::Transport => Place::TopLeft,
            _ => Place::Sidebar,
        }
    }

    /// The places the entry can go. The pins are a whole list, which only the sidebar holds.
    pub fn places(self) -> &'static [Place] {
        match self {
            Self::Pins => &[Place::Sidebar, Place::Hidden],
            Self::Transport => &[Place::TopLeft, Place::Top, Place::TopRight, Place::Hidden],
            _ => &Place::ALL,
        }
    }
}

/// Where a navigation entry is shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    Sidebar,
    /// The centered toolbar region. The legacy stored id `top` also maps here.
    Top,
    TopLeft,
    TopRight,
    Hidden,
}

impl Place {
    pub const ALL: [Self; 5] = [
        Self::TopLeft,
        Self::Top,
        Self::TopRight,
        Self::Sidebar,
        Self::Hidden,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Sidebar => "sidebar",
            Self::Top => "top-center",
            Self::TopLeft => "top-left",
            Self::TopRight => "top-right",
            Self::Hidden => "hidden",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "top" => Some(Self::Top),
            _ => Self::ALL.into_iter().find(|place| place.id() == id),
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Sidebar => "nav-place-sidebar",
            Self::Top => "nav-place-top-center",
            Self::TopLeft => "nav-place-top-left",
            Self::TopRight => "nav-place-top-right",
            Self::Hidden => "nav-place-hidden",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Home,
    Search,
    History,
    Songs,
    Albums,
    Playlists,
    Artists,
}

impl Screen {
    pub const ALL: [Self; 7] = [
        Self::Home,
        Self::Search,
        Self::Songs,
        Self::Albums,
        Self::Artists,
        Self::Playlists,
        Self::History,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Search => "search",
            Self::History => "history",
            Self::Songs => "songs",
            Self::Albums => "albums",
            Self::Playlists => "playlists",
            Self::Artists => "artists",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Home => "nav-home",
            Self::Search => "common-search",
            Self::History => "nav-history",
            Self::Songs => "nav-songs",
            Self::Albums => "nav-albums",
            Self::Playlists => "nav-playlists",
            Self::Artists => "nav-artists",
        }
    }

    /// The screen a stored id names. The local music screen joined Songs, so its old id opens there.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "imported" => Some(Self::Songs),
            id => Self::ALL.into_iter().find(|screen| screen.id() == id),
        }
    }

    pub fn destination(self) -> Destination {
        match self {
            Self::Home => Destination::Home,
            Self::Search => Destination::Search,
            Self::History => Destination::History,
            Self::Songs => Destination::Library(LibraryTab::Songs),
            Self::Albums => Destination::Library(LibraryTab::Albums),
            Self::Playlists => Destination::Library(LibraryTab::Playlists),
            Self::Artists => Destination::Library(LibraryTab::Artists),
        }
    }
}

/// One page of settings. The first thirteen are sections of the six tabs, each tab showing its sections
/// one after another on one scrolling page, and the last two are drill-ins that open from a row
/// on their parent page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingsPage {
    Providers,
    General,
    Appearance,
    PlayerBar,
    NowPlaying,
    Lyrics,
    Playback,
    Sound,
    Storage,
    Discord,
    Scrobbling,
    About,
    Keybinds,
    LyricsSources,
    MusicFolders,
}

impl SettingsPage {
    /// Every page, the sections first and the drill-ins after them.
    pub const ALL: [Self; 15] = [
        Self::Providers,
        Self::General,
        Self::Appearance,
        Self::PlayerBar,
        Self::NowPlaying,
        Self::Lyrics,
        Self::Playback,
        Self::Sound,
        Self::Storage,
        Self::Discord,
        Self::Scrobbling,
        Self::About,
        Self::Keybinds,
        Self::LyricsSources,
        Self::MusicFolders,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Providers => "providers",
            Self::General => "general",
            Self::Appearance => "appearance",
            Self::PlayerBar => "player-bar",
            Self::NowPlaying => "now-playing",
            Self::Lyrics => "lyrics",
            Self::Playback => "playback",
            Self::Sound => "sound",
            Self::Storage => "storage",
            Self::Discord => "discord",
            Self::Scrobbling => "scrobbling",
            Self::About => "about",
            Self::Keybinds => "keybinds",
            Self::LyricsSources => "lyrics-sources",
            Self::MusicFolders => "music-folders",
        }
    }

    /// The Fluent key of the page's name.
    pub fn key(self) -> &'static str {
        match self {
            Self::Providers => "settings-page-providers",
            Self::General => "settings-page-general",
            Self::Appearance => "settings-page-appearance",
            Self::PlayerBar => "settings-page-player-bar",
            Self::NowPlaying => "queue-now-playing",
            Self::Lyrics => "lyrics-title",
            Self::Playback => "settings-page-playback",
            Self::Sound => "settings-page-sound",
            Self::Storage => "settings-page-storage",
            Self::Discord => "settings-page-discord",
            Self::Scrobbling => "settings-page-scrobbling",
            Self::About => "settings-page-about",
            Self::Keybinds => "settings-page-keybinds",
            Self::LyricsSources => "settings-lyrics-sources",
            Self::MusicFolders => "settings-music-folders",
        }
    }

    /// The page a drill-in returns to, or `None` for a section of a tab.
    pub fn parent(self) -> Option<Self> {
        match self {
            Self::LyricsSources => Some(Self::Lyrics),
            Self::MusicFolders => Some(Self::Providers),
            _ => None,
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|page| page.id() == id)
    }

    /// The tab the page shows in. A drill-in belongs to its parent's tab.
    pub fn tab(self) -> SettingsTab {
        match self {
            Self::Providers | Self::MusicFolders | Self::General | Self::About => {
                SettingsTab::General
            }
            Self::Appearance | Self::PlayerBar | Self::NowPlaying => SettingsTab::Appearance,
            Self::Lyrics | Self::LyricsSources => SettingsTab::Lyrics,
            Self::Playback | Self::Sound | Self::Storage => SettingsTab::Playback,
            Self::Discord | Self::Scrobbling => SettingsTab::Integrations,
            Self::Keybinds => SettingsTab::Keybinds,
        }
    }
}

/// One tab of the settings header. A tab is one scrolling page of its sections, each a former
/// page under its own title.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingsTab {
    General,
    Appearance,
    Lyrics,
    Playback,
    Integrations,
    Keybinds,
}

impl SettingsTab {
    /// Every tab, in the order the header shows them.
    pub const ALL: [Self; 6] = [
        Self::General,
        Self::Appearance,
        Self::Lyrics,
        Self::Playback,
        Self::Integrations,
        Self::Keybinds,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Appearance => "appearance",
            Self::Lyrics => "lyrics",
            Self::Playback => "playback",
            Self::Integrations => "integrations",
            Self::Keybinds => "keybinds",
        }
    }

    /// The Fluent key of the tab's name.
    pub fn key(self) -> &'static str {
        match self {
            Self::General => "settings-page-general",
            Self::Appearance => "settings-page-appearance",
            Self::Lyrics => "lyrics-title",
            Self::Playback => "settings-page-playback",
            Self::Integrations => "settings-tab-integrations",
            Self::Keybinds => "settings-page-keybinds",
        }
    }

    /// The icon before the tab's name in the header, and beside a search result found in it.
    pub fn icon(self) -> &'static str {
        match self {
            Self::General => "icons/settings.svg",
            Self::Appearance => "icons/palette.svg",
            Self::Lyrics => "icons/mic-vocal.svg",
            Self::Playback => "icons/play.svg",
            Self::Integrations => "icons/link.svg",
            Self::Keybinds => "icons/keyboard.svg",
        }
    }

    /// The pages the tab shows, top to bottom. The first is the one the tab opens on.
    pub fn sections(self) -> &'static [SettingsPage] {
        match self {
            Self::General => &[
                SettingsPage::Providers,
                SettingsPage::General,
                SettingsPage::About,
            ],
            Self::Appearance => &[
                SettingsPage::Appearance,
                SettingsPage::PlayerBar,
                SettingsPage::NowPlaying,
            ],
            Self::Lyrics => &[SettingsPage::Lyrics],
            Self::Playback => &[
                SettingsPage::Playback,
                SettingsPage::Sound,
                SettingsPage::Storage,
            ],
            Self::Integrations => &[SettingsPage::Discord, SettingsPage::Scrobbling],
            Self::Keybinds => &[SettingsPage::Keybinds],
        }
    }

    /// The page the tab opens on, at its top.
    pub fn lead(self) -> SettingsPage {
        self.sections()[0]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Destination {
    Home,
    History,
    Library(LibraryTab),
    Local(LibraryTab),
    Album(SharedString),
    Song(SharedString),
    Playlist(SharedString),
    Artist(SharedString),
    User(SharedString),
    Genre(SharedString),
    Search,
    Settings(SettingsPage),
    /// The equalizer's own settings page, opened from Sound.
    Equalizer,
    Fullscreen,
}

impl From<&ui::Pin> for Destination {
    fn from(pin: &ui::Pin) -> Self {
        let id = SharedString::from(pin.id.clone());
        match pin.kind {
            ui::PinKind::Album => Destination::Album(id),
            ui::PinKind::Artist => Destination::Artist(id),
            ui::PinKind::Playlist => Destination::Playlist(id),
            ui::PinKind::Song => Destination::Song(id),
            ui::PinKind::Liked => Destination::Library(LibraryTab::Songs),
        }
    }
}

impl Destination {
    /// Whether the sidebar row leading to `self` is lit while `other` is open. Every settings page
    /// lights the one Settings row, and every other row is lit only on its own page.
    pub fn same_section(&self, other: &Destination) -> bool {
        (self.in_settings() && other.in_settings()) || self == other
    }

    /// Whether this destination is a settings page, the equalizer included.
    pub fn in_settings(&self) -> bool {
        matches!(self, Destination::Settings(_) | Destination::Equalizer)
    }

    /// The settings page a drill-in's back button returns to. Only drill-ins have one.
    pub fn parent(&self) -> Option<Destination> {
        match self {
            Destination::Equalizer => Some(Destination::Settings(SettingsPage::Sound)),
            Destination::Settings(page) => page.parent().map(Destination::Settings),
            _ => None,
        }
    }

    /// The tabbed page this destination belongs to: the page itself, or a drill-in's parent.
    pub fn settings_page(&self) -> Option<SettingsPage> {
        match self {
            Destination::Equalizer => Some(SettingsPage::Sound),
            Destination::Settings(page) => Some(page.parent().unwrap_or(*page)),
            _ => None,
        }
    }

    /// Whether this page shows provider data that Refresh can load again. Settings, search and
    /// the fullscreen player have nothing to reload, so they get no refresh button.
    pub fn refreshes(&self) -> bool {
        !matches!(
            self,
            Destination::Search
                | Destination::Settings(_)
                | Destination::Equalizer
                | Destination::Fullscreen
        )
    }
}

#[derive(Clone)]
struct Router(Entity<Navigation>);

impl Global for Router {}

pub fn init(start: Destination, cx: &mut App) {
    let navigation = cx.new(|_| Navigation::new(start));
    cx.set_global(Router(navigation));
}

pub fn trail(cx: &App) -> Entity<Navigation> {
    cx.global::<Router>().0.clone()
}

pub fn navigate(destination: Destination, cx: &mut App) {
    trail(cx).update(cx, |navigation, cx| navigation.go(destination, cx));
}

/// Replaces the current destination so Back and Forward do not revisit a page that disappeared.
pub fn replace(destination: Destination, cx: &mut App) {
    trail(cx).update(cx, |navigation, cx| navigation.replace(destination, cx));
}

pub fn back(cx: &mut App) {
    trail(cx).update(cx, |navigation, cx| navigation.back(cx));
}

pub fn forward(cx: &mut App) {
    trail(cx).update(cx, |navigation, cx| navigation.forward(cx));
}

/// Steps up from a drill-in to `parent`, going back when it is the page before so the history
/// holds no loop, and replacing the drill-in otherwise.
pub fn up(parent: Destination, cx: &mut App) {
    trail(cx).update(cx, |navigation, cx| navigation.up(parent, cx));
}

/// Opens settings on the page they last showed, or on General the first time.
pub fn open_settings(cx: &mut App) {
    trail(cx).update(cx, |navigation, cx| navigation.open_settings(cx));
}
