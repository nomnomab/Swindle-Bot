use serenity::{
    framework::{
        standard::macros::group,
    }
};

pub mod add;
pub mod profile;
pub mod about;

use add::*;
use profile::*;
use about::*;

#[group]
#[commands(add, profile, about)]
struct General;