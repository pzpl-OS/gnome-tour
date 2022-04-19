use gettextrs::gettext;
use gtk::prelude::*;
use std::process::{Command, Stdio};

pub struct OfficeSuitesPageWidget {
    pub widget: gtk::Box,
}

impl OfficeSuitesPageWidget {
    pub fn new(resource_uri: &str, head: String, body: String) -> Self {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let office_suites_page = Self { widget };

        office_suites_page.init(resource_uri, head, body);
        office_suites_page
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

        let actions_container = gtk::Box::new(gtk::Orientation::Horizontal, 128);
        actions_container.set_halign(gtk::Align::Center);
        actions_container.set_margin_top(24);

        let office_suite_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);

        // LibreOffice
        let libreoffice_container = gtk::Box::new(gtk::Orientation::Vertical, 12);
        libreoffice_container.set_halign(gtk::Align::Center);

        let libreoffice_title = gtk::Label::new(Some("LibreOffice"));
        libreoffice_title.set_justify(gtk::Justification::Center);
        libreoffice_title.set_valign(gtk::Align::Center);
        libreoffice_title.get_style_context().add_class("app-title");
        libreoffice_title.show();
        libreoffice_container.add(&libreoffice_title);

        let check_libreoffice_apt = Command::new("/bin/bash")
                        .arg("-c")
                        .arg("command -v libreoffice")
                        .stdout(Stdio::null())
                        .status();
        let check_libreoffice_apt_installed = match check_libreoffice_apt {
                Ok(installed) => installed.success(),
                Err(error) => {eprintln!("Failed to check if the LibreOffice APT package is installed: {}", error); false},
        };

        let check_libreoffice_snap = Command::new("/bin/bash")
                        .arg("-c")
                        .arg("command -v /snap/bin/libreoffice")
                        .stdout(Stdio::null())
                        .status();
        let check_libreoffice_snap_installed = match check_libreoffice_snap {
                Ok(installed) => installed.success(),
                Err(error) => {eprintln!("Failed to check if the LibreOffice Snap is installed: {}", error); false},
        };

        let check_libreoffice_flatpak = Command::new("/usr/bin/flatpak")
                        .arg("info")
                        .arg("org.libreoffice.LibreOffice")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status();
        let check_libreoffice_flatpak_installed = match check_libreoffice_flatpak {
                Ok(installed) => installed.success(),
                Err(error) => {eprintln!("Failed to check if the LibreOffice Flatpak is installed: {}", error); false},
        };

        if check_libreoffice_apt_installed || check_libreoffice_snap_installed || check_libreoffice_flatpak_installed {
		let libreoffice_installed_button = gtk::Button::with_label(&gettext("Installed"));
		libreoffice_installed_button.set_property_width_request(140);
		libreoffice_installed_button.set_sensitive(false);
		libreoffice_installed_button.show();
		libreoffice_container.add(&libreoffice_installed_button);
        } else {
		let libreoffice_install_button = gtk::Button::with_label(&gettext("Install"));
		libreoffice_install_button.set_property_width_request(100);
		libreoffice_install_button.connect_clicked(move |_| {
		        let result = Command::new("gnome-software")
		        	.arg("--details-pkg=libreoffice-writer")
		        	.spawn();
		        if result.is_err() {
		                println!("{}", "Failed to run command to open LibreOffice in Software")
		        }
		});
		libreoffice_install_button.get_style_context().add_class("suggested-action");
		libreoffice_install_button.show();
		libreoffice_container.add(&libreoffice_install_button);
        }

        let libreoffice_works_best_with_label = gtk::Label::new(Some(&gettext("Works best with")));
        libreoffice_works_best_with_label.set_justify(gtk::Justification::Center);
        libreoffice_works_best_with_label.set_valign(gtk::Align::Center);
        libreoffice_works_best_with_label.show();
        libreoffice_container.add(&libreoffice_works_best_with_label);

