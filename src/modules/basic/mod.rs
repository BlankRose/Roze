/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - mod.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 19, 2024 [7:01 AM]
//     ||  '-'
/* ************************************************************************** */

pub(crate) mod ping;

use crate::declare_module;
declare_module!(
    Basic,
    description "All basic commands, which are always enable per default",
    docs "",
    sub_modules ping::PingShard{}
);