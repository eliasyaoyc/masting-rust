fn main() {
    cc::Build::new().file("mystrlen.c")
        .static_flag(true)
        .compile("mystrlen");
}