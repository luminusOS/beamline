mod window;

use adw::prelude::*;
use gtk::{gio, glib};

const APP_ID: &str = "org.luminusos.Beamline";
const GETTEXT_DOMAIN: &str = "beamline";

fn main() -> glib::ExitCode {
    gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "");
    let locale_dir =
        std::env::var_os("BEAMLINE_LOCALEDIR").unwrap_or_else(|| env!("BEAMLINE_LOCALEDIR").into());
    if let Err(error) = gettextrs::bindtextdomain(GETTEXT_DOMAIN, locale_dir) {
        eprintln!("Failed to bind translation domain: {error}");
        return glib::ExitCode::FAILURE;
    }
    if let Err(error) = gettextrs::bind_textdomain_codeset(GETTEXT_DOMAIN, "UTF-8") {
        eprintln!("Failed to set translation encoding: {error}");
        return glib::ExitCode::FAILURE;
    }
    if let Err(error) = gettextrs::textdomain(GETTEXT_DOMAIN) {
        eprintln!("Failed to set translation domain: {error}");
        return glib::ExitCode::FAILURE;
    }

    let resource_bytes = glib::Bytes::from_static(include_bytes!(env!("BEAMLINE_GRESOURCE")));
    let resource = match gio::Resource::from_data(&resource_bytes) {
        Ok(resource) => resource,
        Err(error) => {
            eprintln!("Failed to load application resources: {error}");
            return glib::ExitCode::FAILURE;
        }
    };
    gio::resources_register(&resource);

    if std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--check-ui")) {
        if let Err(error) = gtk::init() {
            eprintln!("Failed to initialize GTK: {error}");
            return glib::ExitCode::FAILURE;
        }
        adw::init().expect("Failed to initialize libadwaita");
        let window = window::BeamlineWindow::new(None);
        let window_title = window.title().expect("Window title is required");
        let (title, description) = window.status_text();
        assert_eq!(window_title, gettextrs::gettext("Cast Screen"));
        assert_eq!(title, gettextrs::gettext("Screen Casting Unavailable"));
        assert_eq!(
            description,
            gettextrs::gettext(
                "Device discovery and screen casting are not available in this development build."
            )
        );
        println!("{window_title}\n{title}\n{description}");
        return glib::ExitCode::SUCCESS;
    }

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        if let Some(window) = app.active_window() {
            window.present();
        } else {
            window::BeamlineWindow::new(Some(app)).present();
        }
    });
    app.run()
}
