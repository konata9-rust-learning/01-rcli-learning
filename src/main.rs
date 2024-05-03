use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_sign, Base64Subcommand,
    Opts, Subcommand, TextSignFormat, TextSubcommand,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    match opts.cmd {
        Subcommand::CSV(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?
        }

        Subcommand::GenPass(opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )?,

        Subcommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
            Base64Subcommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
        },

        Subcommand::Text(subcmd) => match subcmd {
            TextSubcommand::Sign(opts) => match opts.format {
                TextSignFormat::Blake3 => process_sign(&opts.input, &opts.key, opts.format)?,
                TextSignFormat::Ed25519 => todo!(),
            },
            TextSubcommand::Verify(opts) => {
                println!("{:?}", opts);
            }
        },
    }

    Ok(())
}
