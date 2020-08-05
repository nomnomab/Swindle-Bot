use serenity::{
    framework::{
        standard::macros::group,
    }
};

pub mod add;
pub mod rank;
pub mod about;

use add::*;
use rank::*;
use about::*;

#[group]
#[commands(add, profile, about)]
struct General;