use clap::Parser;
use tango_dataview::save::Save;

#[derive(clap::Parser)]
struct Args {
    #[arg(long, default_value = "false")]
    us: bool,

    input_file: std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let save = tango_dataview::game::bn1::save::Save::new(&std::fs::read(&args.input_file)?)?;

    let mut raw = save.as_raw_wram().to_vec();

    raw[tango_dataview::game::bn1::save::CHECKSUM_OFFSET
        ..tango_dataview::game::bn1::save::CHECKSUM_OFFSET + 4]
        .copy_from_slice(b"\0\0\0\0");

    if args.us {
        // Rename the save.
        raw[tango_dataview::game::bn1::save::GAME_NAME_OFFSET
            ..tango_dataview::game::bn1::save::GAME_NAME_OFFSET + 20]
            .copy_from_slice(b"ROCKMAN EXE 20010727");
    }

    std::fs::write(&args.output_file, &raw)?;

    Ok(())
}
