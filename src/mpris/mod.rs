use generated::{
    media_player2::MediaPlayer2Proxy,
    player::{Metadata, PlaybackStatus, PlayerProxy},
};
use tokio_stream::{Stream, StreamExt};
use zbus::{
    fdo::NameOwnerChanged,
    proxy::{CacheProperties, PropertyStream},
    Connection, MatchRule, MessageStream, Proxy,
};

pub mod generated;

#[derive(Debug)]
pub struct Players<'a> {
    proxy: zbus::fdo::DBusProxy<'a>,
}

impl<'a> Players<'a> {
    pub async fn new() -> Self {
        let connection = Connection::session().await.unwrap();

        let proxy = zbus::fdo::DBusProxy::builder(&connection)
            .cache_properties(CacheProperties::No)
            .build()
            .await
            .unwrap();

        Self { proxy }
    }

    pub async fn owner_changed_steam(&self) -> Option<impl Stream<Item = NameOwnerChanged> + '_> {
        self.proxy
            .receive_name_owner_changed()
            .await
            .ok()
            .map(|names| {
                names.filter(|name| {
                    name.args()
                        .is_ok_and(|name| name.name.starts_with("org.mpris.MediaPlayer2."))
                })
            })
    }

    pub async fn list_names(&self) -> Vec<String> {
        self.proxy
            .list_names()
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|name| name.as_str().into())
            .collect()
    }
}

#[derive(Debug)]
pub struct Player<'a> {
    proxy: Proxy<'a>,
    player_proxy: PlayerProxy<'a>,
}

impl<'a> Player<'a> {
    pub async fn all() -> Vec<Self> {
        let connection = Connection::session().await.unwrap();

        let message = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/",
                Some("org.freedesktop.DBus"),
                "ListNames",
                &(),
            )
            .await
            .unwrap();

        let names: Vec<String> = message.body().deserialize().unwrap();
        let mut names = names
            .into_iter()
            .filter(|name| name.starts_with("org.mpris.MediaPlayer2."))
            .collect::<Vec<_>>();

        names.sort_by_key(|name| name.to_lowercase());

        let mut players = vec![];

        for name in names {
            let proxy = zbus::Proxy::new(
                &connection,
                name.clone(),
                "/org/mpris/MediaPlayer2",
                "org.mpris.MediaPlayer",
            )
            .await;

            let media_proxy = MediaPlayer2Proxy::builder(&connection)
                .destination(name.clone())
                .and_then(|builder| builder.path("/org/mpris/MediaPlayer2"))
                .map(|builder| builder.cache_properties(CacheProperties::No).build());

            let player_proxy = PlayerProxy::builder(&connection)
                .destination(name)
                .and_then(|builder| builder.path("/org/mpris/MediaPlayer2"))
                .map(|builder| {
                    builder
                        .cache_properties(CacheProperties::Yes)
                        .uncached_properties(&["Position"])
                        .build()
                });

            if let (Ok(proxy), Ok(media_proxy), Ok(player_proxy)) =
                (proxy, media_proxy, player_proxy)
            {
                if let (Ok(_), Ok(player_proxy)) = (media_proxy.await, player_proxy.await) {
                    players.push(Self {
                        proxy,
                        // media_proxy,
                        player_proxy,
                    });
                }
            }
        }

        players
    }

    pub async fn find_active() -> Option<Self> {
        let players = Self::all().await;

        if players.is_empty() {
            return None;
        }

        let mut first_paused: Option<Player> = None;
        let mut first_with_track: Option<Player> = None;
        let mut first_found: Option<Player> = None;

        for player in players {
            let player_status = player.get_playback_status().await?;

            if player_status == PlaybackStatus::Playing {
                return Some(player);
            }

            if first_paused.is_none() && player_status == PlaybackStatus::Paused {
                first_paused.replace(player);
            } else if first_with_track.is_none() && !player.get_metadata().await?.is_empty() {
                first_with_track.replace(player);
            } else if first_found.is_none() {
                first_found.replace(player);
            }
        }

        first_paused.or(first_with_track).or(first_found)
    }

    pub async fn get_metadata(&self) -> Option<Metadata> {
        self.player_proxy.metadata().await.ok()
    }

    pub async fn get_playback_status(&self) -> Option<PlaybackStatus> {
        self.player_proxy.playback_status().await.ok()
    }

    pub async fn get_playback_status_stream(&self) -> PropertyStream<PlaybackStatus> {
        self.player_proxy.receive_playback_status_changed().await
    }

    pub async fn get_stream(&self) -> PropertyStream<bool> {
        self.player_proxy.receive_can_control_changed().await
    }

    pub async fn metadata_stream(&self) -> PropertyStream<Metadata> {
        self.player_proxy.receive_metadata_changed().await
    }

    pub async fn get_position(&self) -> Option<i64> {
        self.player_proxy.position().await.ok()
    }

    pub async fn props_changed_steam(&self) -> MessageStream {
        MessageStream::for_match_rule(
            MatchRule::builder()
                .msg_type(zbus::message::Type::Signal)
                .interface("org.freedesktop.DBus.Properties")
                .and_then(|x| x.member("PropertiesChanged"))
                .and_then(|x| x.path("/org/mpris/MediaPlayer2"))
                .unwrap()
                .build(),
            self.proxy.connection(),
            None,
        )
        .await
        .unwrap()
    }
}
