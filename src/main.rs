use gtk4::InputHints;
use gtk4::prelude::{EditableExt, EntryExt, WidgetExt};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use rand::{Rng, thread_rng};
use std::borrow::BorrowMut;
use relm4_icons::icon_names;
struct AppModel {
    ip: Option<IpModel>,
    user_input: IpModel,
    valid: Validator,
}

#[derive(Debug,Clone)]
enum EntryInput {
    Mask(String),
    NetworkAddress(String),
    BroadcastAddress(String),
    FirstHost(String),
    LastHost(String),
    PossibleHosts(u32),
}

#[derive(Debug,Default,Clone)]
struct IpModel {
    ip: String,
    subnet_mask: u8,
    prefix: String,
    mask: String,
    binary_address: String,
    network_address: String,
    broadcast_address: String,
    first_host: String,
    last_host: String,
    possible_hosts: u32,
}

#[derive(Debug,Default)]
struct Validator {
    mask: bool,
    network_address: bool,
    broadcast_address: bool,
    first_host: bool,
    last_host: bool,
    possible_hosts: bool,
}





#[derive(Debug)]
enum AppMsg {
    GenerateIp,
    CheckIp,
    EntryInput(EntryInput),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppMsg;

    type Output = ();
    type Init = Option<IpModel>;

    view! {
        gtk::Window {
            set_title: Some("Ip Calculater"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Generate IP",
                    connect_clicked => AppMsg::GenerateIp,
                },

                #[name="ip_label"]
                gtk::Label {
                    #[watch]
                    set_label: format!("IP: {}{}", model.ip.as_ref().map(|ip| &ip.ip).unwrap_or(&"None".to_string()),model.ip.as_ref().map(|ip| &ip.prefix).unwrap_or(&"None".to_string())).as_str(),
                    set_margin_all: 5,
                },

                // Input Fields
                gtk::Entry {
                    set_placeholder_text: Some("Enter Subnet Mask"),
                    #[watch]
                    set_secondary_icon_name: if model.valid.mask { Some(icon_names::CHECK_PLAIN) } else { Some(icon_names::CROSS) },
                    set_margin_all: 5,
                    connect_changed[sender] => move |entry| {
                        if let text = entry.text() {
                            if let subnet_mask = text.to_string() {
                                sender.input(AppMsg::EntryInput(EntryInput::Mask(subnet_mask)));
                            }
                        }
                    },
                },
                gtk::Entry {
                    set_placeholder_text: Some("Network Address"),
                    #[watch]
                    set_secondary_icon_name: if model.valid.network_address { Some(icon_names::CHECK_PLAIN) } else { Some(icon_names::CROSS) },
                    set_margin_all: 5,
                    connect_changed[sender] => move |entry| {
                        if let text = entry.text() {
                            if let ip = text.to_string() {
                                sender.input(AppMsg::EntryInput(EntryInput::NetworkAddress(ip)));
                            }
                        }
                    },
                },
                gtk::Entry {
                    set_placeholder_text: Some("Broadcast Address"),
                    #[watch]
                    set_secondary_icon_name: if model.valid.broadcast_address { Some(icon_names::CHECK_PLAIN) } else { Some(icon_names::CROSS) },
                    set_margin_all: 5,
                    connect_changed[sender] => move |entry| {
                        if let text = entry.text() {
                            if let broadcast = text.to_string() {
                                sender.input(AppMsg::EntryInput(EntryInput::BroadcastAddress(broadcast)));
                            }
                        }
                    },
                },
                gtk::Entry {
                    set_placeholder_text: Some("First Host"),
                    #[watch]
                    set_secondary_icon_name: if model.valid.first_host { Some(icon_names::CHECK_PLAIN) } else { Some(icon_names::CROSS) },
                    set_margin_all: 5,
                    connect_changed[sender] => move |entry| {
                        if let text = entry.text() {
                            if let first = text.to_string() {
                                sender.input(AppMsg::EntryInput(EntryInput::FirstHost(first)));
                            }
                        }
                    },
                },
                gtk::Entry {
                    set_placeholder_text: Some("Last Host"),
                    #[watch]
                    set_secondary_icon_name: if model.valid.last_host { Some(icon_names::CHECK_PLAIN) } else { Some(icon_names::CROSS) },
                    set_margin_all: 5,
                    connect_changed[sender] => move |entry| {
                        if let text = entry.text() {
                            if let last = text.to_string() {
                                sender.input(AppMsg::EntryInput(EntryInput::LastHost(last)));
                            }
                        }
                    },
                },
                gtk::Entry {
                    set_placeholder_text: Some("Possible Hosts"),
                    #[watch]
                    set_secondary_icon_name: if model.valid.possible_hosts { Some(icon_names::CHECK_PLAIN) } else { Some(icon_names::CROSS) },
                    set_margin_all: 5,
                    connect_changed[sender] => move |entry| {
                        if let text = entry.text() {
                            if let possible = text.to_string() {
                                sender.input(AppMsg::EntryInput(EntryInput::PossibleHosts(possible.parse().unwrap())));
                            }
                        }
                    },
                },
                gtk::Button {
                    set_label: "Check IP",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::CheckIp);
                    },
                },
            }
        }
    }

    // Initialize the UI.
    fn init(
        ip: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = AppModel {ip: None, user_input: IpModel::default(),valid: Validator::default()};

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::GenerateIp => {
                self.ip = Some(ip_calculations());
            }
            AppMsg::EntryInput(input) => {
                match input {
                    EntryInput::Mask(mask) => {
                        self.user_input.mask = mask;
                    }
                    EntryInput::NetworkAddress(ip) => {
                        self.user_input.ip = ip;
                    }
                    EntryInput::BroadcastAddress(broadcast) => {
                        self.user_input.broadcast_address = broadcast;
                    }
                    EntryInput::FirstHost(first) => {
                        self.user_input.first_host = first;
                    }
                    EntryInput::LastHost(last) => {
                        self.user_input.last_host = last;
                    }
                    EntryInput::PossibleHosts(possible) => {
                        self.user_input.possible_hosts = possible;
                    }
                }
            }
            AppMsg::CheckIp => {
                self.valid = Validator {
                    mask: self.ip.as_ref().map(|ip| &ip.mask).unwrap_or(&"".to_string()) == &self.user_input.mask,
                    network_address: self.ip.as_ref().map(|ip| &ip.network_address).unwrap_or(&"".to_string()) == &self.user_input.ip,
                    broadcast_address: self.ip.as_ref().map(|ip| &ip.broadcast_address).unwrap_or(&"".to_string()) == &self.user_input.broadcast_address,
                    first_host: self.ip.as_ref().map(|ip| &ip.first_host).unwrap_or(&"".to_string()) == &self.user_input.first_host,
                    last_host: self.ip.as_ref().map(|ip| &ip.last_host).unwrap_or(&"".to_string()) == &self.user_input.last_host,
                    possible_hosts: self.ip.as_ref().map(|ip| &ip.possible_hosts).unwrap_or(&0) == &self.user_input.possible_hosts,
                };
            }
        }
    }
}

