/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - mod.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 13, 2024 [7:06 AM]
//     ||  '-'
/* ************************************************************************** */

pub(crate) mod embed;

use crate::declare_module;
declare_module!(
    Prettier,
    description "Toolkit for generating various elements",
    docs "",
    sub_modules embed::EmbedCreator{}
);