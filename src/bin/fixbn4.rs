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
                return 0xe8;
            }

            tango_dataview::game::bn4::rom::EN_CHARSET
                .iter()
                .position(|c2| c.to_string() == *c2)
                .unwrap() as u8
        })
        .chain([0xe5])
        .chain(std::iter::repeat(0))
        .take(n)
        .collect::<Vec<_>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut save = tango_dataview::game::bn4::save::Save::new(&std::fs::read(&args.input_file)?)?;
    {
        let mut patch_cards_mut = match save.view_patch_cards_mut().unwrap() {
            tango_dataview::save::PatchCardsViewMut::PatchCard4s(patch_cards_mut) => {
                patch_cards_mut
            }
            tango_dataview::save::PatchCardsViewMut::PatchCard56s(_) => unreachable!(),
        };
        for i in 0..6 {
            patch_cards_mut.set_patch_card(i, None);
        }
    }
    {
        let mut abd_mut = save.view_auto_battle_data_mut().unwrap();
        for i in 0..tango_dataview::game::bn4::NUM_CHIPS {
            abd_mut.set_chip_use_count(i, 0);
        }
        abd_mut.clear_materialized();
    }

    let mut raw = save.as_raw_wram().to_vec();

    // Remove shift.
    raw[tango_dataview::game::bn4::save::SHIFT_OFFSET..][..4].copy_from_slice(b"\0\0\0\0");
    raw[tango_dataview::game::bn4::save::MASK_OFFSET..][..4].copy_from_slice(b"\0\0\0\0");
    raw[tango_dataview::game::bn4::save::CHECKSUM_OFFSET..][..4].copy_from_slice(b"\0\0\0\0");

    if args.us {
        // Duo
        {
            let offset = tango_dataview::game::bn4::save::EREADER_NAME_OFFSET;
            raw[offset..offset + tango_dataview::game::bn4::save::EREADER_NAME_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "Duo",
                    tango_dataview::game::bn4::save::EREADER_NAME_SIZE - 2,
                ));
        }
        {
            let offset = tango_dataview::game::bn4::save::EREADER_DESCRIPTION_OFFSET;
            raw[offset..offset + tango_dataview::game::bn4::save::EREADER_DESCRIPTION_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "Fist of\nJustice:\nMetKnukl!",
                    tango_dataview::game::bn4::save::EREADER_DESCRIPTION_SIZE - 2,
                ));
        }

        // PrixPowr
        {
            let offset = tango_dataview::game::bn4::save::EREADER_NAME_OFFSET
                + tango_dataview::game::bn4::save::EREADER_NAME_SIZE;
            raw[offset..offset + tango_dataview::game::bn4::save::EREADER_NAME_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "PrixPowr",
                    tango_dataview::game::bn4::save::EREADER_NAME_SIZE - 2,
                ));
        }
        {
            let offset = tango_dataview::game::bn4::save::EREADER_DESCRIPTION_OFFSET
                + tango_dataview::game::bn4::save::EREADER_DESCRIPTION_SIZE;
            raw[offset..offset + tango_dataview::game::bn4::save::EREADER_DESCRIPTION_SIZE - 2]
                .copy_from_slice(&convert_string(
                    "The three\nGrandPrix\nwinners!",
                    tango_dataview::game::bn4::save::EREADER_DESCRIPTION_SIZE - 2,
                ));
        }
    }

    std::fs::write(&args.output_file, &raw)?;

    Ok(())
}
