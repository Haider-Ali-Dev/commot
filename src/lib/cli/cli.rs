use clap::{Arg, Command};

use colorful::Color;
use colorful::Colorful;
use term_table::{Table, TableStyle, row::Row, table_cell::TableCell};

use crate::{errors::CommotError, reader::Reader};
/// Main function
/// This creates the CLI with the clap create and gets executed in the 
/// main binary file.
pub fn run() -> Result<(), CommotError> {
    let app = Command::new("commot")
        .about("A better alternative to cat command. It displays file and its metdata")
        .arg(Arg::new("FILE"));

        
    
    let matches = app.get_matches();

    match matches.value_of("FILE") {
        Some(e) => {
            let reader = Reader::new(e.to_owned());
            let data = reader.read()?;
            let metadata = reader.metadata()?;
            let mut table = Table::new();
            table.style = TableStyle::extended();
            table.add_row(Row::new(
                vec![
                    TableCell::new_with_alignment("File Content".gradient(Color::LightBlue), 8, term_table::table_cell::Alignment::Center),
                ]
            ));

            table.add_row(
                Row::new(
                    vec![
                        TableCell::new_with_alignment(data.into_inner(), 8, term_table::table_cell::Alignment::Left)
                    ]
                )
            );

            table.add_row(
                Row::new(
                    vec![
                        TableCell::new_with_alignment("Metadata".gradient(Color::Green), 8, term_table::table_cell::Alignment::Center)
                    ]
                )
            );

            table.add_row(
                Row::new(
                    vec![
                        TableCell::new_with_alignment
                        (&format!("Size: {}", metadata.size.into_inner().to_string()).gradient(Color::LightYellow)
                        , 4, term_table::table_cell::Alignment::Center),

                        TableCell::new_with_alignment(
                            format!("Last Modified: {}", metadata.modified.format("%B %d %Y:%I:%M:%S %p").to_string()).gradient(Color::LightCyan),
                            4,
                            term_table::table_cell::Alignment::Center
                        ),
                    ]

                 )
            );

            table.add_row(
                Row::new(
                    vec![
                        TableCell::new_with_alignment(
                            format!("Created: {}", metadata.created.format("%B %d %Y:%I:%M:%S %p").to_string())
                            .gradient(Color::LightGray),
                            4,
                            term_table::table_cell::Alignment::Center
                        ),
                        TableCell::new_with_alignment(
                            format!("Last Accessed on: {}", metadata.accessed.format("%B %d %Y:%I:%M:%S %p").to_string())
                            .gradient(Color::LightSteelBlue),
                            4,
                            term_table::table_cell::Alignment::Center
                        )
                    ]
                )
            );

            println!("{}", table.render());
            Ok(())
        },
        None => Err(CommotError::ArgNotFound("You did not provide the required arg which is a file.".to_string()))
    }
}