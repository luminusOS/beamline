use gtk::{gio, glib, subclass::prelude::*};

mod imp {
    use super::*;
    use adw::subclass::prelude::*;
    use gtk::CompositeTemplate;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/org/luminusos/Beamline/window.ui")]
    pub struct BeamlineWindow {
        #[template_child]
        pub status_page: TemplateChild<adw::StatusPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BeamlineWindow {
        const NAME: &'static str = "BeamlineWindow";
        type Type = super::BeamlineWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BeamlineWindow {}
    impl WidgetImpl for BeamlineWindow {}
    impl WindowImpl for BeamlineWindow {}
    impl ApplicationWindowImpl for BeamlineWindow {}
    impl AdwApplicationWindowImpl for BeamlineWindow {}
}

glib::wrapper! {
    pub struct BeamlineWindow(ObjectSubclass<imp::BeamlineWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl BeamlineWindow {
    pub fn new(app: Option<&adw::Application>) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn status_text(&self) -> (glib::GString, glib::GString) {
        let status = &self.imp().status_page;
        (
            status.title(),
            status
                .description()
                .expect("Status description is required"),
        )
    }
}
