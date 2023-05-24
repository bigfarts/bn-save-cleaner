use clap::Parser;
use tango_dataview::save::Save;

#[derive(clap::Parser)]
struct Args {
    #[arg(long, default_value = "false")]
    us: bool,

    input_file: std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn convert_string(s: &str, n: usize) -> Vec<u8> {
    s.chars()
        .into_iter()
        .map(|c| {
            if c == '\n' {
                return 0xe9;
            }

            tango_dataview::game::bn5::rom::EN_CHARSET
                .iter()
                .position(|c2| c.to_string() == *c2)
                .unwrap() as u8
        })
        .chain([0xe6])
        .chain(std::iter::repeat(0))
        .take(n)
        .collect::<Vec<_>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut save = tango_dataview::game::bn5::save::Save::new(&std::fs::read(&args.input_file)?)?;
    {
        let mut abd_mut = save.view_auto_battle_data_mut().unwrap();
        for i in 0..tango_dataview::game::bn5::NUM_CHIPS {
            abd_mut.set_chip_use_count(i, 0);
        }
        abd_mut.clear_materialized();
    }
    let variant = save.game_info().variant;

    let mut raw = save.as_raw_wram().to_vec();

    raw[tango_dataview::game::bn5::save::MASK_OFFSET
        ..tango_dataview::game::bn5::save::MASK_OFFSET + 4]
        .copy_from_slice(b"\0\0\0\0");

    raw[tango_dataview::game::bn5::save::CHECKSUM_OFFSET
        ..tango_dataview::game::bn5::save::CHECKSUM_OFFSET + 4]
        .copy_from_slice(b"\0\0\0\0");

    if args.us {
        // Rename the save.
        raw[tango_dataview::game::bn5::save::GAME_NAME_OFFSET
            ..tango_dataview::game::bn5::save::GAME_NAME_OFFSET + 20]
            .copy_from_slice(match variant {
                tango_dataview::game::bn5::save::Variant::Protoman => b"REXE5TOB 20041006 US",
                tango_dataview::game::bn5::save::Variant::Colonel => b"REXE5TOK 20041006 US",
            });

        // LeadRaid
        {
            let offset = tango_dataview::game::bn5::save::EREADER_NAME_OFFSET;
            raw[offset..offset + tango_dataview::game::bn5::save::EREADER_NAME_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "LeadRaid",
                    tango_dataview::game::bn5::save::EREADER_NAME_SIZE - 2,
                ));
        }
        {
            let offset = tango_dataview::game::bn5::save::EREADER_DESCRIPTION_OFFSET;
            raw[offset..offset + tango_dataview::game::bn5::save::EREADER_DESCRIPTION_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "ProtoMan\n& Colonel\ntogether!",
                    tango_dataview::game::bn5::save::EREADER_DESCRIPTION_SIZE - 2,
                ));
        }

        // ChaosLrd
        {
            let offset = tango_dataview::game::bn5::save::EREADER_NAME_OFFSET
                + tango_dataview::game::bn5::save::EREADER_NAME_SIZE;
            raw[offset..offset + tango_dataview::game::bn5::save::EREADER_NAME_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "ChaosLrd",
                    tango_dataview::game::bn5::save::EREADER_NAME_SIZE - 2,
                ));
        }
        {
            let offset = tango_dataview::game::bn5::save::EREADER_DESCRIPTION_OFFSET
                + tango_dataview::game::bn5::save::EREADER_DESCRIPTION_SIZE;
            raw[offset..offset + tango_dataview::game::bn5::save::EREADER_DESCRIPTION_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "Hatred\nformed\ninto Bass",
                    tango_dataview::game::bn5::save::EREADER_DESCRIPTION_SIZE - 2,
                ));
        }
    }

    std::fs::write(&args.output_file, &raw)?;

    Ok(())
}
