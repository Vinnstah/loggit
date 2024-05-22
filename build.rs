fn main() {
    uniffi::generate_scaffolding("src/loggit.udl")
        .expect("Build script panics can be ignored");
}