use std::fmt::Display;


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CipherID {
    Caesar,
    Affine,
    Decoder,
    Substitution,
    Polybius,

    M209,
    Enigma,
    SIGABA,

    Playfair,
    Slidefair,

    Vigenere,
    Beaufort,
    Alberti,
    Bazieres,
    M94,

    Columnar,

    ADFGVX,
    Bifid,
    B64,
}

impl Default for CipherID {
    fn default() -> Self {
        Self::Caesar
    }
}

impl CipherID {
    // Describe the history of the cipher
    pub fn description(&self) -> &'static str {
        match self {
            CipherID::Caesar => "The Caesar Cipher is perhaps the simplest of ciphers, supposedly used by Julius Caesar to send messages to his troops. Each letter in the alphabet is just shifted some number of positions, wrapping around if needed. For example with the standard English alphabet a shift of 2 turns A in C and Y into A. The practical security of the cipher probably relied on low literacy in the era and lack of knowledge of the method of encryption. Since there are only as many possible keys as letters in the alphabet it is trivial to check all of them even if working by hand.",
            CipherID::Affine => "The Affine Cipher is a slight improvement to the Caesar Cipher. Rather than a simple shift an affine transformation is applied to the letters. This means each letter is assigned a value based on its position then the first key value is added to it, then that number is multiplied by the second key value, and finally the value is divided by the length of the alphabet and the remainder is taken. That number corresponds to some letter in the alphabet. However this method was not used generally by the ancients the multiplicative key sometimes cannot be reversed. The Atbash Code, which effectively reverses the alphabet can be produced as an affine cipher.",
            CipherID::Decoder => "A Decoder Ring (as popularized by Little Orphan Annie and Captain Midnight, presets exist for each) is a minor variation on the Caesar Cipher. Rather than simply shift the letter's position instead a number is assigned a number and the key is added to that number then reduced by the modulo operation. The original decoder rings were keyed in a slightly more complex way by telling listeners to match a specific letter to a specific number but this is equivalent to simply adding a value.",
            CipherID::Substitution => "The General Substituion Cipher maps a set of symbols one-to-one onto another arbitary set. This implementation allows only maping the symbols of an alphabet but all simple substitution ciphers are included in principle.",
            CipherID::Polybius => "The Polybius Square is an ancient substitutuion cipher that converts each character of the plaintext into a pair that describes its coordinates in a grid. Though it provides no special security on its own it is a key component of very strong composite ciphers.",
            
            CipherID::M209 => "The M209 was an entirely mechanical cipher machine used by the US Military with very complex key settings. The positions of the pins and lugs were set once a day. The exteral positions of the rotors were changed with each message.",
            CipherID::Enigma => "The Enigma machine is probably the most famous rotor machine from the brief era in which they dominated encryption. It was remarkable for its simplicity and compact size. Although it contained critical flaws ultimately the failure of Engima was caused by operational mistakes in the Nazi military that leaked information to the Allies.",
            CipherID::SIGABA => "SIGABA was the most complex rotor machine of its era and is not known to have been successfully attacked during its use. Despite its complexity the United States was extremely paranoid about the device and did not allow allies direct access to it.",

            CipherID::Playfair => "The Playfair Cipher swaps letters on a grid to encrypt letters pair by pair. Developed by Charles Wheatstone and promoted by Lord Playfair it was advertised as strong enough for tactical use and simple enough to teach to schoolchildren.",
            CipherID::Slidefair => "The Slidefair Cipher is a strong but more complex variation on the Playfair Cipher developed by Helen Gaines. It alters the grid with each pair encypted by sliding the alphabet into new positions. Effectively it combines the concept of the Playfair and Vigenère ciphers.",
            
            CipherID::Vigenere => "The Vigenère Cipher, introduced at least as early as 1553, was once known as 'le chifre indéchiffrable' (the unsolvable cipher) as it was among the first ciphers to provide more security than any simple subtitution cipher. The keyword is used as if it were a series of Caesar ciphers.  Although Babbage and Kasiski independetly developed methods to break the cipher in the mid 1800s its popular reputation as unbreakable persisted into the early 1900s. By encrypting a message several times the security of a Vigenere cipher can be increased dramatically. The effective key length is the least common multiple of all the keys used, assuming none of the keys repeat themselves. Thus the pair of key words 'VIGENERE' and 'CIPHER' would have a key with a length of 24. Further it is hard to seperate out the overlapping keys meaning that even if attacker has a large amount of text and knows the period of the key they cannot easily guess it.",
            CipherID::Beaufort => "The Beaufort Cipher is clever variation of the Vigenère that uses subtraction to produce a reciprocal cipher in which the action of encryption and decryption is identical.",
            //CipherID::Alberti => "The Alberti Cipher",
            //CipherID::Bazieres => "The Baziers Cipher",
            CipherID::M94 => "The M94 Cipher was a low security tactical cipher US Army that consisted of 25 wheels each with a scrambled alphabet, placed sequentially on a rod. The order of the wheels was changed daily. To send a message the wheels were turned to display it and then an arbitrary other line was used. Decryption relied on the reciever searching for the only sensible line on their own set of wheels but this implementation specifies an offset for each message. Messages had to be sent with exactly 25 letters at a time, padded if the message was too short and broken into pices if it was too long.",

            CipherID::Columnar => "The Columnar Transposition Cipher encrypts information by writing the text into a grid row by row and then reading it off column by column in the order decided by a keyword. To decrypt the text is simply written back into the grid column by column in the required order. The cipher is somewhat easier to use if the text fills all of the rows but this creates a serious weakness in that the key size can be guessed by factoring the length of the message. Though insecure on its own columnar transposition is a strong cipher if applied twice or combined with another layer of encryption.",
 
            CipherID::ADFGVX => "The ADFGX and ADFGVX Ciphers are among the most effective classical ciphers that can be executed entirely by hand. The first step of encryption is to use a Polybius square to convert each letter into a pair of symbols (after which the ciphers are named). Then those symbols are rearranged using a columnar transposition cipher. The symbols were chosen to be distinctive in Morse Code so as to reduce transmission errors.",
            CipherID::Bifid => "The Bifid Cipher combines a Polybius square with a very simple transposition in order to obscure as much information as possible about the plaintext. First the Polybius square is used to convert each letter into a pair of symbol, Then first symbol in each pair is written down after that the second symbol in each pair is written down. Finally this converted back to the original alphabet using the Polybius square once more.",
            CipherID::B64 => "The B64 Cipher is not a historical cipher. It is an (to the author's knowlege) novel cipher based on the ADFGVX and Bifid ciphers. Only symbols from the MIME Base64 alphabet are used, ensuring the message can be sent without risk of corruption over most digital systems. First a Polybius square is used to change the text into pairs of digits, then these pairs are shuffled by applying two columnar transpositions, and finally they Polybius square is applied in reverse to convert the results back to the Base64 alphabet to reduce the message size.",
            _ => "Missing description. Please complain to the author.",
        }
    }


}



impl Display for  CipherID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CipherID::Caesar => "Caesar",
            CipherID::Affine => "Affine",
            CipherID::Decoder => "Decoder Ring",
            CipherID::Substitution => "General Substittution",
            CipherID::M209 => "M209",
            CipherID::Playfair => "Playfair",
            CipherID::Alberti => "Alberti Cipher Disk",
            CipherID::Polybius => "Polybius Square",
            CipherID::Enigma => "Enigma",
            CipherID::SIGABA => "SIGABA",
            CipherID::Slidefair => "Slidefair",
            CipherID::Columnar => "Columnar Transposition",
            CipherID::Bazieres => "Bazieres",
            CipherID::M94 => "M94",
            CipherID::Vigenere => "Vigenère",
            CipherID::Beaufort => "Beaufort",
            CipherID::ADFGVX => "ADFGVX",
            CipherID::Bifid => "Bifid",
            CipherID::B64 => "B64",
        };
        write!(f,"{}",name)
    }
}

impl From<CipherID> for String {
    fn from(id: CipherID) -> Self {
        id.to_string()
    }
}