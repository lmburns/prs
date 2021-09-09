pub mod add;
#[cfg(feature = "alias")]
pub mod alias;
pub mod clone;
#[cfg(feature = "clipboard")]
pub mod copy;
pub mod duplicate;
pub mod edit;
pub mod generate;
pub mod git;
pub mod housekeeping;
pub mod init;
pub mod internal;
pub mod list;
pub mod r#move;
pub mod otp;
pub mod recipients;
pub mod remove;
pub mod show;
pub mod sync;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub mod tomb;

// Re-export to cmd module
#[cfg(feature = "alias")]
pub use self::alias::CmdAlias;
#[cfg(feature = "clipboard")]
pub use self::copy::CmdCopy;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub use self::tomb::CmdTomb;
#[rustfmt::skip]
pub use self::{
    add::CmdAdd,
    clone::CmdClone,
    duplicate::CmdDuplicate,
    edit::CmdEdit,
    generate::CmdGenerate,
    git::CmdGit,
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
