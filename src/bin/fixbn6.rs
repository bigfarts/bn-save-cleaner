use byteorder::WriteBytesExt;
use clap::Parser;
use tango_dataview::save::Save;

fn convert_string(s: &str, n: usize) -> Vec<u8> {
    s.chars()
        .into_iter()
        .map(|c| {
            if c == '\n' {
                return 0xe9;
            }

            tango_dataview::game::bn6::rom::EN_CHARSET
                .iter()
                .position(|c2| c.to_string() == *c2)
                .unwrap() as u8
        })
        .chain([0xe6])
        .chain(std::iter::repeat(0))
        .take(n)
        .collect::<Vec<_>>()
}

#[derive(clap::Parser)]
struct Args {
    #[arg(long, default_value = "false")]
    us: bool,

    input_file: std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut save = tango_dataview::game::bn6::save::Save::new(&std::fs::read(&args.input_file)?)?;
    let variant = save.game_info().variant;

    if args.us {
        let mut chips_mut = save.view_chips_mut().unwrap();

        // Remove illegal chips.
        for id in [
            18,  // Gun del Sol EX
            278, // Django
            279, // Django V2
            280, // Django V3
            275, // Count
            276, // Count EX
            277, // Count SP
            153, // Otenko
            311, // Double Beast
            312, // Falzar
            313, // Gregar
        ] {
            for i in 0..4 {
                chips_mut.set_pack_count(id, i, 0);
            }
        }
    }

    let mut raw = if args.us {
        save.as_us_wram()
    } else {
        save.as_jp_wram()
    }
    .to_vec();

    raw[tango_dataview::game::bn6::save::MASK_OFFSET..][..4].copy_from_slice(b"\0\0\0\0");
    raw[tango_dataview::game::bn6::save::CHECKSUM_OFFSET..][..4].copy_from_slice(b"\0\0\0\0");

    if args.us {
        // Rename the save.
        raw[tango_dataview::game::bn6::save::GAME_NAME_OFFSET..][..20].copy_from_slice(
            match variant {
                tango_dataview::game::bn6::save::Variant::Gregar => b"REXE6 G 20060110a US",
                tango_dataview::game::bn6::save::Variant::Falzar => b"REXE6 F 20060110a US",
            },
        );

        // Double Beast
        {
            let offset = tango_dataview::game::bn6::save::EREADER_NAME_OFFSET
                + tango_dataview::game::bn6::save::EREADER_NAME_SIZE * 0;
            raw[offset..][..tango_dataview::game::bn6::save::EREADER_NAME_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "DblBeast",
                    tango_dataview::game::bn6::save::EREADER_NAME_SIZE - 2,
                ));
        }
        {
            let offset = tango_dataview::game::bn6::save::EREADER_DESCRIPTION_OFFSET
                + tango_dataview::game::bn6::save::EREADER_DESCRIPTION_SIZE * 0;
            raw[offset..][..tango_dataview::game::bn6::save::EREADER_DESCRIPTION_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "Ferocious\nbeast\npower!",
                    tango_dataview::game::bn6::save::EREADER_DESCRIPTION_SIZE - 2,
                ));
        }

        match variant {
            tango_dataview::game::bn6::save::Variant::Gregar => {
                // Gregar
                {
                    let offset = tango_dataview::game::bn6::save::EREADER_NAME_OFFSET
                        + tango_dataview::game::bn6::save::EREADER_NAME_SIZE * 1;
                    raw[offset..][..tango_dataview::game::bn6::save::EREADER_NAME_SIZE - 2]
                        .copy_from_slice(&convert_string(
                            "Gregar",
                            tango_dataview::game::bn6::save::EREADER_NAME_SIZE - 2,
                        ));
                }
                {
                    let offset = tango_dataview::game::bn6::save::EREADER_DESCRIPTION_OFFSET
                        + tango_dataview::game::bn6::save::EREADER_DESCRIPTION_SIZE * 1;
                    raw[offset..][..tango_dataview::game::bn6::save::EREADER_DESCRIPTION_SIZE - 2]
                        .copy_from_slice(&convert_string(
                            "Gregar's\nbreath\nattack!",
                            tango_dataview::game::bn6::save::EREADER_DESCRIPTION_SIZE - 2,
                        ));
                }
            }
            tango_dataview::game::bn6::save::Variant::Falzar => {
                // Falzar
                {
                    let offset = tango_dataview::game::bn6::save::EREADER_NAME_OFFSET
                        + tango_dataview::game::bn6::save::EREADER_NAME_SIZE * 1;
                    raw[offset..][..tango_dataview::game::bn6::save::EREADER_NAME_SIZE - 2]
                        .copy_from_slice(&convert_string(
                            "Falzar",
                            tango_dataview::game::bn6::save::EREADER_NAME_SIZE - 2,
                        ));
                }
                {
                    let offset = tango_dataview::game::bn6::save::EREADER_DESCRIPTION_OFFSET
                        + tango_dataview::game::bn6::save::EREADER_DESCRIPTION_SIZE * 1;
                    raw[offset..][..tango_dataview::game::bn6::save::EREADER_DESCRIPTION_SIZE - 2]
                        .copy_from_slice(&convert_string(
                            "Falzar's\nruinous\ntornado!",
                            tango_dataview::game::bn6::save::EREADER_DESCRIPTION_SIZE - 2,
                        ));
                }
            }
        }

        // Rename folder names.
        for (i, name) in ["LanFldr", "ExpoFldr", "GiftFldr"].into_iter().enumerate() {
            let offset = 0x1850 + i * 0x20;
            let mut buf = &mut raw[offset..offset + 0x8 * 2];
            for c in name.chars() {
                buf.write_u16::<byteorder::LittleEndian>(
                    tango_dataview::game::bn6::rom::EN_CHARSET
                        .iter()
                        .position(|c2| c.to_string() == *c2)
                        .unwrap() as u16
                        | 0xe500,
                )
                .unwrap();
            }
            while !buf.is_empty() {
                buf.write_u16::<byteorder::LittleEndian>(0xe5e5).unwrap();
            }
        }
    }

    std::fs::write(&args.output_file, &raw)?;

    Ok(())
}
