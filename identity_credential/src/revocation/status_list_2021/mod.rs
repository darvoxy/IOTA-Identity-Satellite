//! Implementation of [StatusList2021](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/)

/// Implementation of [StatusList2021Credential](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021credential)
pub mod credential;
mod entry;
mod status_list;

pub use status_list::StatusList2021;
