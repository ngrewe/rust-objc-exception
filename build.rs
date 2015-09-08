extern crate gcc;

fn main() {
    gcc::Config::new().flag("-fexceptions")
                      .flag("-fobjc-exceptions")
                      .file("extern/exception.m")
                      .compile("libexception.a");
}
