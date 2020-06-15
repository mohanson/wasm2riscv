#[derive(Clone, Debug)]
pub enum Platform {
    PosixX8664,
    PosixX8664Spectest,
    Unknown,
}

// A Config specifies the global config for a build.
#[derive(Clone, Debug)]
pub struct Config {
    // Path of cc, usually the result of "$ which gcc".
    pub binary_cc: String,
    pub binary_wavm: String,
    // Platfrom flag and their files.
    pub platform: Platform,
    pub platform_posix_x86_64: &'static str,
    pub platform_posix_x86_64_runtime: &'static str,
    pub platform_posix_x86_64_spectest: &'static str,
    pub platform_posix_x86_64_spectest_runtime: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            binary_cc: String::from("gcc"),
            binary_wavm: String::from("wavm"),
            platform: Platform::Unknown,
            platform_posix_x86_64: include_str!("./platform/posix_x86_64.h"),
            platform_posix_x86_64_runtime: include_str!("./platform/posix_x86_64_runtime.S"),
            platform_posix_x86_64_spectest: include_str!("./platform/posix_x86_64_spectest.h"),
            platform_posix_x86_64_spectest_runtime: include_str!("./platform/posix_x86_64_spectest_runtime.S"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Middle {
    // Config is the global config for a build.
    pub config: Config,
    // CurrentDir is the caller's working directory, or the empty string to use
    // the current directory of the running process.
    pub current_dir: std::path::PathBuf,
    // Source wasm/wast file.
    pub file: std::path::PathBuf,
    // File stem is the source wasm/wast file's name without extension.
    // Example: file_stem(helloworld.wasm) => helloworld
    pub file_stem: String,
    // Folder for collect platform based code.
    pub platform_code_path: std::path::PathBuf,
    // Project address, usually equals to file.dirname
    pub prog_dir: std::path::PathBuf,
    // Precompiled wasm file built by wavm.
    pub wavm_precompiled_wasm: std::path::PathBuf,

    pub aot_object: std::path::PathBuf,
    // The middle AOT glue file generated by aot_generator
    pub aot_glue: std::path::PathBuf,
    // Path of dummy file.
    pub dummy: std::path::PathBuf,
    pub misc_has_init: bool,
}

impl Middle {
    // Set global config for middle.
    pub fn init_config(&mut self, config: Config) {
        self.config = config;
    }

    // Initialize the compilation environment.
    pub fn init_file<P: AsRef<std::path::Path>>(&mut self, p: P) {
        self.current_dir = std::env::current_dir().unwrap();
        self.file = p.as_ref().to_path_buf();
        self.file_stem = self.file.file_stem().unwrap().to_str().unwrap().to_string();
        self.prog_dir = self.file.parent().unwrap().to_path_buf();
        if self.prog_dir.parent() == None {
            self.prog_dir = std::path::PathBuf::from("./");
        }
        self.platform_code_path = self.prog_dir.join(self.file_stem.clone() + "_platform");
        self.wavm_precompiled_wasm = self.prog_dir.join(self.file_stem.clone() + "_precompiled.wasm");
    }
}