fn ip_calculations() -> IpModel {
    let mut rng = thread_rng();
     let octets: [u8; 4] = [
        rng.gen_range(111..255),
        rng.gen_range(111..255),
        rng.gen_range(111..255),
        rng.gen_range(111..255),
    ];
    let subnet_mask: u8 = rng.gen_range(18..28);
    let prefix = format!("\\{}", subnet_mask);
    let host_bits = 32-subnet_mask;
    let ip = format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3]);
    let mut binary_mask = "1".repeat(subnet_mask as usize) + &"0".repeat(32-subnet_mask as usize);
    binary_mask = add_dots(&binary_mask);
    let binary_address = format!("{:08b}.{:08b}.{:08b}.{:08b}", octets[0], octets[1], octets[2], octets[3]);

    // calculate network address
    let mut network_address = String::new();
    for (i, c) in binary_address.chars().enumerate() {
        if c == '.' {
            network_address.push('.');
        } else {
            network_address.push_str(&((c.to_digit(10).unwrap() & binary_mask.chars().nth(i).unwrap().to_digit(10).unwrap()).to_string()));
        }
    }

    // calculate broadcast address
    let mut broadcast_address = String::new();
    for (i, c) in binary_address.chars().enumerate() {
        if i >= subnet_mask as usize {
            if c == '.' {
                broadcast_address.push('.');
            } else {
                broadcast_address.push('1');
            }
        } else {
            broadcast_address.push(c);
        }
    }
    // calculate broadcast address
    let mut broadcast_address = String::new();
    for (i, c) in network_address.chars().enumerate() {
        if i >= (subnet_mask + subnet_mask/8) as usize {
            if c == '.' {
                broadcast_address.push('.');
            } else {
                broadcast_address.push('1');
            }
        } else {
            broadcast_address.push(c);
        }
    }

    // calculate first host address
    let mut first_host = network_address.clone();
    let first_host_int = u32::from_str_radix(&first_host.replace(".", ""), 2).unwrap() + 1;
    first_host = format!("{:032b}", first_host_int);
    first_host = add_dots(&first_host);
    // calculate last host address
    let mut last_host = broadcast_address.clone();
    let last_host_int = u32::from_str_radix(&last_host.replace(".", ""), 2).unwrap() - 1;
    last_host = format!("{:032b}", last_host_int);
    last_host = add_dots(&last_host);

    broadcast_address = binary_dotted_to_decimal(&broadcast_address);

    first_host = binary_dotted_to_decimal(&first_host);
    binary_mask = binary_dotted_to_decimal(&binary_mask);
    last_host = binary_dotted_to_decimal(&last_host);
    network_address = binary_dotted_to_decimal(&network_address);
    // calculate number of possible hosts
    let number_of_hosts = if subnet_mask < 31 { 2u32.pow(32 - subnet_mask as u32) - 2 } else { 0 };
    let model = IpModel {
        ip,
        subnet_mask,
        prefix,
        mask: binary_mask,
        binary_address,
        network_address,
        broadcast_address,
        first_host,
        last_host,
        possible_hosts: number_of_hosts,
    };
    println!("{:?}", model);
    return model;
}

fn add_dots(binary_string: &str) -> String {
    let mut result = String::new();
    for (i, c) in binary_string.chars().enumerate() {
        if i % 8 == 0 && i != 0 {
            result.push('.');
        }
        result.push(c);
    }
    result
}
fn binary_dotted_to_decimal(binary_dotted: &str) -> String {
    let mut result = String::new();
    for octet in binary_dotted.split('.') {
        result.push_str(&u8::from_str_radix(octet, 2).unwrap().to_string());
        result.push('.');
    }
    result.pop();
    result
}





fn main() {
    let app = RelmApp::new("timtom2016.com.IpChecker");

    relm4_icons::initialize_icons();
    app.run::<AppModel>(None);
}