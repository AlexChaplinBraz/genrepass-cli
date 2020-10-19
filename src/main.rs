use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg};
use copypasta_ext::{prelude::*, x11_fork::ClipboardContext};
use genrepass::*;
use std::error::Error;
use std::process::exit;
use std::str::FromStr;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}.", e);
        exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let default_config = PassConfig::new();
    let pass_amount = default_config.pass_amount.to_string();
    let max_resets = default_config.reset_amount.to_string();

    let capitalise_short = "Uppercase the first character of every word";
    let capitalise_long = "Uppercase the first character of every word. Makes the password much easier to read, but also slightly less secure due to the predictability of having capitalised words. Still, the highly improved readability makes it worth it to always have it on.";
    let clipboard_short = "Copy the generated password/s to clipboard instead of writing to stdout";
    let clipboard_long = "Copy the generated password/s to clipboard instead of writing to stdout.";
    let dont_lower_short = "Don't lowercase at all to keep original casing";
    let dont_lower_long = "Don't lowercase at all to keep original casing. Ignores `--force-lower`, both manual and automatic.";
    let dont_upper_short = "Don't uppercase at all to keep original casing";
    let dont_upper_long = "Don't uppercase at all to keep original casing. Ignores `--force-upper`, both manual and automatic.";
    let force_lower_short = "Force the specified amount of lowercase characters";
    let force_lower_long = "Force the specified amount of lowercase characters. Gets ignored if `--dont-lower` is also set.";
    let force_upper_short = "Force the specified amount of uppercase characters";
    let force_upper_long = "Force the specified amount of uppercase characters. Gets ignored if `--dont-upper` is also set.";
    let keep_nums_short = "Choose to keep numbers from the source in the password";
    let keep_nums_long = "Choose to keep numbers from the source in the password. It will treat blocks of numbers as words, not counting them towards the amount of numbers to insert into the password.";
    let randomise_short = "Shuffle the words";
    let randomise_long = "Shuffle the words. Useful if the source text is just a list of words without order anyway and you want to have a different order with each run of the program.";
    let replace_short = "Replace the original characters";
    let replace_long = "Replace the original characters. Instead of inserting the numbers and special characters (which preserves the original words), replace the characters at random positions.";
    let length_short = "Set the length of the password";
    let length_long = "Set the length of the password. Can either be a range like 24-30, which will generate a password between that length, or it can be an exact number like 25 for a password of that exact length.";
    let lower_short = "Amount of lowercase characters";
    let lower_long = "Amount of lowercase characters. Can take either a range like 2-4 or an exact amount like 2. If there are no lowercase characters, the `--force-lower` flag is turned on automatically to decapitalise up to the specified amount of alphabetic characters. But if there's at least one lowercase character there won't be any decapitalisation unless `--force-lower` is turned on manually.";
    let upper_short = "Amount of uppercase characters";
    let upper_long = "Amount of uppercase characters. Can take either a range like 2-4 or an exact amount like 2. If there are no uppercase characters, the `--force-upper` flag is turned on automatically to capitalise up to the specified amount of alphabetic characters. But if there's at least one uppercase character there won't be any capitalisation unless `--force-upper` is turned on manually.";
    let resets_short = "Amount of times to try generating before truncating";
    let resets_long = "Amount of times to try generating password before truncating. If the range is too small or an exact number, it'll be harder to get a fitting set of words, so the word selection will restart if the password exceeds the maximum length. But since it would keep looping if it doesn't find the right length it needs a way to stop, which in this case is simply truncating the password to the maximum length.";
    let num_short = "Amount of numbers to insert";
    let num_long = "Amount of numbers to insert. Can take either a range like 2-4 or an exact amount like 2. Doesn't take into consideration the amount of numbers already in the password if `--keep-nums` is activated.";
    let special_short = "Amount of special characters to insert";
    let special_long = "Amount of special characters to insert. Can take either a range like 2-4 or an exact amount like 2.";
    let special_chars_short = "The special characters to insert";
    let special_chars_long =
        "The special characters to insert. Non-ASCII characters are not supported.";
    let pass_amount_short = "Amount of passwords to generate";
    let pass_amount_long = "Amount of passwords to generate. Each password comes on a new line. Useful for providing a list of passwords to choose from.";
    let path_short = "Path/s to text file or directory with text files to source words from";
    let path_long = "One or more paths to text file or directory with text files to source words from.

In case of a directory, it recursively parses every file inside it while ignoring non-plaintext files and following links.

