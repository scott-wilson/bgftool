use clap::Parser;
use color_eyre::eyre::Result;
use rayon::prelude::*;

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
        } => compile(&input_conf, &output_bgf)?,
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

fn compile(input_conf: &std::path::Path, output_bgf: &std::path::Path) -> Result<()> {
    let input_conf = input_conf.canonicalize()?;
    let input_conf_dir = input_conf.parent().unwrap();
    let conf: bgftool::conf::Bgf = serde_json::from_reader(std::fs::File::open(&input_conf)?)?;

    let bitmap_results = conf
        .bitmaps
        .into_par_iter()
        .map(|bitmap_conf| -> Result<bgftool::bgf::Bitmap> {
            let bitmap_path = input_conf_dir.join(bitmap_conf.path);
            let options = bgftool::bgf::BitmapImageOptions {
                compression: bitmap_conf.compression,
                transparency_clip: 0.5,
            };
            let mut bitmap = bgftool::bgf::Bitmap::from_image(bitmap_path, &options)?;
            bitmap.offset = bitmap_conf.offset;
            bitmap.hotspots = bitmap_conf
                .hotspots
                .into_iter()
                .map(|h| bgftool::bgf::Hotspot {
                    number: h.number,
                    position: bgftool::bgf::Point(h.position.0, h.position.1),
                })
                .collect();

            Ok(bitmap)
        })
        .collect::<Vec<_>>();
    let mut bitmaps = Vec::with_capacity(bitmap_results.len());

    for bitmap_result in bitmap_results {
        bitmaps.push(bitmap_result?);
    }

    let bgf = bgftool::bgf::Bgf {
        version: conf.version,
        name: conf.name,
        bitmaps: bitmaps,
        index_groups: conf
            .index_groups
            .into_iter()
            .map(|g| bgftool::bgf::Group { indices: g.indices })
            .collect(),
        max_indices: conf.max_indices,
        shrink_factor: conf.shrink_factor,
    };
    bgf.write(std::fs::File::create(output_bgf)?)?;

    Ok(())
}
