/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - base.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 13, 2024 [4:32 AM]
//     ||  '-'
/* ************************************************************************** */

use serenity::all::{CreateCommand};

pub trait SubModuleBase
{
    fn name(&self) -> &'static str;
    fn docs(&self) -> Option<&'static str> { None }
    fn register_command(&self) -> Option<CreateCommand> { None }
    fn run_command(&self) -> Result<(), ()> { Ok(()) }
}

pub trait ModuleBase
{
    fn new() -> Box<Self> where Self: Sized;
    fn sub_modules(&self) -> &SubModulesArray;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn docs(&self) -> &'static str;
}

pub type SubModulesArray = Vec<Box<dyn SubModuleBase + Send + Sync>>;
pub type ModulesArray = Vec<Box<dyn ModuleBase + Send + Sync>>;

#[macro_export]
/// Intended for single use in modules' mod.rs file
macro_rules! declare_module
{
    ($name: ident,
        description $desc: literal,
        docs $docs: literal $(,)?)
    => {
        declare_module!($name, description $desc, docs $docs, sub_modules);
    };

    ($name: ident,
        description $desc: literal,
        docs $docs: literal,
        sub_modules $($sub_mods: expr),*)
    => {
        use $crate::modules::{ModuleBase, SubModulesArray};

        pub struct $name
        {
            sub_modules: SubModulesArray
        }

        impl ModuleBase for $name
        {
            fn new() -> Box<Self> where Self: Sized
                { Box::new(Self{ sub_modules: vec![$($sub_mods{})*] }) }

            fn sub_modules(&self) -> &SubModulesArray { &self.sub_modules }
            fn name(&self) -> &'static str { stringify!($name) }
            fn description(&self) -> &'static str { $desc }
            fn docs(&self) -> &'static str { $docs }
        }
    };
}