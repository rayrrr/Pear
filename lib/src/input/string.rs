pub use crate::input::{Input, Token, Slice, ParserInfo};

impl<'a, 'b: 'a> Slice<&'a str> for &'b str {
    default fn eq_slice(&self, other: &&str) -> bool { self == other }
    default fn into_slice(self) -> &'a str { self }
}

impl<'a> Input for &'a str {
    type Token = char;
    type Slice = &'a str;
    type Many = Self::Slice;

    type Marker = usize;
    type Context = String;

    /// Returns a copy of the current token, if there is one.
    fn token(&mut self) -> Option<Self::Token> {
        self.chars().next()
    }

    /// Returns a copy of the current slice of size `n`, if there is one.
    fn slice(&mut self, n: usize) -> Option<Self::Slice> {
        if self.len() < n {
            None
        } else {
            self.get(..n)
        }
    }

    /// Checks if the current token fulfills `cond`.
    fn peek<F>(&mut self, mut cond: F) -> bool
        where F: FnMut(&Self::Token) -> bool
    {
        self.token().map(|t| cond(&t)).unwrap_or(false)
    }

    /// Checks if the current slice of size `n` (if any) fulfills `cond`.
    fn peek_slice<F>(&mut self, n: usize, mut cond: F) -> bool
        where F: FnMut(&Self::Slice) -> bool
    {
        self.slice(n).map(|s| cond(&s)).unwrap_or(false)
    }

    /// Checks if the current token fulfills `cond`. If so, the token is
    /// consumed and returned. Otherwise, returns `None`.
    fn eat<F>(&mut self, mut cond: F) -> Option<Self::Token>
        where F: FnMut(&Self::Token) -> bool
    {
        if let Some(token) = self.token() {
            if cond(&token) {
                *self = &self[token.len_utf8()..];
                return Some(token)
            }
        }

        None
    }

    /// Checks if the current slice of size `n` (if any) fulfills `cond`. If so,
    /// the slice is consumed and returned. Otherwise, returns `None`.
    fn eat_slice<F>(&mut self, n: usize, mut cond: F) -> Option<Self::Slice>
        where F: FnMut(&Self::Slice) -> bool
    {
        if let Some(slice) = self.slice(n) {
            if cond(&slice) {
                *self = &self[slice.len()..];
                return Some(slice)
            }
        }

        None
    }

    /// Takes tokens while `cond` returns true, collecting them into a
    /// `Self::Many` and returning it.
    fn take<F>(&mut self, mut cond: F) -> Self::Many
        where F: FnMut(&Self::Token) -> bool
    {
        let mut consumed = 0;
        for c in self.chars() {
            if !cond(&c) { break; }
            consumed += c.len_utf8();
        }

        let value = &self[..consumed];
        *self = &self[consumed..];
        value
    }

    /// Skips tokens while `cond` returns true. Returns the number of skipped
    /// tokens.
    fn skip<F>(&mut self, mut cond: F) -> usize
        where F: FnMut(&Self::Token) -> bool
    {
        let mut skipped = 0;
        match self.take(|c| { skipped += 1; cond(c) }) {
            "" => 0,
            _ => skipped - 1
        }
    }

    /// Returns `true` if there are no more tokens.
    fn is_eof(&mut self) -> bool {
        self.is_empty()
    }

    fn context(&mut self, _mark: Option<&Self::Marker>) -> Option<Self::Context> {
        match std::cmp::min(self.len(), 5) {
            0 => None,
            n => Some(format!("{:?}", &self[..n]))
        }
    }
}
