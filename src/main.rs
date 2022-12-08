extern crate csv;
#[macro_use]
extern crate json;
#[macro_use]
extern crate anyhow;
use anyhow::Result;
use csv::Reader;
use ctj::cli::Cli;
use json::JsonValue;
use std::{
    fs::{read_to_string, File, create_dir_all},
    io::Write, path::Path,
};

fn main() -> Result<()> {
    let client = Cli::build();
    if let Some(file_name) = client.source.extension() {
        if !file_name
            .to_string_lossy()
            .to_string()
            .cmp(&String::from("csv"))
            .is_eq()
        {
            return Err(anyhow!(
                "Src file is invalid. Should be specified and should contain the .csv extension!"
            ));
        }
    } else {
        return Err(anyhow!("Fetch Source file name failed!"));
    }

    let contents = read_to_string(client.source.clone())?;
    let json = array![];
    match client.destination {
        Some(destination) => {
            if let Some(detination_file_name) = destination.extension() {
                if !detination_file_name
                    .to_string_lossy()
                    .to_string()
                    .cmp(&String::from("json"))
                    .is_eq()
                {
                    return Err(anyhow!("Destination file is invalid. Should be specified and should contain the .json extension!"));
                }
            } else {
                return Err(anyhow!("Fetch destination file name failed!"));
            }
            write_to_file(json, contents, destination.to_string_lossy().to_string())?;
        }
        None => {
            let source_file = client.source;
            if let Some(source_file_name) = source_file.file_name() {
                let destination = source_file_name
                    .to_string_lossy()
                    .to_string()
                    .replace(".csv", ".json");
                write_to_file(json, contents, destination)?;
            }
        }
    };
    Ok(())
}

fn update_json_with_record_row(
    mut json: JsonValue,
    record: Vec<String>,
    headers: &[String],
) -> Result<JsonValue> {
    let mut element = object! {};
    for index in 0..headers.len() {
        if index >= record.len() {
            break;
        }

        let header: &str = &headers[index][..];
        let value: &str = &record[index];
        element[header] = value.into();
    }
    json.push(element.clone())?;
    Ok(json)
}

fn write_to_file(mut json: JsonValue, contents: String, destination: String) -> Result<()> {
    let mut rdr = Reader::from_reader(contents.as_bytes());
    let mut headers = Vec::new();
    let header_list = rdr.headers()?;
    for header in header_list.into_iter() {
        headers.push(header.to_owned());
    }
    let mut records_iter = rdr.records();
    while let Some(record) = records_iter.next() {
        let mut record_list = Vec::new();
        for item in record?.into_iter() {
            record_list.push(item.to_owned());
        }
        json = update_json_with_record_row(json, record_list, &headers)?;
    }
    if let Some(parent) = Path::new(&destination).parent() {
        create_dir_all(parent)?;
    }
    let mut file = File::create(&destination)?;
    file.write(json::stringify_pretty(json, 4).as_bytes())?;
    Ok(())
}
