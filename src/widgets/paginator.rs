use gettextrs::gettext;
use gtk::glib::{self, clone};
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PaginatorWidget {
    pub widget: gtk::Box,
    carousel: adw::Carousel,
    carousel_dots: adw::CarouselIndicatorDots,
    headerbar: gtk::HeaderBar,
    pages: RefCell<Vec<gtk::Widget>>,
    current_page: RefCell<u32>,
    next_overlay: gtk::Overlay,
    next_btn: gtk::Button,
    start_btn: gtk::Button,
    finish_btn: gtk::Button,
    close_btn: gtk::Button,
    previous_btn: gtk::Button,
}

impl PaginatorWidget {
    pub fn new() -> Rc<Self> {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let paginator = Rc::new(Self {
            widget,
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
            current_page: RefCell::new(0),
        });
        paginator.init(paginator.clone());
        paginator
    }

    pub fn try_next(&self) -> Option<()> {
        let p = *self.current_page.borrow() + 1;
        if p == self.carousel.n_pages() {
            return None;
        }
        self.set_page(p);
        Some(())
    }

    pub fn try_previous(&self) -> Option<()> {
        let p = *self.current_page.borrow();
        if p == 0 {
            return None;
        }
        self.set_page(p - 1);
        Some(())
    }

    pub fn add_page(&self, page: gtk::Widget) {
        let page_nr = self.pages.borrow().len();
        self.carousel.insert(&page, page_nr as i32);
        self.pages.borrow_mut().push(page);

        self.update_position();
    }

    fn update_position(&self) {
        let position = self.carousel.position();
        let page_nr = position.round() as u32;

        let n_pages = self.carousel.n_pages() as f64;
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

        self.start_btn.set_opacity(opacity_start);
        self.start_btn.set_visible(opacity_start > 0_f64);

        self.next_btn.set_opacity(opacity_next);
        self.next_btn.set_visible(opacity_next > 0_f64);
        self.next_overlay.set_can_target(opacity_next > 0_f64);

        self.finish_btn.set_opacity(opacity_finish);
        self.finish_btn.set_visible(opacity_finish > 0_f64);

        self.previous_btn.set_opacity(opacity_previous);
        self.previous_btn.set_visible(opacity_previous > 0_f64);

        self.close_btn.set_opacity(opacity_close);
        self.start_btn.set_visible(opacity_close > 0_f64);

        self.current_page.replace(page_nr);
    }

    fn init(&self, p: Rc<Self>) {
        self.carousel_dots.set_carousel(Some(&self.carousel));
        self.carousel.set_hexpand(true);
        self.carousel.set_vexpand(true);
        self.carousel
            .set_scroll_params(&adw::SpringParams::new(1.0, 0.5, 300.0));

        self.carousel
            .connect_position_notify(clone!(@weak p => move |_| {
                p.update_position();
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

        self.widget.append(&self.headerbar);
        self.widget.append(&self.carousel);
    }

    pub fn set_page(&self, page_nr: u32) {
        if page_nr < self.carousel.n_pages() {
            let pages = &self.pages.borrow();
            let page = pages.get(page_nr as usize).unwrap();
            self.carousel.scroll_to(page, true);
        }
    }
}
