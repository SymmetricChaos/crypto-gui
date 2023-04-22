use itertools::Itertools;

use crate::codes::Code;
use crate::errors::Error;

use super::{BinaryToText, BinaryToTextMode};

const PGP_WORDS: [[&'static str; 2]; 256] = [
    ["aardvark", "adroitness"],
    ["absurd", "adviser"],
    ["accrue", "aftermath"],
    ["acme", "aggregate"],
    ["adrift", "alkali"],
    ["adult", "almighty"],
    ["afflict", "amulet"],
    ["ahead", "amusement"],
    ["aimless", "antenna"],
    ["Algol", "applicant"],
    ["allow", "Apollo"],
    ["alone", "armistice"],
    ["ammo", "article"],
    ["ancient", "asteroid"],
    ["apple", "Atlantic"],
    ["artist", "atmosphere"],
    ["assume", "autopsy"],
    ["Athens", "Babylon"],
    ["atlas", "backwater"],
    ["Aztec", "barbecue"],
    ["baboon", "belowground"],
    ["backfield", "bifocals"],
    ["backward", "bodyguard"],
    ["banjo", "bookseller"],
    ["beaming", "borderline"],
    ["bedlamp", "bottomless"],
    ["beehive", "Bradbury"],
    ["beeswax", "bravado"],
    ["befriend", "Brazilian"],
    ["Belfast", "breakaway"],
    ["berserk", "Burlington"],
    ["billiard", "businessman"],
    ["bison", "butterfat"],
    ["blackjack", "Camelot"],
    ["blockade", "candidate"],
    ["blowtorch", "cannonball"],
    ["bluebird", "Capricorn"],
    ["bombast", "caravan"],
    ["bookshelf", "caretaker"],
    ["brackish", "celebrate"],
    ["breadline", "cellulose"],
    ["breakup", "certify"],
    ["brickyard", "chambermaid"],
    ["briefcase", "Cherokee"],
    ["Burbank", "Chicago"],
    ["button", "clergyman"],
    ["buzzard", "coherence"],
    ["cement", "combustion"],
    ["chairlift", "commando"],
    ["chatter", "company"],
    ["checkup", "component"],
    ["chisel", "concurrent"],
    ["choking", "confidence"],
    ["chopper", "conformist"],
    ["Christmas", "congregate"],
    ["clamshell", "consensus"],
    ["classic", "consulting"],
    ["classroom", "corporate"],
    ["cleanup", "corrosion"],
    ["clockwork", "councilman"],
    ["cobra", "crossover"],
    ["commence", "crucifix"],
    ["concert", "cumbersome"],
    ["cowbell", "customer"],
    ["crackdown", "Dakota"],
    ["cranky", "decadence"],
    ["crowfoot", "December"],
    ["crucial", "decimal"],
    ["crumpled", "designing"],
    ["crusade", "detector"],
    ["cubic", "detergent"],
    ["dashboard", "determine"],
    ["deadbolt", "dictator"],
    ["deckhand", "dinosaur"],
    ["dogsled", "direction"],
    ["dragnet", "disable"],
    ["drainage", "disbelief"],
    ["dreadful", "disruptive"],
    ["drifter", "distortion"],
    ["dropper", "document"],
    ["drumbeat", "embezzle"],
    ["drunken", "enchanting"],
    ["Dupont", "enrollment"],
    ["dwelling", "enterprise"],
    ["eating", "equation"],
    ["edict", "equipment"],
    ["egghead", "escapade"],
    ["eightball", "Eskimo"],
    ["endorse", "everyday"],
    ["endow", "examine"],
    ["enlist", "existence"],
    ["erase", "exodus"],
    ["escape", "fascinate"],
    ["exceed", "filament"],
    ["eyeglass", "finicky"],
    ["eyetooth", "forever"],
    ["facial", "fortitude"],
    ["fallout", "frequency"],
    ["flagpole", "gadgetry"],
    ["flatfoot", "Galveston"],
    ["flytrap", "getaway"],
    ["fracture", "glossary"],
    ["framework", "gossamer"],
    ["freedom", "graduate"],
    ["frighten", "gravity"],
    ["gazelle", "guitarist"],
    ["Geiger", "hamburger"],
    ["glitter", "Hamilton"],
    ["glucose", "handiwork"],
    ["goggles", "hazardous"],
    ["goldfish", "headwaters"],
    ["gremlin", "hemisphere"],
    ["guidance", "hesitate"],
    ["hamlet", "hideaway"],
    ["highchair", "holiness"],
    ["hockey", "hurricane"],
    ["indoors", "hydraulic"],
    ["indulge", "impartial"],
    ["inverse", "impetus"],
    ["involve", "inception"],
    ["island", "indigo"],
    ["jawbone", "inertia"],
    ["keyboard", "infancy"],
    ["kickoff", "inferno"],
    ["kiwi", "informant"],
    ["klaxon", "insincere"],
    ["locale", "insurgent"],
    ["lockup", "integrate"],
    ["merit", "intention"],
    ["minnow", "inventive"],
    ["miser", "Istanbul"],
    ["Mohawk", "Jamaica"],
    ["mural", "Jupiter"],
    ["music", "leprosy"],
    ["necklace", "letterhead"],
    ["Neptune", "liberty"],
    ["newborn", "maritime"],
    ["nightbird", "matchmaker"],
    ["Oakland", "maverick"],
    ["obtuse", "Medusa"],
    ["offload", "megaton"],
    ["optic", "microscope"],
    ["orca", "microwave"],
    ["payday", "midsummer"],
    ["peachy", "millionaire"],
    ["pheasant", "miracle"],
    ["physique", "misnomer"],
    ["playhouse", "molasses"],
    ["Pluto", "molecule"],
    ["preclude", "Montana"],
    ["prefer", "monument"],
    ["preshrunk", "mosquito"],
    ["printer", "narrative"],
    ["prowler", "nebula"],
    ["pupil", "newsletter"],
    ["puppy", "Norwegian"],
    ["python", "October"],
    ["quadrant", "Ohio"],
    ["quiver", "onlooker"],
    ["quota", "opulent"],
    ["ragtime", "Orlando"],
    ["ratchet", "outfielder"],
    ["rebirth", "Pacific"],
    ["reform", "pandemic"],
    ["regain", "Pandora"],
    ["reindeer", "paperweight"],
    ["rematch", "paragon"],
    ["repay", "paragraph"],
    ["retouch", "paramount"],
    ["revenge", "passenger"],
    ["reward", "pedigree"],
    ["rhythm", "Pegasus"],
    ["ribcage", "penetrate"],
    ["ringbolt", "perceptive"],
    ["robust", "performance"],
    ["rocker", "pharmacy"],
    ["ruffled", "phonetic"],
    ["sailboat", "photograph"],
    ["sawdust", "pioneer"],
    ["scallion", "pocketful"],
    ["scenic", "politeness"],
    ["scorecard", "positive"],
    ["Scotland", "potato"],
    ["seabird", "processor"],
    ["select", "provincial"],
    ["sentence", "proximate"],
    ["shadow", "puberty"],
    ["shamrock", "publisher"],
    ["showgirl", "pyramid"],
    ["skullcap", "quantity"],
    ["skydive", "racketeer"],
    ["slingshot", "rebellion"],
    ["slowdown", "recipe"],
    ["snapline", "recover"],
    ["snapshot", "repellent"],
    ["snowcap", "replica"],
    ["snowslide", "reproduce"],
    ["solo", "resistor"],
    ["southward", "responsive"],
    ["soybean", "retraction"],
    ["spaniel", "retrieval"],
    ["spearhead", "retrospect"],
    ["spellbind", "revenue"],
    ["spheroid", "revival"],
    ["spigot", "revolver"],
    ["spindle", "sandalwood"],
    ["spyglass", "sardonic"],
    ["stagehand", "Saturday"],
    ["stagnate", "savagery"],
    ["stairway", "scavenger"],
    ["standard", "sensation"],
    ["stapler", "sociable"],
    ["steamship", "souvenir"],
    ["sterling", "specialist"],
    ["stockman", "speculate"],
    ["stopwatch", "stethoscope"],
    ["stormy", "stupendous"],
    ["sugar", "supportive"],
    ["surmount", "surrender"],
    ["suspense", "suspicious"],
    ["sweatband", "sympathy"],
    ["swelter", "tambourine"],
    ["tactics", "telephone"],
    ["talon", "therapist"],
    ["tapeworm", "tobacco"],
    ["tempest", "tolerance"],
    ["tiger", "tomorrow"],
    ["tissue", "torpedo"],
    ["tonic", "tradition"],
    ["topmost", "travesty"],
    ["tracker", "trombonist"],
    ["transit", "truncated"],
    ["trauma", "typewriter"],
    ["treadmill", "ultimate"],
    ["Trojan", "undaunted"],
    ["trouble", "underfoot"],
    ["tumor", "unicorn"],
    ["tunnel", "unify"],
    ["tycoon", "universe"],
    ["uncut", "unravel"],
    ["unearth", "upcoming"],
    ["unwind", "vacancy"],
    ["uproot", "vagabond"],
    ["upset", "vertigo"],
    ["upshot", "Virginia"],
    ["vapor", "visitor"],
    ["village", "vocalist"],
    ["virus", "voyager"],
    ["Vulcan", "warranty"],
    ["waffle", "Waterloo"],
    ["wallet", "whimsical"],
    ["watchword", "Wichita"],
    ["wayside", "Wilmington"],
    ["willow", "Wyoming"],
    ["woodlark", "yesteryear"],
    ["Zulu", "Yucatan"],
];

// This can't be done with a binary search because the right side list is not sorted with "applicant" before "Apollo"
pub fn left_word(word: &str) -> Result<usize, Error> {
    PGP_WORDS
        .iter()
        .position(|p| p[0] == word)
        .ok_or_else(|| Error::Input(format!("invalid left word `{}` found", word)))
}

pub fn right_word(word: &str) -> Result<usize, Error> {
    PGP_WORDS
        .iter()
        .position(|p| p[1] == word)
        .ok_or_else(|| Error::Input(format!("invalid right word `{}` found", word)))
}

pub struct PgpWords {
    pub mode: BinaryToTextMode,
}

impl Default for PgpWords {
    fn default() -> Self {
        Self {
            mode: BinaryToTextMode::Utf8,
        }
    }
}

impl PgpWords {
    pub fn chars_codes(&mut self) -> impl Iterator<Item = (String, String)> + '_ {
        (0..256).map(|n| {
            (
                format!("{n:02x}"),
                format!("{}, {}", PGP_WORDS[n][0], PGP_WORDS[n][1]),
            )
        })
    }
}

