use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use glob::glob;

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about = "Removes the hash from the front of Prelude files and optionally adds a new prefix"
)]
struct Args {
    #[arg(help = "Path to the directory that contains the files to rename")]
    directory: PathBuf,

    #[arg(short, long, help = "The length of the prefix to replace")]
    prefix_length: Option<usize>,

    #[arg(short, long, help = "Recursively traverse directories")]
    recursive: bool,

    #[arg(
        short,
        long,
        help = "Don't append the parent directory name to the file name"
    )]
    no_append: bool,

    #[arg(short, long, help = "Verbose output")]
    verbose: bool,
}

fn renamer(from_file: &Path, to_file: &Path, verbose: &bool) {
    fs::rename(from_file, to_file).unwrap();

    if *verbose {
        println!(
            "file {} renamed to {}",
            from_file.display(),
            to_file.display()
        );
    }
}

fn recursive_gather_dirs(dir_path: &Path) -> Vec<PathBuf> {
    let mut paths = vec![dir_path.to_path_buf()];
    let root = dir_path.join(Path::new("**")).display().to_string();
    for entry in glob(&root).expect("Root directory not found") {
        match entry {
            Ok(path) => paths.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    paths
}

fn rename_xml(dirs: Vec<PathBuf>, prefix_length: Option<usize>, append_dir: bool, verbose: bool) {
    let prefix = prefix_length.unwrap_or(32);

    for dir in dirs {
        let mut d = dir.to_string_lossy().to_string();
        d.push_str("/*.xml");
        for entry in glob(&d).expect("Failed to read file path") {
            let path = entry.expect("Path error");
            let parent = path.parent();
            let file_name = &path
                .file_name()
                .unwrap_or_else(|| OsStr::new(""))
                .to_string_lossy()
                .to_string();
            let rename_file = &file_name[prefix..file_name.len()];
            if append_dir {
                let mut parent_dir = parent
                    .unwrap()
                    .file_name()
                    .unwrap_or_else(|| OsStr::new(""))
                    .to_string_lossy()
                    .to_string();
                parent_dir.push_str(rename_file);
                let rename = &parent.unwrap_or_else(|| Path::new(".")).join(parent_dir);
                renamer(&path, rename, &verbose);
            } else {
                let rename = &parent.unwrap_or_else(|| Path::new(".")).join(rename_file);
                renamer(&path, rename, &verbose);
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let dirs = if args.recursive {
        recursive_gather_dirs(&args.directory)
    } else {
        vec![args.directory]
    };

    let append_dir = !args.no_append;

    rename_xml(dirs, args.prefix_length, append_dir, args.verbose)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_renamer() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tmp_dir = tempdir().unwrap();
        let file_name = String::from("C6A91478B2AF28F550DD3B128D5D2886_test_1.xml");
        let from_file = &tmp_dir.path().join(&file_name);
        let to_file = &tmp_dir.path().join("test_1.xml");

        fs::copy(root.join("tests/assets/sites/").join(file_name), from_file).unwrap();

        renamer(from_file, to_file, &false);

        assert!(!from_file.is_file());
        assert!(to_file.is_file());
    }

    #[test]
    fn test_renamer_verbose() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tmp_dir = tempdir().unwrap();
        let file_name = String::from("C6A91478B2AF28F550DD3B128D5D2886_test_1.xml");
        let from_file = &tmp_dir.path().join(&file_name);
        let to_file = &tmp_dir.path().join("test_1.xml");

        fs::copy(root.join("tests/assets/sites/").join(file_name), from_file).unwrap();

        renamer(from_file, to_file, &true);

        assert!(!from_file.is_file());
        assert!(to_file.is_file());
    }

    #[test]
    fn test_recursive_gather_dirs() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dirs = recursive_gather_dirs(&root.join("tests/assets"));
        let expected_dirs = vec![
            root.join("tests/assets"),
            root.join("tests/assets/sites"),
            root.join("tests/assets/subjects"),
        ];
        assert_eq!(&dirs, &expected_dirs);
    }

    #[test]
    fn test_rename_xml() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tmp_dir = tempdir().unwrap();
        let file_name = String::from("C6A91478B2AF28F550DD3B128D5D2886_test_1.xml");
        let site_dir = &tmp_dir.path().join("sites");
        let from_file = &site_dir.join(&file_name);
        let to_file = &site_dir.join("sites_test_1.xml");

        fs::create_dir(site_dir).unwrap();
        fs::copy(root.join("tests/assets/sites/").join(&file_name), from_file).unwrap();

        rename_xml(vec![site_dir.to_path_buf()], None, true, false);

        assert!(!from_file.is_file());
        assert!(to_file.is_file());
    }
}
