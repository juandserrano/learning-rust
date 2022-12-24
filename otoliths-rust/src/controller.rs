use crate::{
    model::Record,
    view::{self, *},
};

const DATA_PATH: &str = "./src/dataset.csv";
// ParseFile opens the dataset file from [filepath] and scans line by line, appending each line to an slice of records.
// Returns an error or nil if parsing is successfull
pub fn parse_file(record_list: &mut Vec<Record>) -> Result<(), csv::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(DATA_PATH)?;
    for result in rdr.records() {
        let record = result?;
        let rec = Record {
            source: record[0].to_string(),
            latin_name: record[1].to_string(),
            english_name: record[2].to_string(),
            french_name: record[3].to_string(),
            year: record[4].parse().unwrap(),
            month: record[5].parse().unwrap(),
            number_otoliths: record[6].parse().unwrap(),
        };
        record_list.push(rec);
    }
    Ok(())
}

pub fn handle_option(user_option: u32, record_list: &mut Vec<Record>) {
    match user_option {
        1 => print_record_list(record_list),
        2 => {
            *record_list = Vec::new();
            _ = parse_file(record_list);
        }
        3 => _ = write_data_to_csv(record_list),
        4 => {
            view::print_display_record_menu();

            let input = get_user_input();
            if input > record_list.len().try_into().unwrap() || input == 666 {
                println!("Not a valid number.");
                return;
            }
            if input == 0 {
                std::process::exit(0);
            } else {
                view::show_one_record(input.try_into().unwrap(), record_list);
            }
        }
        5 => {
            view::print_create_record_menu();
            let input = get_string_input();
            create_record_from_string(input, record_list);
        }
        6 => {
            view::print_edit_record_menu();
            let input = get_user_input();
            if input > record_list.len().try_into().unwrap() || input == 666 {
                println!("Not a valid number.");
                return;
            }
            if input == 0 {
                std::process::exit(0);
            } else {
                edit_record(input, record_list);
            }
        }
        7 => {
                      view::print_delete_record_menu();
            		let input = get_user_input();
            if input > record_list.len().try_into().unwrap() || input == 666 {
                println!("Not a valid number.");
                return;
            }
            if input == 0 {
                std::process::exit(0);
            		} else {
            			delete_record(input, record_list);
            		}
        }
        _ => println!("Not a valid option"),
    };
}

pub fn get_user_input() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error getting user input");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_e) => 666,
    };
    input
}

pub fn get_string_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error getting user input");
    input.trim().to_string()
}

// // WriteDataToCsv creates a new writer that takes the data from model.Record and creates a csv file.
fn write_data_to_csv(record_list: &mut Vec<Record>) -> Result<(), Box<dyn std::error::Error>> {
    let mut wrt = csv::Writer::from_path("otoliths.csv")?;
    for record in record_list.iter() {
        let source = &record.source;
        let latin = &record.latin_name;
        let english = &record.english_name;
        let french = &record.french_name;
        let year = &record.year;
        let month = &record.month;
        let number = &record.number_otoliths;
        wrt.write_record(&[
            source,
            latin,
            english,
            french,
            &year.to_string(),
            &month.to_string(),
            &number.to_string(),
        ])?;
    }
    wrt.flush()?;
    Ok(())
}

// // CreateRecordFromString Takes a string, splits it into a slice on commas, trims outer spaces and creates a new record.
// // It then it appends the record to the list of records [model.RecordList].
fn create_record_from_string(input: String, record_list: &mut Vec<Record>) {
    let mut temp_record: Vec<&str> = input.split(',').collect();

    for element in &mut temp_record {
        *element = element.trim();
    }

    let new_record = Record {
        source: temp_record[0].to_string(),
        latin_name: temp_record[1].to_string(),
        english_name: temp_record[2].to_string(),
        french_name: temp_record[3].to_string(),
        year: temp_record[4].parse().unwrap(),
        month: temp_record[5].parse().unwrap(),
        number_otoliths: temp_record[6].parse().unwrap(),
    };

    record_list.push(new_record);
}

// // deleteRecord removes a record based on the number passed as argument.
fn delete_record(number: u32, record_list: &mut Vec<Record>) {
    let number: usize = number.try_into().unwrap();
    let record = &record_list[number-1];
	view::print_confirmation(&record, (number - 1).try_into().unwrap(), "delete");
	let input = get_string_input();
	if input == "Y" || input == "y" {
        record_list.remove(number - 1);
		print!("Record {} deleted successfully.", number)
	}
}

// // editRecord take a record number and calls view.PrintEditConfirmation which prompt the user for new record
// // information, creates such record and replaces the previous record.
fn edit_record(number: u32, record_list: &mut Vec<Record>) {
    let number: usize = number.try_into().unwrap();
    let record = &record_list[number - 1];
    view::print_confirmation(&record, (number - 1).try_into().unwrap(), "edit");
    let mut input = get_string_input();
    if input == "Y" || input == "y" {
        view::print_edit_confirmation();
        input = get_string_input();
        let mut temp_record: Vec<&str> = input.split(',').collect();
        for val in &mut temp_record {
            *val = val.trim();
        }

        let new_record = Record {
            source: temp_record[0].to_string(),
            latin_name: temp_record[1].to_string(),
            english_name: temp_record[2].to_string(),
            french_name: temp_record[3].to_string(),
            year: temp_record[4].parse().unwrap(),
            month: temp_record[5].parse().unwrap(),
            number_otoliths: temp_record[6].parse().unwrap(),
        };
        record_list[number - 1] = new_record;
    }
}
