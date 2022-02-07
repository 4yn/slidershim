use std::{
  env, fs,
  path::{Path, PathBuf},
};

const COPY_DIR: &'static str = "res";

fn copy_dir<P, Q>(from: P, to: Q)
where
  P: AsRef<Path>,
  Q: AsRef<Path>,
{
  // https://stackoverflow.com/a/68950006
  let to = to.as_ref().to_path_buf();

  match fs::read_dir(from) {
    Ok(paths) => {
      for path in paths {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
          fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
          if !to.exists() {
            fs::create_dir(&to).unwrap();
          }

          copy_dir(&path, to);
        } else { /* Skip other content */
        }
      }
    }
    Err(_) => {}
  }
}

fn main() {
  let out = env::var("PROFILE").unwrap();
  let out = PathBuf::from(format!("target/{}/{}", out, COPY_DIR));

  if out.exists() {
    fs::remove_dir_all(&out).unwrap()
  };
  fs::create_dir(&out).unwrap();
  copy_dir(COPY_DIR, &out);

  tauri_build::build();
}
