use gettextrs::gettext;
use gtk::prelude::*;
use gtk::{
    glib::{self, clone},
    subclass::prelude::*,
};
use std::cell::RefCell;

mod imp {
    use std::cell::Cell;

    use super::*;

    #[derive(Debug)]
    pub struct PaginatorWidget {
        pub(super) carousel: adw::Carousel,
        pub(super) carousel_dots: adw::CarouselIndicatorDots,
        pub(super) headerbar: gtk::HeaderBar,
        pub(super) pages: RefCell<Vec<gtk::Widget>>,
        pub(super) current_page: Cell<u32>,
        pub(super) next_overlay: gtk::Overlay,
        pub(super) next_btn: gtk::Button,
        pub(super) start_btn: gtk::Button,
        pub(super) finish_btn: gtk::Button,
        pub(super) close_btn: gtk::Button,
        pub(super) previous_btn: gtk::Button,
    }

    impl Default for PaginatorWidget {
        fn default() -> Self {
            Self {
                carousel: adw::Carousel::new(),
                carousel_dots: adw::CarouselIndicatorDots::new(),
                headerbar: gtk::HeaderBar::builder().show_title_buttons(false).build(),
                start_btn: gtk::Button::with_label(&gettext("_Start")),
                next_overlay: gtk::Overlay::new(),
                next_btn: gtk::Button::with_label(&gettext("_Next")),
                finish_btn: gtk::Button::with_label(&gettext("_Close")),
                close_btn: gtk::Button::with_label(&gettext("_Close")),
                previous_btn: gtk::Button::with_label(&gettext("_Previous")),
                pages: RefCell::new(Vec::new()),
                current_page: Cell::new(0),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PaginatorWidget {
        const NAME: &'static str = "PaginatorWidget";
        type ParentType = gtk::Box;
        type Type = super::PaginatorWidget;
    }

    impl ObjectImpl for PaginatorWidget {
        fn constructed(&self, obj: &Self::Type) {
            let layout_manager = obj
                .layout_manager()
                .map(|l| l.downcast::<gtk::BoxLayout>().unwrap())
                .unwrap();
            layout_manager.set_orientation(gtk::Orientation::Vertical);

            self.carousel_dots.set_carousel(Some(&self.carousel));
            self.carousel.set_hexpand(true);
            self.carousel.set_vexpand(true);
            self.carousel
                .set_scroll_params(&adw::SpringParams::new(1.0, 0.5, 300.0));

            self.carousel
                .connect_position_notify(clone!(@weak obj => move |_| {
                    obj.update_position();
                }));
            self.start_btn.add_css_class("suggested-action");
            self.start_btn.set_use_underline(true);
            self.start_btn.set_action_name(Some("app.start-tour"));

            self.next_btn.add_css_class("suggested-action");
            self.next_btn.set_use_underline(true);
            self.next_btn.set_action_name(Some("app.next-page"));

            self.close_btn.set_use_underline(true);
            self.close_btn.set_action_name(Some("app.quit"));

            self.finish_btn.add_css_class("suggested-action");
            self.finish_btn.set_use_underline(true);
            self.finish_btn.set_action_name(Some("app.quit"));

            self.previous_btn.set_use_underline(true);
            self.previous_btn.set_action_name(Some("app.previous-page"));

            self.next_overlay.set_child(Some(&self.next_btn));
            self.next_overlay.add_overlay(&self.finish_btn);
            self.next_overlay.set_can_target(false);

            let previous_overlay = gtk::Overlay::new();
            previous_overlay.set_child(Some(&self.close_btn));
            previous_overlay.add_overlay(&self.previous_btn);

            let start_overlay = gtk::Overlay::new();
            start_overlay.set_child(Some(&self.start_btn));
            start_overlay.add_overlay(&self.next_overlay);

            let btn_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
            btn_size_group.add_widget(&self.previous_btn);
            btn_size_group.add_widget(&self.close_btn);
            btn_size_group.add_widget(&self.next_overlay);
            btn_size_group.add_widget(&start_overlay);
            btn_size_group.add_widget(&self.finish_btn);

            self.headerbar.set_title_widget(Some(&self.carousel_dots));
            self.headerbar.pack_start(&previous_overlay);
            self.headerbar.pack_end(&start_overlay);

            obj.append(&self.headerbar);
            obj.append(&self.carousel);

            self.parent_constructed(obj);
        }
    }
    impl WidgetImpl for PaginatorWidget {}
    impl BoxImpl for PaginatorWidget {}
}

glib::wrapper! {
    pub struct PaginatorWidget(ObjectSubclass<imp::PaginatorWidget>)
        @extends gtk::Widget, gtk::Box;

}

impl PaginatorWidget {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    pub fn try_next(&self) -> Option<()> {
        let imp = self.imp();
        let p = imp.current_page.get() + 1;
        if p == imp.carousel.n_pages() {
            return None;
        }
        self.set_page(p);
        Some(())
    }

    pub fn try_previous(&self) -> Option<()> {
        let p = self.imp().current_page.get();
        if p == 0 {
            return None;
        }
        self.set_page(p - 1);
        Some(())
    }

    pub fn add_page(&self, page: impl IsA<gtk::Widget>) {
        let imp = self.imp();
        let page_nr = imp.pages.borrow().len();
        imp.carousel.insert(&page, page_nr as i32);
        imp.pages.borrow_mut().push(page.upcast());

        self.update_position();
    }

    fn update_position(&self) {
        let imp = self.imp();

        let position = imp.carousel.position();
        let page_nr = position.round() as u32;

        let n_pages = imp.carousel.n_pages() as f64;
        let forelast_page = n_pages - 2.0;
        let last_page = n_pages - 1.0;

        let (opacity_finish, opacity_previous, opacity_start, opacity_next, opacity_close) =
            if (0.0..1.0).contains(&position) {
                if position == 0.0 {
                    (0.0, position, 1.0, position, 1.0)
                } else {
                    (0.0, position, 1.0, position, 1f64 - position)
                }
            } else if (0.0 <= position) && (position <= forelast_page) {
                (0.0, 1.0, 1f64 - position, 1.0, 0.0)
            } else if (forelast_page < position) && (position <= last_page) {
                (position - forelast_page, 1.0, 0.0, 1.0, 0.0)
            } else {
                panic!("Position of the carousel is outside the allowed range");
            };

        imp.start_btn.set_opacity(opacity_start);
        imp.start_btn.set_visible(opacity_start > 0_f64);

        imp.next_btn.set_opacity(opacity_next);
        imp.next_btn.set_visible(opacity_next > 0_f64);
        imp.next_overlay.set_can_target(opacity_next > 0_f64);

        imp.finish_btn.set_opacity(opacity_finish);
        imp.finish_btn.set_visible(opacity_finish > 0_f64);

        imp.previous_btn.set_opacity(opacity_previous);
        imp.previous_btn.set_visible(opacity_previous > 0_f64);

        imp.close_btn.set_opacity(opacity_close);
        imp.start_btn.set_visible(opacity_close > 0_f64);

        imp.current_page.set(page_nr);
    }

    pub fn set_page(&self, page_nr: u32) {
        let imp = self.imp();
        if page_nr < imp.carousel.n_pages() {
            let pages = &imp.pages.borrow();
            let page = pages.get(page_nr as usize).unwrap();
            imp.carousel.scroll_to(page, true);
        }
    }
}
