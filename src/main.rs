use ldtk_rust::Project;


fn main() {
    let file_path = "assets/Typical_TopDown_example.ldtk".to_string();
    let ldtk = Project::new(file_path);
    println!("First level pixel height is {}!", ldtk.levels[0].px_hei);
    println!("First level pixel height is {}!", ldtk.levels[0].bg_color);
}
