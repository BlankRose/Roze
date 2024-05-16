/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - localization.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 16, 2024 [10:34 AM]
//     ||  '-'
/* ************************************************************************** */

use std::collections::HashMap;
use std::fs::{read, read_dir};
use lazy_static::lazy_static;
use log::warn;

type SingleLocaleMap = HashMap<String, String>;
type LocaleMap = HashMap<String, SingleLocaleMap>;

pub struct Localization
{
    locales: LocaleMap
}

impl Localization
{
    pub fn new(dir_path: &'static str) -> Self
    {
        println!("Loading localizations..");
        let paths = match read_dir(dir_path) {
            Ok(paths) => paths,
            Err(error) => panic!("Failed to read directory {}: {}", dir_path, error)
        };

        let mut locales = LocaleMap::new();
        for entry in paths
        {
            let entry = entry.unwrap().path();

            let name = entry.as_path().file_stem().unwrap();
            let content = match read(entry.clone()) {
                Ok(content) => String::from_utf8(content).unwrap(),
                Err(error) => {
                    warn!("Failed to read file {}: {}", entry.as_path().to_str().unwrap(), error);
                    continue;
                }
            };

            let mut sub_locale = SingleLocaleMap::new();
            for line in content.lines()
            {
            }

            locales.insert(name.to_str().unwrap().to_string(), sub_locale);
        }

        println!("Localizations successfully loaded!");
        return Self{locales};
    }

    pub fn get(&self, key: &'static str, locale: &'static str) -> String
    {
        return key.to_string();
    }
}

lazy_static!(
    pub static ref LOCALES: Localization
        = Localization::new("locales");
);