impl BinaryToText for PgpWords {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, Error> {
        Ok(bytes
            .into_iter()
            .enumerate()
            .map(|(idx, byte)| PGP_WORDS[*byte as usize][idx % 2])
            .join(" "))
    }
}

impl Code for PgpWords {
    fn encode(&self, text: &str) -> Result<String, Error> {
        match self.mode {
            BinaryToTextMode::Hex => self.encode_hex(text),
            BinaryToTextMode::Utf8 => self.encode_utf8(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let words = text.split(" ");
        let mut left = true;
        let mut out = Vec::with_capacity(words.clone().count());
        for word in words {
            if left {
                out.push(format!("{:02X}", left_word(word)?))
            } else {
                out.push(format!("{:02X}", right_word(word)?))
            }
            left = !left;
        }
        Ok(out.join(" "))
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod pgp_tests {
    use super::*;

    #[test]
    pub fn encode_pgp_words() {
        let words = "topmost Istanbul Pluto vagabond treadmill Pacific brackish dictator goldfish Medusa afflict bravado chatter revolver Dupont midsummer stopwatch whimsical cowbell bottomless";
        let nums = "E5 82 94 F2 E9 A2 27 48 6E 8B 06 1B 31 CC 52 8F D7 FA 3F 19";
        let code = PgpWords {
            mode: BinaryToTextMode::Hex,
        };
        assert_eq!(words, code.encode(nums).unwrap())
    }

    #[test]
    pub fn encode_pgp_words_errs() {
        let nums = "E5 82 94 F2 E9 A2 27 48 6E 8B 06 1B 31 C 52 8F D7 FA 3F 19";
        let code = PgpWords {
            mode: BinaryToTextMode::Hex,
        };
        assert_eq!(
            Error::Input("not valid hex bytes".into()),
            code.encode(nums).unwrap_err()
        )
    }

    #[test]
    pub fn decode_pgp_words() {
        let words = "topmost Istanbul Pluto vagabond treadmill Pacific brackish dictator goldfish Medusa afflict bravado chatter revolver Dupont midsummer stopwatch whimsical cowbell bottomless";
        let nums = "E5 82 94 F2 E9 A2 27 48 6E 8B 06 1B 31 CC 52 8F D7 FA 3F 19";
        let code = PgpWords {
            mode: BinaryToTextMode::Hex,
        };
        assert_eq!(nums, code.decode(words).unwrap())
    }

    #[test]
    pub fn decode_pgp_words_errs() {
        let words1 = "topmst Istanbul Pluto vagabond treadmill Pacific";
        let words2 = "topmost Istannbul Pluto vagabond treadmill Pacific";

        let code = PgpWords::default();
        assert_eq!(
            Error::Input("invalid left word `topmst` found".into()),
            code.decode(words1).unwrap_err()
        );
        assert_eq!(
            Error::Input("invalid right word `Istannbul` found".into()),
            code.decode(words2).unwrap_err()
        );
    }
}
