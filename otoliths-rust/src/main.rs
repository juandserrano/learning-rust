mod controller;
mod model;
mod view;

use model::Record;

// A record holds the information for each line of the dataset.
fn main() {
    let mut record_list: Vec<Record> = Vec::new();
    _ = controller::parse_file(&mut record_list);
    loop {
        view::print_menu();
        let option: u32 = controller::get_user_input();
        if option == 0 {
            break;
        }
        controller::handle_option(option, &mut record_list);
    }
}
