use utils::functions::string_chunks;

pub struct SeriatedPlayfair {
    period: usize,
    spacer: char,
}

impl SeriatedPlayfair {
    fn groups(&self, text: &str) -> Vec<String> {
        let mut chunks = string_chunks(text, self.period);

        // if there are an even number of chunks fill ot the last one with spacers
        if chunks.len() % 2 == 0 {
            let x = chunks.last_mut().unwrap();
            while x.len() != self.period {
                x.push(self.spacer)
            }
        // if there are an odd number of chunks split the last one in half and fill out the second half with spacers
        } else {
            let last = chunks.pop().unwrap();
            let len = last.len();
            let left: String = last.chars().take(len / 2 + 1).collect();
            let right: String = last.chars().skip(len / 2 + 1).collect();
            chunks.push(left);
            chunks.push(right);
        }
        chunks
    }
}

#[cfg(test)]
mod seriated_playfair_tests {
    use super::*;

    #[test]
    fn chunks() {
        println!(
            "{:?}",
            string_chunks("THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG", 4)
        )
    }
}
