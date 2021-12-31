#[cfg(feature = "video")]
use crate::config;
use crate::utils::i18n_f;
use gettextrs::gettext;
#[cfg(feature = "video")]
use gio::FileExt;
use gtk::glib;
#[cfg(feature = "video")]
use gtk::glib::clone;
#[cfg(feature = "video")]
use gtk::glib::{Receiver, Sender};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
#[cfg(feature = "video")]
use std::cell::RefCell;

#[derive(PartialEq)]
#[cfg(feature = "video")]
pub enum Action {
    VideoReady,
    VideoUp,
}

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct WelcomePageWidget {
        #[cfg(feature = "video")]
        player: gst_player::Player,
        #[cfg(feature = "video")]
        receiver: RefCell<Option<Receiver<Action>>>,
        #[cfg(feature = "video")]
        sender: Sender<Action>,
    }

    impl Default for WelcomePageWidget {
        fn default() -> Self {
            #[cfg(feature = "video")]
            let player = {
                let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
                let sink = gst::ElementFactory::make("gtksink", None)
                .expect("Missing dependency: element gtksink is needed (usually, in gstreamer-plugins-good or in gst-plugin-gtk).");
                let renderer =
                    gst_player::PlayerVideoOverlayVideoRenderer::with_sink(&sink).upcast();
                gst_player::Player::new(
                    Some(&renderer),
                    Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
                )
            };
            #[cfg(feature = "video")]
            let (sender, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            #[cfg(feature = "video")]
            let receiver = RefCell::new(Some(r));

            Self {
                #[cfg(feature = "video")]
                player,
                #[cfg(feature = "video")]
                sender,
                #[cfg(feature = "video")]
                receiver,
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WelcomePageWidget {
        const NAME: &'static str = "WelcomePageWidget";
        type ParentType = gtk::Box;
        type Type = super::WelcomePageWidget;
    }

    impl ObjectImpl for WelcomePageWidget {
        fn constructed(&self, widget: &Self::Type) {
            let layout_manager = widget
                .layout_manager()
                .map(|l| l.downcast::<gtk::BoxLayout>().unwrap())
                .unwrap();
            layout_manager.set_orientation(gtk::Orientation::Vertical);

            let container = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(0)
                .hexpand(true)
                .vexpand(true)
                .valign(gtk::Align::Center)
                .halign(gtk::Align::Center)
                .margin_top(24)
                .margin_bottom(24)
                .build();
            widget.add_css_class("page");
            widget.add_css_class("welcome-page");

            let clamp = adw::Clamp::new();
            clamp.set_child(Some(&container));

            #[cfg(not(feature = "video"))]
            let header = {
                let logo = gtk::Picture::builder()
                    .can_shrink(false)
                    .keep_aspect_ratio(true)
                    .build();
                logo.set_resource(Some("/org/gnome/Tour/welcome.svg"));

                logo.upcast::<gtk::Widget>()
            };

            #[cfg(feature = "video")]
            let header = {
                let video_widget = self
                    .player
                    .pipeline()
                    .property::<gst::Element>("video-sink")
                    .property::<gtk::Widget>("widget");

                video_widget.set_size_request(-1, 360);
                video_widget.set_property("ignore-alpha", &false).unwrap();
                video_widget.add_css_class("video");
                video_widget
            };

            container.append(&header);

            #[cfg(feature = "video")]
            {
                let receiver = self.receiver.borrow_mut().take().unwrap();
                receiver.attach(
                    None,
                    clone!(@strong self.player as player => move |action| {
                        match action {
                            Action::VideoReady => player.play(),
                            Action::VideoUp => header.add_css_class("playing"),
                        };
                        glib::Continue(true)
                    }),
                );

                self.player.connect_state_changed(
                    clone!(@strong self.sender as sender => move |_p,state| {
                        if state == gst_player::PlayerState::Playing {
                            sender.send(Action::VideoUp).unwrap();
                        }
                    }),
                );

                self.player.connect_uri_loaded(
                    clone!(@strong self.sender as sender => move |_p, _uri| {
                        sender.send(Action::VideoReady).unwrap();
                    }),
                );
                self.player.connect_end_of_stream(move |p| p.stop());

                let video_file = gio::File::new_for_path(config::VIDEO_PATH);
                gtk::timeout_add(
                    500,
                    clone!(@strong self.player as player => move || {
                        player.set_uri(&video_file.get_uri());
                        glib::Continue(false)
                    }),
                );
            };

            let title = gtk::Label::new(Some(&gettext("Start the Tour")));
            title.set_margin_top(36);
            title.add_css_class("title-1");
            container.append(&title);

            let name = glib::os_info("NAME").unwrap_or_else(|| "GNOME".into());
            let version = glib::os_info("VERSION").unwrap_or_else(|| "".into());
            // Translators: The following string is formated as "Learn about new and essential features in GNOME 3.36" for example
            let text = gtk::Label::new(Some(&i18n_f(
                "Learn about the key features in {} {}.",
                &[&name, &version],
            )));
            text.add_css_class("body");
            text.set_margin_top(12);
            container.append(&text);

            widget.append(&clamp);
            self.parent_constructed(widget);
        }
    }
    impl WidgetImpl for WelcomePageWidget {}
    impl BoxImpl for WelcomePageWidget {}
}

glib::wrapper! {
    pub struct WelcomePageWidget(ObjectSubclass<imp::WelcomePageWidget>)
        @extends gtk::Widget, gtk::Box;
}

impl WelcomePageWidget {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}
