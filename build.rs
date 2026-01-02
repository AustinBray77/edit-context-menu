use std::{
    env::{self},
    fs::remove_dir_all,
    path::{Path, PathBuf},
};

use copy_dir;

fn main() {
    let resource_path = get_output_path().join("resources");

    if resource_path.exists() {
        remove_dir_all(&resource_path).unwrap();
    }

    copy_dir::copy_dir("./resources", resource_path).unwrap();
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

/*
fn build_view(view_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    Command::new("npm.cmd")
        .arg("run")
        .arg("build")
        .current_dir(view_dir)
        .output()?;

    Ok(())
}

fn compress_view(view_dir: &PathBuf) -> Result<String, Box<dyn Error>> {
    use std::fs::read_to_string;

    let dist_dir = view_dir.join("dist");

    let mut html = read_to_string(dist_dir.join("index.html"))?;
    let mut result = String::new();

    while let Some((first, second)) = html.split_once("<script type=\"module\" crossorigin src=\"")
    {
        if let Some((name, next)) = second.split_once("\"></script>") {
            let js_path = name
                .split("/")
                .fold(dist_dir.clone(), |path, next| path.join(next));

            println!("{}", js_path.to_str().unwrap());

            let javascript = read_to_string(js_path)?;

            result += first;
            result += "<script type=\"module\" crossorigin >";
            result += remove_js_comments(javascript)?.as_str();
            result += "</script>";
            html = next.to_string();
        } else {
            return Err("<script> must have an accompanying </script>".into());
        }
    }

    result += html.as_str();

    Ok(result)
}

fn copy_to_outdir(compressed_html: String) -> Result<(), Box<dyn Error>> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = out_dir + "\\VIEW";

    fs::write(dest_path, compressed_html)?;

    Ok(())
}

fn remove_js_comments(script: String) -> Result<String, Box<dyn Error>> {
    let mut cur = script;
    let mut result = String::new();

    while let Some((before, middle)) = cur.split_once("/*") {
        if let Some((_, after)) = middle.split_once("*/
") {
            result += before;
            cur = after.to_string();
        } else {
return Err(" /* should have a matching */
".into());
        }
    }

    result += cur.as_str();

    Ok(result)
}
*/