        let libreoffice_formats_container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        libreoffice_formats_container.set_halign(gtk::Align::Center);

        let odt_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
        odt_container.set_halign(gtk::Align::Center);
        let odt_icon = gtk::Image::from_icon_name(Some("x-office-document"), gtk::IconSize::Dnd);
        odt_icon.set_valign(gtk::Align::Start);
        odt_icon.show();
        odt_container.add(&odt_icon);
        let odt_label = gtk::Label::new(Some(".odt"));
        odt_label.set_justify(gtk::Justification::Center);
        odt_label.set_valign(gtk::Align::Center);
        odt_label.show();
        odt_container.add(&odt_label);
        odt_container.show();
        libreoffice_formats_container.add(&odt_container);

        let ods_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
        ods_container.set_halign(gtk::Align::Center);
        let ods_icon = gtk::Image::from_icon_name(Some("x-office-spreadsheet"), gtk::IconSize::Dnd);
        ods_icon.set_valign(gtk::Align::Start);
        ods_icon.show();
        ods_container.add(&ods_icon);
        let ods_label = gtk::Label::new(Some(".ods"));
        ods_label.set_justify(gtk::Justification::Center);
        ods_label.set_valign(gtk::Align::Center);
        ods_label.show();
        ods_container.add(&ods_label);
        ods_container.show();
        libreoffice_formats_container.add(&ods_container);

        let odp_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
        odp_container.set_halign(gtk::Align::Center);
        let odp_icon = gtk::Image::from_icon_name(Some("x-office-presentation"), gtk::IconSize::Dnd);
        odp_icon.set_valign(gtk::Align::Start);
        odp_icon.show();
        odp_container.add(&odp_icon);
        let odp_label = gtk::Label::new(Some(".odp"));
        odp_label.set_justify(gtk::Justification::Center);
        odp_label.set_valign(gtk::Align::Center);
        odp_label.show();
        odp_container.add(&odp_label);
        odp_container.show();
        libreoffice_formats_container.add(&odp_container);

        libreoffice_formats_container.show();
        libreoffice_container.add(&libreoffice_formats_container);

        libreoffice_container.show();
        office_suite_size_group.add_widget(&libreoffice_container);
        actions_container.add(&libreoffice_container);

        // ONLYOFFICE
        let onlyoffice_container = gtk::Box::new(gtk::Orientation::Vertical, 12);
        onlyoffice_container.set_halign(gtk::Align::Center);

        let onlyoffice_title = gtk::Label::new(Some("ONLYOFFICE"));
        onlyoffice_title.set_justify(gtk::Justification::Center);
        onlyoffice_title.set_valign(gtk::Align::Center);
        onlyoffice_title.get_style_context().add_class("app-title");
        onlyoffice_title.show();
        onlyoffice_container.add(&onlyoffice_title);

        let check_onlyoffice_snap = Command::new("/bin/bash")
                        .arg("-c")
                        .arg("command -v /snap/bin/onlyoffice-desktopeditors")
                        .stdout(Stdio::null())
                        .status();
        let check_onlyoffice_snap_installed = match check_onlyoffice_snap {
                Ok(installed) => installed.success(),
                Err(error) => {eprintln!("Failed to check if the ONLYOFFICE Snap is installed: {}", error); false},
        };

        let check_onlyoffice_flatpak = Command::new("/usr/bin/flatpak")
                        .arg("info")
                        .arg("org.onlyoffice.desktopeditors")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status();
        let check_onlyoffice_flatpak_installed = match check_onlyoffice_flatpak {
                Ok(installed) => installed.success(),
                Err(error) => {eprintln!("Failed to check if the ONLYOFFICE Flatpak is installed: {}", error); false},
        };

