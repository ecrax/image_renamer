use std::{env::args, fs};

fn main() -> color_eyre::Result<()> {
    let mut args = args();

    if args.len() < 2 {
        println!("USAGE: image_renamer <folder> [yes]");
        return Ok(());
    }

    let folder = args.nth(1).unwrap();
    let confirm = args.next().unwrap_or("no".to_string());
    println!("confirm: {}", confirm);
    println!("folder: {}", folder);

    let paths = std::fs::read_dir(folder)?;
    for path in paths {
        let path = path.expect("path not found").path();
        if path
            .extension()
            .unwrap()
            .to_ascii_lowercase()
            .to_str()
            .unwrap()
            != "jpg"
        {
            continue;
        }
        let file = std::fs::File::open(&path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        // for f in exif.fields() {
        //     println!("{} {} {}",
        //              f.tag, f.ifd_num, f.display_value().with_unit(&exif));
        // }

        let date = exif
            .get_field(exif::Tag::DateTime, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .with_unit(&exif);
        let camera = exif
            .get_field(exif::Tag::Model, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .with_unit(&exif);
        let aperture = exif
            .get_field(exif::Tag::FNumber, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .with_unit(&exif);
        let focal_length = exif
            .get_field(exif::Tag::FocalLength, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .with_unit(&exif);
        let iso = exif
            .get_field(exif::Tag::PhotographicSensitivity, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .with_unit(&exif);
        let shutter_speed = exif
            .get_field(exif::Tag::ExposureTime, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .with_unit(&exif);
        let mode = exif
            .get_field(exif::Tag::ExposureProgram, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .with_unit(&exif);

        let white_balance = exif
            .get_field(exif::Tag::WhiteBalance, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .with_unit(&exif);

        //TODO: make this configurable

        let file_name = format!(
            "{}__{}_{}_{}_{}_ISO{}_{}_{}.jpg",
            date.to_string().replace(" ", "_"),
            mode.to_string().replace(" ", "-"),
            aperture.to_string().replace("/", ""),
            focal_length.to_string().replace(" ", ""),
            shutter_speed.to_string().replace(" ", "").replace("/", "T"),
            iso,
            white_balance.to_string().replace(" ", "-"),
            camera.to_string().replace('"', "").replace(' ', "-")
        );
        let file_path = path.parent().unwrap().join(&file_name);

        if confirm == "yes" {
            fs::rename(&path, &file_path).expect(
                format!(
                    "cannot rename file {} -> {}",
                    path.display(),
                    file_path.as_path().display()
                )
                .as_str(),
            );
            println!("renaming: {} -> {file_name}", path.display());
        } else {
            println!("{} -> {file_name}", path.display());
        }
    }

    Ok(())
}
