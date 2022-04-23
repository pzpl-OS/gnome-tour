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
                "Otwórz menu, aby uruchamiać programy".to_string(),
                gettext("Możesz szukać aplikacji i skrótów do często używanych lokalizacji w menu."),
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
                        "Zmień wygląd swojego pulpitu dzięki Zorin Appearance".to_string(),
                        "Zmień ułożenie pulpitu i jego styl wizualny.".to_string(),
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
                            vm_button = (format!("{}", "Zainstaluj VM Tools dla VMware"), "apturl", "apt:open-vm-tools-desktop");
                    } else {
                            vm_button = (format!("{}", "Zainstaluj Guest Additions dla VirtualBox"), "apturl", "apt:zorin-virtualbox-guest-additions");
                    }
                    self.paginator.borrow_mut().add_page(
                        ImageButtonPageWidget::new(
                            "/org/gnome/Tour/virtual-machine.svg",
                            "Przyśpiesz swoją maszynę wirtualną".to_string(),
                            format!("{} {}", "pzpl OS wykrył, że korzystasz z maszyny wirtualnej. ", "Aby zwiększyć wydajność, zainstaluj dodatki dla twojej maszyny wirtualnej i zrestartuj system."),
                            vec![vm_button],
                        )
                        .widget
                        .upcast::<gtk::Widget>(),
                    );
            } else if !virtual_machine.contains("none") {
                    self.paginator.borrow_mut().add_page(
                        ImagePageWidget::new(
                            "/org/gnome/Tour/virtual-machine.svg",
                            "Szybkość może być ograniczona w maszynie wirtualnej".to_string(),
                            format!("{} {}", "pzpl OS wykrył, że korzystasz z maszyny wirtualnej. ", gettext("Dla najlepszego doświadczenia, zainstaluj Zorin OS bezpośrednio na swój komputer.")),
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
                    "Połącz swoje internetowe konta".to_string(),
                    gettext("Połącz swoje konta aby w łatwy sposób korzystać ze swoich kalendarzy online, zdjęć, dokumentów i nie tylko."),
                    vec![(format!("{}", "Połącz konta online"), "gnome-control-center", "online-accounts")],
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
                                "Połącz swój telefon z komputerem wykorzystując narzędzie Zorin Connect".to_string(),
                                "Synchronizuj powiadomienia, odpowiadaj na wiadomości, dziel pliki, wykorzystuj swój telefon jako pilot do komputera i jeszcze więcej.".to_string(),
                                vec![(format!("{} {}", "Otwórz", "Zorin Connect"), "zorin-connect", "")],
                            )
                            .widget
                            .upcast::<gtk::Widget>(),
                        );
                }
        }

        self.paginator.borrow_mut().add_page(
            ImageButtonPageWidget::new(
                "/org/gnome/Tour/zorin-os-software.svg",
                "Użyj Menedżera oprogramowania do wyszukiwania i instalowania programów".to_string(),
                "Odkrywaj świetne programy za pomocą wyszukiwarki, przeglądarki i naszych ".to_string(),
                vec![(format!("{} {}", "Otwórz", "Menedżer oprogramowania"), "gnome-software", ""), (format!("{}", gettext("Learn More about Installing Apps")), "xdg-open", "https://help.zorin.com/docs/apps-games/install-apps/")],
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
                        "Pakiet biurowy do pracy".to_string(),
                        "Wybierz pakiet który najlepiej pracuje z dokumentami których używasz. Obydwa pakiety są kompatybilne z dokumentami Microsoft Office oraz z innymi formatami.".to_string(),
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
            i18n_f("Wszystko gotowe! Mamy nadzieję, że korzystanie z systemu {} przyniesie Ci radość.", &[&name]),
            gettext("To get more advice and tips, see the Help page on our website."),
            vec![(format!("{}", gettext("Visit the Help page")), "xdg-open", "https://help.zorin.com")],
        );
        last_page.widget.get_style_context().add_class("last-page");
        self.paginator.borrow_mut().add_page(last_page.widget.upcast::<gtk::Widget>());

        self.widget.add(&self.paginator.borrow().widget);
    }
}
