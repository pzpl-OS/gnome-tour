use gtk::prelude::*;

pub struct ImagePageWidget {
    pub widget: gtk::Box,
}

impl ImagePageWidget {
    pub fn new(resource_uri: &str, head: String, body: String) -> Self {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let image_page = Self { widget };

        image_page.init(resource_uri, head, body);
        image_page
    }

    fn init(&self, resource_uri: &str, head: String, body: String) {
        self.widget.set_property_expand(true);
        self.widget.set_halign(gtk::Align::Fill);
        self.widget.set_valign(gtk::Align::Fill);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 12);
        container.set_halign(gtk::Align::Center);
        container.set_valign(gtk::Align::Center);
        container.set_vexpand(true);
        container.set_margin_bottom(48);
        container.set_margin_top(12);
        container.set_margin_start(64);
        container.set_margin_end(64);

        let image = gtk::Image::from_resource(&resource_uri);
        image.set_valign(gtk::Align::Start);
        image.show();
        container.add(&image);

        let head_label = gtk::Label::new(Some(&head));
        head_label.set_justify(gtk::Justification::Center);
        head_label.set_valign(gtk::Align::Center);
        head_label.set_property_wrap(true);
        head_label.set_margin_top(36);
        head_label.get_style_context().add_class("page-title");
        head_label.show();
        container.add(&head_label);

        let body_label = gtk::Label::new(Some(&body));
        body_label.set_lines(2);
        body_label.set_property_wrap(true);
        body_label.set_justify(gtk::Justification::Center);
        body_label.set_valign(gtk::Align::Center);
        body_label.set_margin_top(12);
        body_label.get_style_context().add_class("page-body");
        body_label.show();
        container.add(&body_label);

        container.show();
        self.widget.add(&container);
        self.widget.show();
    }
}
