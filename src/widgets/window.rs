use crate::utils::i18n_f;
use gettextrs::gettext;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::process::{Command, Stdio};
use ::std::env;

use super::pages::{ImagePageWidget, ImageButtonPageWidget, OfficeSuitesPageWidget, WelcomePageWidget};
use super::paginator::PaginatorWidget;
use crate::config::{APP_ID, PROFILE};

pub struct Window {
    pub widget: libhandy::ApplicationWindow,
    pub paginator: RefCell<Rc<PaginatorWidget>>,
}

impl Window {
    pub fn new(app: &gtk::Application) -> Self {
        let widget = libhandy::ApplicationWindow::new();
        widget.set_application(Some(app));

        let paginator = RefCell::new(PaginatorWidget::new());

        let mut window_widget = Window { widget, paginator };

        window_widget.init();
        window_widget
    }

    pub fn start_tour(&self) {
        self.paginator.borrow_mut().set_page(1);
    }

    pub fn reset_tour(&self) {
        self.paginator.borrow_mut().set_page(0);
    }

    fn init(&mut self) {
        self.widget.set_default_size(818, 600);
        self.widget.set_icon_name(Some(APP_ID));

        self.widget.set_position(gtk::WindowPosition::Center);
         // Checking the current desktop environment
        let desktop_environment = match env::var("XDG_CURRENT_DESKTOP") {
                Err(_) => String::from("Unknown"),
                Ok(current_desktop) => current_desktop,
        };
         // Checking what virtual machine software is being used (if any)
        let virtual_machine_detect = Command::new("systemd-detect-virt")
                .output();
        let virtual_machine = match virtual_machine_detect {
                Ok(used) => match String::from_utf8(used.stdout) {
                        Ok(output) => output,
                        Err(_) => String::from("Error"),
                },
                Err(error) => {eprintln!("Failed to check if using a Virtual Machine: {}", error); String::from("none")},
        };
         // Checking if the current user is a system administrator
        let user_in_sudoers = Command::new("/bin/bash")
                .arg("-c")
                .arg("groups | grep sudo")
                .stdout(Stdio::null())
                .status();
        if user_in_sudoers.is_err() {
                eprintln!("{}", "Failed to check if user is in the sudo group")
        }
        let user_is_admin = match user_in_sudoers {
                Ok(admin) => admin.success(),
                Err(error) => {eprintln!("Failed to check if user is an administrator: {}", error); false},
        };

        // Devel Profile
        if PROFILE == "Devel" {
            self.widget.get_style_context().add_class("devel");
        }
        self.paginator.borrow_mut().add_page(WelcomePageWidget::new().widget.upcast::<gtk::Widget>());

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/zorin-menu.svg",
                gettext("Open the menu to launch apps"),
                gettext("You can search for apps and find shortcuts to common locations in the menu."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        // Zorin Appearance page
        let check_zorin_appearance = Command::new("/bin/bash")
                        .arg("-c")
                        .arg("command -v zorin-appearance")
                        .stdout(Stdio::null())
                        .status();
        let check_zorin_appearance_installed = match check_zorin_appearance {
                Ok(installed) => installed.success(),
                Err(error) => {eprintln!("Failed to check if Zorin Appearance is installed: {}", error); false},
        };

        if check_zorin_appearance_installed {
                self.paginator.borrow_mut().add_page(
                    ImageButtonPageWidget::new(
                        "/org/gnome/Tour/zorin-appearance.svg",
                        gettext("Choose your desktop look with Zorin Appearance"),
                        gettext("Select a different layout for your desktop and change its visual style."),
                        vec![(format!("{} {}", gettext("Launch"), gettext("Zorin Appearance")), "zorin-appearance", "")],
                    )
                    .widget
                    .upcast::<gtk::Widget>(),
                );
        }

        // Virtual machine page
        if user_is_admin {
            if virtual_machine.contains("oracle") || virtual_machine.contains("vmware") {
                    let vm_button: (String, &'static str, &'static str);
                    if virtual_machine.contains("vmware") {
                            vm_button = (format!("{}", gettext("Install VM Tools for VMware")), "apturl", "apt:open-vm-tools-desktop");
                    } else {
                            vm_button = (format!("{}", gettext("Install Guest Additions for VirtualBox")), "apturl", "apt:zorin-virtualbox-guest-additions");
                    }
                    self.paginator.borrow_mut().add_page(
                        ImageButtonPageWidget::new(
                            "/org/gnome/Tour/virtual-machine.svg",
                            gettext("Speed-up your Virtual Machine"),
                            format!("{} {}", gettext("Zorin OS has detected that you're using a Virtual Machine."), gettext("To improve performance, install the extensions for your virtualization software and restart the system.")),
                            vec![vm_button],
                        )
                        .widget
                        .upcast::<gtk::Widget>(),
                    );
            } else if !virtual_machine.contains("none") {
                    self.paginator.borrow_mut().add_page(
                        ImagePageWidget::new(
                            "/org/gnome/Tour/virtual-machine.svg",
                            gettext("Speed may be limited in a Virtual Machine"),
                            format!("{} {}", gettext("Zorin OS has detected that you're using a Virtual Machine."), gettext("For the best experience and performance, please install Zorin OS directly onto your computer.")),
                        )
                        .widget
                        .upcast::<gtk::Widget>(),
                    );
            }
        }

        // Online accounts page
        if desktop_environment == "zorin:GNOME" {
            self.paginator.borrow_mut().add_page(
                ImageButtonPageWidget::new(
                    "/org/gnome/Tour/online-accounts.svg",
                    gettext("Connect your online accounts"),
                    gettext("Sync your accounts to easily access your online calendar, documents, photos, and more."),
                    vec![(format!("{}", gettext("Connect Online Accounts")), "gnome-control-center", "online-accounts")],
                )
                .widget
                .upcast::<gtk::Widget>(),
            );
        }
        // Zorin Connect page
        if desktop_environment == "zorin:GNOME" {
                let check_zorin_connect = Command::new("/bin/bash")
                                .arg("-c")
                                .arg("command -v zorin-connect")
                                .stdout(Stdio::null())
                                .status();
                let check_zorin_connect_installed = match check_zorin_connect {
                        Ok(installed) => installed.success(),
                        Err(error) => {eprintln!("Failed to check if Zorin Connect is installed: {}", error); false},
                };
                if check_zorin_connect_installed {
                        self.paginator.borrow_mut().add_page(
                            ImageButtonPageWidget::new(
                                "/org/gnome/Tour/zorin-connect.svg",
                                gettext("Link your phone and computer with Zorin Connect"),
                                gettext("Integrate your devices to sync notifications, reply to messages on the desktop, share files, use your phone as a remote control for your computer, and more."),
                                vec![(format!("{} {}", gettext("Launch"), gettext("Zorin Connect")), "zorin-connect", "")],
                            )
                            .widget
                            .upcast::<gtk::Widget>(),
                        );
                }
        }

        self.paginator.borrow_mut().add_page(
            ImageButtonPageWidget::new(
                "/org/gnome/Tour/zorin-os-software.svg",
                gettext("Use Software to find and install apps"),
                gettext("Discover great apps through search, browsing and our recommendations."),
                vec![(format!("{} {}", gettext("Launch"), gettext("Software")), "gnome-software", ""), (format!("{}", gettext("Learn More about Installing Apps")), "xdg-open", "https://help.zorin.com/docs/apps-games/install-apps/")],
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        // Office suites page
        if desktop_environment == "zorin:GNOME" {
            if user_is_admin {
                 self.paginator.borrow_mut().add_page(
                    OfficeSuitesPageWidget::new(
                        "/org/gnome/Tour/office.svg",
                        gettext("Office suite for work"),
                        gettext("Choose the office suite that works best with the documents you use. Both office suites are compatible with Microsoft Office documents and other formats."),
                    )
                    .widget
                    .upcast::<gtk::Widget>(),
                );
            }
        }

        self.paginator.borrow_mut().add_page(
            ImageButtonPageWidget::new(
                "/org/gnome/Tour/zorin-os-software.svg",
                gettext("Odkryj oprogramowanie edukacyjne dla twoich uczniów"),
                gettext("Wybierz grupę wiekową aby zobaczyć propozycje."),
                vec![(format!("{}", gettext("Przedszkole")), "xdg-open", "https://pzpl.ovh/os/redir.php?target=gnometour_kindergarten&v=delft"),
                (format!("{}", gettext("Klasy 1-3")), "xdg-open", "https://pzpl.ovh/os/redir.php?target=gnometour_1_3&v=delft"),
                (format!("{}", gettext("Klasy 4-6")), "xdg-open", "https://pzpl.ovh/os/redir.php?target=gnometour_4_6&v=delft"),
                (format!("{}", gettext("Klasy 7-8")), "xdg-open", "https://pzpl.ovh/os/redir.php?target=gnometour_7_8&v=delft"),
                (format!("{}", gettext("Szkoła Ponadpostawowa")), "xdg-open", "https://pzpl.ovh/os/redir.php?target=gnometour_middleschool&v=delft")],
            )
            .widget
            .upcast::<gtk::Widget>(),
        );


        let name = glib::get_os_info("NAME").unwrap_or_else(|| "GNOME".into());
        let last_page = ImageButtonPageWidget::new(
            "/org/gnome/Tour/ready-to-go.svg",
            // Translators: The following string is formated as "We hope that you enjoy GNOME"
            i18n_f("That's it! We hope that you enjoy {}.", &[&name]),
            gettext("To get more advice and tips, see the Help page on our website."),
            vec![(format!("{}", gettext("Visit the Help page")), "xdg-open", "https://help.zorin.com")],
        );
        last_page.widget.get_style_context().add_class("last-page");
        self.paginator.borrow_mut().add_page(last_page.widget.upcast::<gtk::Widget>());

        self.widget.add(&self.paginator.borrow().widget);
    }
}
