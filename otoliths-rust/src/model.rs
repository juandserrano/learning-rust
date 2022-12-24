#[derive(Debug)]
pub struct Record {
    pub source: String,
    pub latin_name: String,
    pub english_name: String,
    pub french_name: String,
    pub year: u32,
    pub month: u8,
    pub number_otoliths: u64,
}