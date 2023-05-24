use clap::Parser;
use tango_dataview::save::Save;

#[derive(clap::Parser)]
struct Args {
    input_file: std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let save = tango_dataview::game::exe45::save::Save::new(&std::fs::read(&args.input_file)?)?;

    let mut raw = save.as_raw_wram().to_vec();

    raw[tango_dataview::game::exe45::save::MASK_OFFSET..][..4].copy_from_slice(b"\0\0\0\0");
    raw[tango_dataview::game::exe45::save::CHECKSUM_OFFSET..][..4].copy_from_slice(b"\0\0\0\0");

    std::fs::write(&args.output_file, &raw)?;

    Ok(())
}
