use adw::subclass::prelude::*;
use gtk::{gdk, glib, prelude::*};

mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/Tour/ui/paginator.ui")]
    pub struct PaginatorWidget {
        #[template_child]
        pub(super) carousel: TemplateChild<adw::Carousel>,
        pub(super) pages: RefCell<Vec<gtk::Widget>>,
        pub(super) current_page: Cell<u32>,
        #[template_child]
        pub(super) next_btn: TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) start_btn: TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) previous_btn: TemplateChild<gtk::Button>,
        pub(super) going_backward: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PaginatorWidget {
        const NAME: &'static str = "PaginatorWidget";
        type ParentType = adw::Bin;
        type Type = super::PaginatorWidget;
        type Interfaces = (gtk::Buildable,);

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PaginatorWidget {
        fn constructed(&self) {
            self.parent_constructed();
            self.carousel
                .set_scroll_params(&adw::SpringParams::new(1.0, 0.5, 300.0));
        }
    }
    impl WidgetImpl for PaginatorWidget {}
    impl BinImpl for PaginatorWidget {}
    impl BuildableImpl for PaginatorWidget {
        fn add_child(&self, builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
            if !self.carousel.is_bound() {
                self.parent_add_child(builder, child, type_);
            } else {
                self.obj()
                    .add_page(child.clone().downcast::<gtk::Widget>().unwrap());
            }
        }
    }
}

glib::wrapper! {
    pub struct PaginatorWidget(ObjectSubclass<imp::PaginatorWidget>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl PaginatorWidget {
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

    fn add_page(&self, page: impl IsA<gtk::Widget>) {
        let imp = self.imp();
        let page_nr = imp.pages.borrow().len();
        imp.carousel.insert(&page, page_nr as i32);
        imp.pages.borrow_mut().push(page.upcast());

        self.on_position_notify();
    }

    #[template_callback]
    fn on_key_pressed(&self, keyval: gdk::Key) -> glib::Propagation {
        if keyval == gdk::Key::Right {
            self.try_next();
        } else if keyval == gdk::Key::Left {
            self.try_previous();
        }
        glib::Propagation::Proceed
    }

    #[template_callback]
    fn on_position_notify(&self) {
        let imp = self.imp();

        let position = imp.carousel.position();
        let page_nr = position.round() as u32;

        let n_pages = imp.carousel.n_pages() as f64;
        let forelast_page = n_pages - 2.0;
        let last_page = n_pages - 1.0;

        let (opacity_previous, opacity_start, opacity_next) = if (0.0..1.0).contains(&position) {
            (position, 1.0 - position, position)
        } else if position <= forelast_page {
            (1.0, 0.0, 1.0)
        } else if position > forelast_page {
            (1.0, 0.0, last_page - position)
        } else {
            panic!("Position of the carousel is outside the allowed range");
        };

        // While transitioning to the last page the next button is still visible
        // pressing it would crash the app so we make it not targetable.
        let can_target_start = opacity_next < f64::EPSILON;
        let can_target_next = opacity_next > 0_f64 && position <= forelast_page;

        imp.start_btn.set_opacity(opacity_start);
        imp.start_btn.set_visible(opacity_start > 0_f64);
        imp.start_btn.set_can_target(can_target_start);

        imp.next_btn.set_opacity(opacity_next);
        imp.next_btn.set_visible(opacity_next > 0_f64);
        imp.next_btn.set_can_target(can_target_next);

        imp.previous_btn.set_opacity(opacity_previous);
        imp.previous_btn.set_visible(opacity_previous > 0_f64);

        imp.current_page.set(page_nr);
    }

    pub fn set_page(&self, page_nr: u32) {
        let imp = self.imp();
        let total_pages = imp.carousel.n_pages();

        if page_nr == total_pages - 1 {
            imp.going_backward.set(true);
        } else if page_nr == 0 {
            imp.going_backward.set(false);
        }

        if !imp.going_backward.get() {
            if page_nr == 0 {
                imp.start_btn.grab_focus();
            } else {
                imp.next_btn.grab_focus();
            }
        } else {
            imp.previous_btn.grab_focus();
        }

        if page_nr < imp.carousel.n_pages() {
            let pages = &imp.pages.borrow();
            let page = pages.get(page_nr as usize).unwrap();
            imp.carousel.scroll_to(page, true);
        }
    }
}
