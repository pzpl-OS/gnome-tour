use gtk::prelude::*;
use std::process::Command;

pub struct ImageButtonPageWidget {
    pub widget: gtk::Box,
}

impl ImageButtonPageWidget {
    pub fn new(resource_uri: &str, head: String, body: String, buttons: Vec<(String, &'static str, &'static str)>) -> Self {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let image_button_page = Self { widget };

        image_button_page.init(resource_uri, head, body, buttons);
        image_button_page
    }

    fn init(&self, resource_uri: &str, head: String, body: String, buttons: Vec<(String, &'static str, &'static str)>) {
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

        let actions_container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        actions_container.set_halign(gtk::Align::Center);
        actions_container.set_margin_top(12);

        /*
                I don't know why, I don't want to know why,
                I shouldn't have to wonder why, but for whatever reason
                this stupid panel isn't laying out correctly unless
                we do this terribleness
                 todo: this is dumb
        */

        let second_actions_container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        second_actions_container.set_halign(gtk::Align::Center);
        second_actions_container.set_margin_top(12);

        let mut suggested_button = true;
        let mut buttonsc = 0;
        for button_data in buttons {
                buttonsc += 1;
                let button = gtk::Button::with_label(&button_data.0);
                button.set_property_height_request(40);
                button.set_property_width_request(180);
                button.connect_clicked(move |_| {
                        let mut command = &mut Command::new(&button_data.1);
                        if button_data.2 != "" {
                                command = command.arg(button_data.2);
                        }
                        let result = command.spawn();
                        if result.is_err() {
                                println!("{}", "Failed to run button command")
                        }
                });
                if suggested_button {
                        button.get_style_context().add_class("suggested-action");
                        suggested_button = false;
                }
                button.show();
                if buttonsc > 3 {
                    second_actions_container.add(&button);
                } else {
                    actions_container.add(&button);
                }
        }

        actions_container.show();
        container.add(&actions_container);


        second_actions_container.show();
        container.add(&second_actions_container);

        container.show();
        self.widget.add(&container);
        self.widget.show();
    }
}
