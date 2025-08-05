use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Decompile {
        #[arg(long)]
        input_bgf: std::path::PathBuf,
        #[arg(long)]
        output_dir: std::path::PathBuf,
        #[arg(long)]
        image_ext: String,
    },
    Compile {
        #[arg(long)]
        input_conf: std::path::PathBuf,
        #[arg(long)]
        output_bgf: std::path::PathBuf,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Decompile {
            input_bgf,
            output_dir,
            image_ext,
        } => decompile(&input_bgf, &output_dir, &image_ext)?,
        Commands::Compile {
            input_conf,
            output_bgf,
        } => todo!(),
    }

    Ok(())
}

fn decompile(
    input_bgf: &std::path::Path,
    output_dir: &std::path::Path,
    image_ext: &str,
) -> Result<()> {
    let bgf = bgftool::bgf::Bgf::read(std::fs::File::open(input_bgf)?)?;
    let name = input_bgf.file_stem().unwrap().to_string_lossy();
    let mut image_paths = Vec::with_capacity(bgf.bitmaps.len());

    for (index, bitmap) in bgf.bitmaps.iter().enumerate() {
        let output_path = output_dir.join(format!("{name}_{index:04}.{image_ext}"));
        bitmap.save_image(&output_path)?;
        image_paths.push(output_path);
    }

    let mut conf = bgftool::conf::Bgf::from(bgf);

    for (index, bitmap) in conf.bitmaps.iter_mut().enumerate() {
        bitmap.path = image_paths[index].strip_prefix(output_dir)?.to_path_buf();
    }

    let conf_path = output_dir.join(format!("{name}.json"));
    serde_json::to_writer_pretty(std::fs::File::create(&conf_path)?, &conf)?;

    Ok(())
}
