/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - mod.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 13, 2024 [4:27 AM]
//     ||  '-'
/* ************************************************************************** */

mod all;
mod base;

pub use all::Modules;
pub use base::*;

pub(crate) mod prettier;
pub use prettier::Prettier;