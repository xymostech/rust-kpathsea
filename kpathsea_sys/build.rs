use pkg_config::find_library;

fn main() {
  if find_library("kpathsea").is_ok() {
    return;
  } else {
    if cfg!(kpathsea_docs_rs) { }
    else {
      panic!("Could not find kpathsea using pkg-config")
    }
  }
}
