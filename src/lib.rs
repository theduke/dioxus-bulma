mod cmp;
mod elem;
mod layout;

pub mod embed;

pub use self::{
    cmp::{
        breadcrumb::{Breadcrumb, BreadcrumbAlign, BreadcrumbItem, BreadcrumbSeparator},
        dropdown::{Dropdown, DropdownCloseCallback, DropdownDivider, DropdownRenderCallback},
        message::Message,
        modal::Modal,
    },
    elem::{
        block::Block,
        boxed::Box,
        content::Content,
        delete::Delete,
        icon::{FaIcon, Icon, IconText},
        notification::Notification,
        progress::Progress,
        table::{ScrollableTable, Table},
        tag::{TagA, TagButton, TagSpan, Tags},
        title::{Subtitle, Title, TitleSize},
    },
    layout::{
        container::{Container, ContainerSize},
        footer::Footer,
        hero::{Hero, HeroSize},
        level::{Level, LevelItem},
        media::Media,
        section::Section,
    },
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl Size {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Small => "is-small",
            Self::Normal => "is-normal",
            Self::Medium => "is-medium",
            Self::Large => "is-large",
        }
    }

    pub fn class_multiple(&self) -> &'static str {
        match self {
            Self::Small => "are-small",
            Self::Normal => "are-normal",
            Self::Medium => "are-medium",
            Self::Large => "are-large",
        }
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.class())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    Link,
    LinkLight,
    Primary,
    PrimaryLight,
    Info,
    InfoLight,
    Success,
    SuccessLight,
    Warning,
    WarningLight,
    Danger,
    DangerLight,
    Black,
    Dark,
    Light,
    White,
}

impl Color {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Link => "is-link",
            Self::LinkLight => "is-link is-light",
            Self::Primary => "is-primary",
            Self::PrimaryLight => "is-primary is-light",
            Self::Info => "is-info",
            Self::InfoLight => "is-info is-light",
            Self::Success => "is-success",
            Self::SuccessLight => "is-success is-light",
            Self::Warning => "is-warning",
            Self::WarningLight => "is-warning is-light",
            Self::Danger => "is-danger",
            Self::DangerLight => "is-danger is-light",
            Self::Black => "is-black",
            Self::Dark => "is-dark",
            Self::Light => "is-light",
            Self::White => "is-white",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            Self::Link => "has-text-link",
            Self::LinkLight => "has-text-link-light",
            Self::Primary => "has-text-primary",
            Self::PrimaryLight => "has-text-primary-light",
            Self::Info => "has-text-info",
            Self::InfoLight => "has-text-info-light",
            Self::Success => "has-text-success",
            Self::SuccessLight => "has-text-success-light",
            Self::Warning => "has-text-warning",
            Self::WarningLight => "has-text-warning-light",
            Self::Danger => "has-text-danger",
            Self::DangerLight => "has-text-danger-light",
            Self::Black => "has-text-black",
            Self::Dark => "has-text-dark",
            Self::Light => "has-text-light",
            Self::White => "has-text-white",
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.class())
    }
}