Accepts UTF-8 characters, but translates them to ASCII for use in the password. So if a word in another language is encountered, it will be transformed into a kind of phonetic spelling in ASCII, and if an emoji is encountered, it will be translated into its meaning, for example, :D would become 'grinning'.";

    let args = App::new("genrepass")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .help_message("Print help and exit")
        .version_message("Print version and exit")
        .version_short("v")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .required(true)
                .multiple(true)
                .help(path_short)
                .long_help(path_long),
        )
        .arg(
            Arg::with_name("capitalise")
                .short("C")
                .long("capitalise")
                .help(capitalise_short)
                .long_help(capitalise_long),
        )
        .arg(
            Arg::with_name("clipboard")
                .short("c")
                .long("clipboard")
                .help(clipboard_short)
                .long_help(clipboard_long),
        )
        .arg(
            Arg::with_name("dont_lower")
                .short("d")
                .long("dont-lower")
                .help(dont_lower_short)
                .long_help(dont_lower_long),
        )
        .arg(
            Arg::with_name("dont_upper")
                .short("D")
                .long("dont-upper")
                .help(dont_upper_short)
                .long_help(dont_upper_long),
        )
        .arg(
            Arg::with_name("force_lower")
                .short("f")
                .long("force-lower")
                .help(force_lower_short)
                .long_help(force_lower_long),
        )
        .arg(
            Arg::with_name("force_upper")
                .short("F")
                .long("force-upper")
                .help(force_upper_short)
                .long_help(force_upper_long),
        )
        .arg(
            Arg::with_name("keep_numbers")
                .short("k")
                .long("keep-numbers")
                .help(keep_nums_short)
                .long_help(keep_nums_long),
        )
        .arg(
            Arg::with_name("randomise")
                .short("X")
                .long("randomise")
                .help(randomise_short)
                .long_help(randomise_long),
        )
        .arg(
            Arg::with_name("replace")
                .short("r")
                .long("replace")
                .help(replace_short)
                .long_help(replace_long),
        )
        .arg(
            Arg::with_name("length")
                .takes_value(true)
                .default_value(&default_config.length)
                .allow_hyphen_values(true)
                .value_name("RANGE")
                .short("L")
                .long("length")
                .help(length_short)
                .long_help(length_long),
        )
        .arg(
            Arg::with_name("reset_amount")
                .takes_value(true)
                .default_value(&max_resets)
                .value_name("INTEGER")
                .short("R")
                .long("reset-amount")
                .help(resets_short)
                .long_help(resets_long),
        )
        .arg(
            Arg::with_name("lower_amount")
                .takes_value(true)
                .default_value(&default_config.lower_amount)
                .allow_hyphen_values(true)
                .value_name("RANGE")
                .short("l")
                .long("lower-amount")
                .help(lower_short)
                .long_help(lower_long),
        )
        .arg(
            Arg::with_name("upper_amount")
                .takes_value(true)
                .default_value(&default_config.upper_amount)
                .allow_hyphen_values(true)
                .value_name("RANGE")
                .short("u")
                .long("upper-amount")
                .help(upper_short)
                .long_help(upper_long),
        )
        .arg(
            Arg::with_name("number_amount")
                .takes_value(true)
                .default_value(&default_config.number_amount)
                .allow_hyphen_values(true)
                .value_name("RANGE")
                .short("n")
                .long("number-amount")
                .help(num_short)
                .long_help(num_long),
        )
        .arg(
            Arg::with_name("special_chars_amount")
                .takes_value(true)
                .default_value(&default_config.special_chars_amount)
                .allow_hyphen_values(true)
                .value_name("RANGE")
                .short("s")
                .long("special-chars-amount")
                .help(special_short)
                .long_help(special_long),
        )
        .arg(
            Arg::with_name("special_chars")
                .takes_value(true)
                .default_value(&default_config.special_chars)
                .value_name("CHARS")
                .short("S")
                .long("special-chars")
                .help(special_chars_short)
                .long_help(special_chars_long),
        )
        .arg(
            Arg::with_name("pass_amount")
                .takes_value(true)
                .default_value(&pass_amount)
                .value_name("INTEGER")
                .short("p")
                .long("pass-amount")
                .help(pass_amount_short)
                .long_help(pass_amount_long),
        )
        .get_matches();

    let mut user_config = PassConfig::new();

    user_config.capitalise = args.is_present("capitalise");
    user_config.dont_lower = args.is_present("dont_lower");
    user_config.dont_upper = args.is_present("dont_upper");
    user_config.force_lower = args.is_present("force_lower");
    user_config.force_upper = args.is_present("force_upper");
    user_config.keep_numbers = args.is_present("keep_numbers");
    user_config.length = args.value_of("length").unwrap().to_string();
    user_config.lower_amount = args.value_of("lower_amount").unwrap().to_string();
    user_config.reset_amount = {
        if args.is_present("reset_amount") {
            let max_resets = args.value_of("reset_amount").unwrap();
            usize::from_str(max_resets)?
        } else {
            usize::from_str(&max_resets)?
        }
    };
    user_config.number_amount = args.value_of("number_amount").unwrap().to_string();
    user_config.pass_amount = {
        if args.is_present("pass_amount") {
            let pass_amount = args.value_of("pass_amount").unwrap();
            usize::from_str(pass_amount)?
        } else {
            usize::from_str(&pass_amount)?
        }
    };
    user_config.randomise = args.is_present("randomise");
    user_config.replace = args.is_present("replace");
    user_config.special_chars_amount = args.value_of("special_chars_amount").unwrap().to_string();
    user_config.special_chars = args.value_of("special_chars").unwrap().to_string();
    user_config.upper_amount = args.value_of("upper_amount").unwrap().to_string();

    for x in args.values_of("path").unwrap() {
        user_config.get_words_from_path(x)?;
    }

    let validated = user_config.validate()?;

    let passwords = validated.generate().join("\n");

    if args.is_present("clipboard") {
        ClipboardContext::new()
            .and_then(|mut ctx| ctx.set_contents(passwords.into()))
            .map_err(|e| -> Box<dyn Error> { e })?;
    } else {
        println!("{}", passwords);
    }

    Ok(())
}
