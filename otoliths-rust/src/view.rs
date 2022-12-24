use crate::model::Record;

pub fn print_menu() {
	println!();
	println!("CST8333 Project 4 by Juan Diego Serrano");
	println!();
	println!("What do you want to do? (Please type the option number)");
	println!("1. View records.");
	println!("2. Reload data with original data.");
	println!("3. Write memory data to a new file.");
	println!("4. Display a specific record from memory.");
	println!("5. Create a new record.");
	println!("6. Edit a specific record.");
	println!("7. Delete a specific record.");
	println!("8. Sort records.");
	println!();
	println!("0. Quit");
}

// PrintDisplayRecordMenu prints the display record menu.
pub fn print_display_record_menu() {
	println!();
	println!("CST8333 Project 4 by Juan Diego Serrano");
	println!();
	println!("Please type the record number you want to display: ");
	println!();
	println!("0. Quit");
}

// PrintSortMenu prints the sorting by menu.
// pub fn print_sort_menu() {
// 	println!();
// 	println!("CST8333 Project 4 by Juan Diego Serrano");
// 	println!();
// 	println!("H;ow do you want to sort by: ");
// 	println!("1;. Source.");
// 	println!("2;. Latin Name.");
// 	println!("3;. English Name.");
// 	println!("4;. French Name.");
// 	println!("5;. Year.");
// 	println!("6;. Month.");
// 	println!("7;. Number of Otoliths.");
// 	println!() ;
// 	println!("0. Quit");
// }

// PrintCreateRecordMenu prints the create record menu.
pub fn print_create_record_menu() {
	println!();
	println!("CST8333 Project 4 by Juan Diego Serrano");
	println!();
	println!("Please type the new record data in the following order");
	println!("Source, Latin Name, English Name, French Name, Year, Month, # Otoliths");
}

// PrintEditRecordMenu prints the edit record menu.
pub fn print_edit_record_menu() {
	println!();
	println!("CST8333 Project 4 by Juan Diego Serrano");
	println!();
	println!("Please type the record number to edit:");
}

// PrintEditConfirmation prints the prompt for editing a record.
pub fn print_edit_confirmation() {
	println!();
	println!("CST8333 Project 4 by Juan Diego Serrano");
	println!();
	println!("Please type the new data in the following order");
	println!("Source, Latin Name, English Name, French Name, Year, Month, # Otoliths");
}

// PrintDeleteRecordMenu prints the delete record prompt.
pub fn print_delete_record_menu() {
	println!();
	println!("CST8333 Project 4 by Juan Diego Serrano");
	println!();
	println!("Please type the record number you want to delete: ");
}

// PrintConfirmation prints the confirmation prompt for editing or deleting a record.
pub fn print_confirmation(record: &Record, i: u64, action: &str) {
	println!();
	println!("CST8333 Project 4 by Juan Diego Serrano");
	println!();
	println!("Are you sure you want to {}: ", action);
	print!("{}) Source: {}, Latin Name: {}, English Name: {}, French Name: {}, Year: {}, Month: {}, # Otoliths: {}\n",
		i+1, record.source, record.latin_name, record.english_name, record.french_name, record.year,
		record.month, record.number_otoliths);
	println!("Y/N");
}

// ShowOneRecord formats and prints the record based on the recordNumber passed as argument.
pub fn show_one_record(record_number: usize, record_list: &Vec<Record>) {
	let record: &Record = &record_list[record_number - 1];
	print!("{}) Source: {}, Latin Name: {}, English Name: {}, French Name: {}, Year: {}, Month: {}, # Otoliths: {}\n",
		record_number, record.source, record.latin_name, record.english_name, record.french_name, record.year,
		record.month, record.number_otoliths);
	println!();
	println!();
}

// printRecordList formats and prints []records (recordList).
pub fn print_record_list(record_list: &Vec<Record>) {
	println!();
	print!("Record List by Juan Diego Serrano:\n");
	for (i, record) in record_list.iter().enumerate() {
		print!("{}) Source: {}, Latin Name: {}, English Name: {}, French Name: {}, Year: {}, Month: {}, # Otoliths: {}\n",
			i+1, record.source, record.latin_name, record.english_name, record.french_name, record.year,
			record.month, record.number_otoliths);
	};
}
