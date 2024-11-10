use std::error::Error;
use csv::{ReaderBuilder, StringRecord};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    generate: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.generate {

        match generate_weather_data(1000) {
            Ok(()) => println!("Weather data generated"),
            Err(e) => println!("Error when generating weather data: {:?}",e),
        }

    } else {

    }

    println!("generate: {:?}", cli.generate);
}


fn generate_weather_data(nrows: usize) -> Result<(), Box<dyn Error>> {

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("./data/weather_stations.csv")?;

    let stations: Vec<String> = rdr.records().into_iter().filter_map(|r| { 
        if r.is_ok() 
            { Some(r.unwrap()[0].to_string()) } else { None }
        }).collect();

    dbg!(stations);
    
    Ok(())

}