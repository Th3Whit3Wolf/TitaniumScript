#![allow(unreachable_pub)]

xflags::xflags! {
    src "./src/flags.rs"

    /// Run custom build command.
    cmd xtask {
        cmd fuzz-tests {}
        cmd grammar {}
        cmd badges {}
        cmd ci {}
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Xtask {
    pub subcommand: XtaskCmd,
}

#[derive(Debug)]
pub enum XtaskCmd {
    Badges(Badges),
    Ci(Ci),
    Grammar(Grammar),
    FuzzTests(FuzzTests),
}
#[derive(Debug)]
pub struct Badges;

#[derive(Debug)]
pub struct Ci;

#[derive(Debug)]
pub struct Grammar;

#[derive(Debug)]
pub struct FuzzTests;

impl Xtask {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end