        if check_onlyoffice_snap_installed || check_onlyoffice_flatpak_installed {
		let onlyoffice_installed_button = gtk::Button::with_label(&gettext("Installed"));
		onlyoffice_installed_button.set_property_width_request(140);
		onlyoffice_installed_button.set_sensitive(false);
		onlyoffice_installed_button.show();
		onlyoffice_container.add(&onlyoffice_installed_button);
        } else {
		let onlyoffice_install_button = gtk::Button::with_label(&gettext("Install"));
		onlyoffice_install_button.set_property_width_request(100);
		onlyoffice_install_button.connect_clicked(move |_| {
		        let result = Command::new("gnome-software")
		        	.arg("--details=org.onlyoffice.desktopeditors")
		        	.spawn();
		        if result.is_err() {
		                println!("{}", "Failed to run command to open ONLYOFFICE in Software")
		        }
		});
		onlyoffice_install_button.get_style_context().add_class("suggested-action");
		onlyoffice_install_button.show();
		onlyoffice_container.add(&onlyoffice_install_button);
        }

        let onlyoffice_works_best_with_label = gtk::Label::new(Some(&gettext("Works best with")));
        onlyoffice_works_best_with_label.set_justify(gtk::Justification::Center);
        onlyoffice_works_best_with_label.set_valign(gtk::Align::Center);
        onlyoffice_works_best_with_label.show();
        onlyoffice_container.add(&onlyoffice_works_best_with_label);

        let onlyoffice_formats_container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        onlyoffice_formats_container.set_halign(gtk::Align::Center);

        let docx_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
        docx_container.set_halign(gtk::Align::Center);
        let docx_icon = gtk::Image::from_icon_name(Some("x-office-document"), gtk::IconSize::Dnd);
        docx_icon.set_valign(gtk::Align::Start);
        docx_icon.show();
        docx_container.add(&docx_icon);
        let docx_label = gtk::Label::new(Some(".docx"));
        docx_label.set_justify(gtk::Justification::Center);
        docx_label.set_valign(gtk::Align::Center);
        docx_label.show();
        docx_container.add(&docx_label);
        docx_container.show();
        onlyoffice_formats_container.add(&docx_container);

        let xlsx_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
        xlsx_container.set_halign(gtk::Align::Center);
        let xlsx_icon = gtk::Image::from_icon_name(Some("x-office-spreadsheet"), gtk::IconSize::Dnd);
        xlsx_icon.set_valign(gtk::Align::Start);
        xlsx_icon.show();
        xlsx_container.add(&xlsx_icon);
        let xlsx_label = gtk::Label::new(Some(".xlsx"));
        xlsx_label.set_justify(gtk::Justification::Center);
        xlsx_label.set_valign(gtk::Align::Center);
        xlsx_label.show();
        xlsx_container.add(&xlsx_label);
        xlsx_container.show();
        onlyoffice_formats_container.add(&xlsx_container);

        let pptx_container = gtk::Box::new(gtk::Orientation::Vertical, 6);
        pptx_container.set_halign(gtk::Align::Center);
        let pptx_icon = gtk::Image::from_icon_name(Some("x-office-presentation"), gtk::IconSize::Dnd);
        pptx_icon.set_valign(gtk::Align::Start);
        pptx_icon.show();
        pptx_container.add(&pptx_icon);
        let pptx_label = gtk::Label::new(Some(".pptx"));
        pptx_label.set_justify(gtk::Justification::Center);
        pptx_label.set_valign(gtk::Align::Center);
        pptx_label.show();
        pptx_container.add(&pptx_label);
        pptx_container.show();
        onlyoffice_formats_container.add(&pptx_container);

        onlyoffice_formats_container.show();
        onlyoffice_container.add(&onlyoffice_formats_container);

        onlyoffice_container.show();
        office_suite_size_group.add_widget(&onlyoffice_container);
        actions_container.add(&onlyoffice_container);

        actions_container.show();
        container.add(&actions_container);

        container.show();
        self.widget.add(&container);
        self.widget.show();
    }
}
