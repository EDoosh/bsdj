const EMPTY_PHRASE: u8 = 0xff;
// 0x00 to 0x7f
const CHAIN_COUNT: usize = 0x80;
// 16 phrases per chain.
const PHRASES_PER_CHAIN: usize = 0x10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chains {
    chains: [Chain; CHAIN_COUNT],
}

impl Chains {
    /// Construct a new resource holding the chains.
    pub fn new() -> Chains {
        Chains {
            chains: [Chain::new(); CHAIN_COUNT],
        }
    }

    /// Get a chain by its index.
    pub fn get(&self, index: usize) -> Option<&Chain> {
        self.chains.get(index)
    }

    /// Gets a mutable chain by its index.
    ///
    /// Returns None if the specified index does not exist.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Chain> {
        self.chains.get_mut(index)
    }
}

impl Default for Chains {
    fn default() -> Self {
        Chains::new()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chain {
    phrases: [u8; PHRASES_PER_CHAIN],
    transposes: [u8; PHRASES_PER_CHAIN],
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            phrases: [EMPTY_PHRASE; PHRASES_PER_CHAIN],
            transposes: [0x00; PHRASES_PER_CHAIN],
        }
    }

    /// Returns a phrase ID at a position in the chain.
    /// Returns None if the phrase at that index is empty or if
    /// the index was out of bounds.
    pub fn get_phrase(&self, index: usize) -> Option<u8> {
        let phrase = self.phrases.get(index);
        phrase.filter(|p| **p != EMPTY_PHRASE).cloned()
    }

    /// Sets the phrase at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn set_phrase(&mut self, index: usize, value: u8) -> Option<()> {
        *self.phrases.get_mut(index)? = value;
        Some(())
    }

    /// Clears the phrase at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn clear_phrase(&mut self, index: usize) -> Option<()> {
        *self.phrases.get_mut(index)? = EMPTY_PHRASE;
        Some(())
    }

    /// Returns a transpose amount at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn get_transpose(&self, index: usize) -> Option<u8> {
        self.transposes.get(index).cloned()
    }

    /// Sets the phrase at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn set_transpose(&mut self, index: usize, value: u8) -> Option<()> {
        *self.transposes.get_mut(index)? = value;
        Some(())
    }
}
