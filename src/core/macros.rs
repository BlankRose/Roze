/* ************************************************************************** */
//         .-.
//   __   /   \   __
//  (  `'.\   /.'`  )  Roze - macros.rs
//   '-._.(;;;)._.-'
//   .-'  ,`"`,  '-.
//  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
//      //\   /        Created at: May 09, 2024 [8:11 AM]
//     ||  '-'
/* ************************************************************************** */

/// Calls print!() when on debug mode
#[macro_export]
macro_rules! debug {
    ($($what: expr),*) => {
        #[cfg(debug_assertions)]
        print!($($what),*);
    };
}

/// Calls println!() when on debug mode
#[macro_export]
macro_rules! debug_ln {
    ($($what: expr),*) => {
        #[cfg(debug_assertions)]
        println!($($what),*);
    };
}

/// Executes given code when on debug mode
#[macro_export]
macro_rules! debug_run {
    ($exec: expr) => {
        #[cfg(debug_assertions)]
        $exec;
    };
}

/// Calls print!() when on release mode
#[macro_export]
macro_rules! release {
    ($($what: expr),*) => {
        #[cfg(not(debug_assertions))]
        print!($($what),*);
    };
}

/// Calls println!() when on release mode
#[macro_export]
macro_rules! release_ln {
    ($($what: expr),*) => {
        #[cfg(not(debug_assertions))]
        println!($($what),*);
    };
}

/// Executes given code when on release mode
#[macro_export]
macro_rules! release_run {
    ($exec: expr) => {
        #[cfg(not(debug_assertions))]
        $exec;
    };
}