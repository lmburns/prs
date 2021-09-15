pub(crate) mod add;
#[cfg(feature = "alias")]
pub(crate) mod alias;
pub(crate) mod clone;
#[cfg(feature = "clipboard")]
pub(crate) mod copy;
pub(crate) mod duplicate;
pub(crate) mod edit;
pub(crate) mod generate;
pub(crate) mod git;
pub(crate) mod grep;
pub(crate) mod housekeeping;
pub(crate) mod init;
pub(crate) mod internal;
pub(crate) mod list;
pub(crate) mod r#move;
pub(crate) mod otp;
pub(crate) mod recipients;
pub(crate) mod remove;
pub(crate) mod show;
pub(crate) mod sync;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub mod tomb;

// Re-export to cmd module
#[cfg(feature = "alias")]
pub(crate) use self::alias::CmdAlias;
#[cfg(feature = "clipboard")]
pub(crate) use self::copy::CmdCopy;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub use self::tomb::CmdTomb;
#[rustfmt::skip]
pub(crate) use self::{
    add::CmdAdd,
    clone::CmdClone,
    duplicate::CmdDuplicate,
    edit::CmdEdit,
    generate::CmdGenerate,
    git::CmdGit,
    grep::CmdGrep,
    housekeeping::CmdHousekeeping,
    init::CmdInit,
    internal::CmdInternal,
    list::CmdList,
    otp::CmdOtp,
    r#move::CmdMove,
    recipients::CmdRecipients,
    remove::CmdRemove,
    show::CmdShow,
    sync::CmdSync,
};
