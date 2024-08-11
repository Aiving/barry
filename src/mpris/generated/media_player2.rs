//! # D-Bus interface proxy for: `org.mpris.MediaPlayer2`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `interface.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(interface = "org.mpris.MediaPlayer2", assume_defaults = true)]
trait MediaPlayer2 {
    /// Quit method
    fn quit(&self) -> zbus::Result<()>;

    /// Raise method
    fn raise(&self) -> zbus::Result<()>;

    /// CanQuit property
    #[zbus(property)]
    fn can_quit(&self) -> zbus::Result<bool>;

    /// CanRaise property
    #[zbus(property)]
    fn can_raise(&self) -> zbus::Result<bool>;

    /// DesktopEntry property
    #[zbus(property)]
    fn desktop_entry(&self) -> zbus::Result<String>;

    /// HasTrackList property
    #[zbus(property)]
    fn has_track_list(&self) -> zbus::Result<bool>;

    /// Identity property
    #[zbus(property)]
    fn identity(&self) -> zbus::Result<String>;

    /// SupportedMimeTypes property
    #[zbus(property)]
    fn supported_mime_types(&self) -> zbus::Result<Vec<String>>;

    /// SupportedUriSchemes property
    #[zbus(property)]
    fn supported_uri_schemes(&self) -> zbus::Result<Vec<String>>;
}
