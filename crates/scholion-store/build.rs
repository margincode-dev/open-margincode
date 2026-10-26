fn main() {
    // sqlx::migrate! embeds the migrations dir; rebuild when files are added.
    println!("cargo:rerun-if-changed=migrations");
}
