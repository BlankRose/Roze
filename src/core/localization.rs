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
    locales: LocaleMap,
    pub default: String,
}

impl Localization
{
    /// Generates a new localization structure for later usage
    /// by opening the directory located at `dir_path`, opening all files,
    /// parsing and loading the entries found into memory, where the files'
    /// name becomes the localization's name. Meanwhile, `default` defines
    /// where to find an alternative string if the requested localization is missing
    /// or the string is not translated yet
    pub fn new(dir_path: &'static str, default: &'static str) -> Self
    {
        let paths = match read_dir(dir_path) {
            Ok(paths) => paths,
            Err(error) => panic!("Failed to read directory {}: {}", dir_path, error)
        };

        let default = default.to_string();
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
                if let Some((key, value)) = line.split_once('=')
                {
                    sub_locale.insert(key.trim().to_string(), value.trim().to_string());
                }
            }

            sub_locale.shrink_to_fit();
            locales.insert(name.to_str().unwrap().to_string(), sub_locale);
        }

        locales.shrink_to_fit();
        return Self{locales, default};
    }

    /// Retrieves the locale string of `key`, for a given `locale`
    /// or the `key` itself if it couldn't be found
    pub fn get<K, L>(&self, key: K, locale: L) -> String
        where K: Into<String>, L: Into<String>
    {
        let key = key.into();
        let locale = locale.into();

        if let Some(sub_locale) = self.locales.get(&locale)
        {
            if let Some(result) = sub_locale.get(&key)
                { return result.to_owned(); }
            else if locale != self.default
                { return self.get(key, &self.default); }
        }
        return key;
    }

    /// Retrieves the locale string of `key`, for the `default` locale
    /// or the `key` itself if it couldn't be found
    pub fn get_default<K>(&self, key: K) -> String
        where K: Into<String>
    {
        return self.get(key, &self.default);
    }

    /// Retrieves the locale string of `key`, for a given `locale`
    /// and format it with the provided `params` in place of `$n`
    /// or the `key` itself if it couldn't be found
    ///
    /// Note: `$$` becomes a `$`
    pub fn getf<K, L>(&self, key: K, locale: L, params: Vec<String>) -> String
        where K: Into<String>, L: Into<String>
    {
        let mut result = self.get(key, locale);
        for (i, param) in params.iter().enumerate()
        {
            result = result.replace(format!("${}", i).as_str(), param);
        }
        result = result.replace("$$", "$");
        return result;
    }

    /// Retrieves the locale string of `key`, for the `default` locale
    /// and format it with the provided `params` in place of `$n`
    /// or the `key` itself if it couldn't be found
    ///
    /// Note: `$$` becomes a `$`
    pub fn getf_default<K>(&self, key: K, params: Vec<String>) -> String
        where K: Into<String>
    {
        return self.getf(key, &self.default, params);
    }

    /// Retrieve the loaded and available localizations
    pub fn get_available(&self) -> Vec<&String>
    {
        return self.locales.keys().collect();
    }

    /// Checks if the provided `default` localization has been
    /// successfully loaded and is available
    pub fn has_default(&self) -> bool
    {
        return self.locales.contains_key(&self.default);
    }

    /// Dumps the stored localizations, keys and values
    /// on the standard output (for debug purposes)
    #[allow(dead_code)]
    pub fn debug(&self)
    {
        for (locale, store) in self.locales.iter()
        {
            println!("LANG: [{}]", locale);
            for (key, value) in store.iter()
            {
                println!(" -- {} = {}", key, value);
            }
        }
    }
}

lazy_static!(
    pub static ref LOCALES: Localization
        = Localization::new("locales", "en-US.lang");
);

#[macro_export]
/// Utility macro which retrieve the translated string for
/// the given `key`, and given `locale` if provided
/// (otherwise, uses `default`).
///
/// Additional `args` may be passed for formating the strings
macro_rules! get_locale {
    ($key: expr) => {
        LOCALES.get_default($key)
    };
    ($key: expr, locale = $locale: expr) => {
        LOCALES.get($key, $locale)
    };
    ($key: expr, args = $($args: expr), *) => {
        LOCALES.getf_default($key, vec![$($args)*])
    };
    ($key: expr, locale = $locale: expr, args = $($args: expr), *) => {
        LOCALES.getf($key, $locale, vec![$($args)*])
    };
    ($key: expr, args = $($args: expr), *, locale = $locale: expr) => {
        LOCALES.getf($key, $locale, vec![$($args)*])
    };
}

#[macro_export]
/// Utility macro which retrieves all the available locales as an array of string,
/// ready to be passed down to `get` functions
macro_rules! get_locales {
    () => {
        LOCALES.get_available()
    };
}