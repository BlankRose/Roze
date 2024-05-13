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

mod modules;
mod module_base;

pub use modules::Modules;
pub use module_base::*;

pub(crate) mod prettier;
pub use prettier::Prettier;