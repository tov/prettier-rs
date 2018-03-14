// I'm going to base this on HughesPJ (Haskell) as much as I can.

use std::fmt::Debug;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Doc<T>(Box<DocBase<T>>);

#[derive(Clone, Debug)]
enum DocBase<T> {
    Empty,
    NilAbove(Doc<T>),
    TextBeside(AnnotDetails<T>, Doc<T>),
    Nest(i32, Doc<T>),
    Union(Doc<T>, Doc<T>),
    NoDoc,
    Beside(Doc<T>, Vec<(bool, Doc<T>)>),
    Above(Doc<T>, Vec<(bool, Doc<T>)>),
}

#[derive(Clone, Debug)]
enum AnnotDetails<T> {
    Start,
    Str(String, i32),
    Chr(char, i32),
    End(T),
}

impl<T> Doc<T> {
    pub fn annotate(self, annotation: T) -> Self {
        let end = AnnotDetails::End(annotation).text_beside(Self::empty());
        AnnotDetails::Start.text_beside(self.reduce().beside(false, end))
    }

    pub fn char(c: char) -> Self {
        AnnotDetails::Chr(c, 1).text_beside(Self::empty())
    }

    pub fn text(s: String) -> Self {
        let len = s.len();
        Self::sized_text(len as i32, s)
    }

    pub fn sized_text(len: i32, s: String) -> Self {
        AnnotDetails::Str(s, len).text_beside(Self::empty())
    }

    pub fn zero_width_text(s: String) -> Self {
        Self::sized_text(0, s)
    }

    pub fn empty() -> Self {
        Doc(Box::new(DocBase::Empty))
    }

    pub fn is_empty(&self) -> bool {
        match *self.0 {
            DocBase::Empty => true,
            _ => false
        }
    }

    pub fn debug<S: Debug>(s: &S) -> Self {
        Self::text(format!("{:?}", s))
    }

    pub fn display<S: Display>(s: &S) -> Self {
        Self::text(format!("{}", s))
    }

    fn surround(self, left: char, right: char) -> Self {
        Self::char(left).beside(false, self).beside(false, Self::char(right))
    }

    pub fn parens(self) -> Self {
        self.surround('(', ')')
    }

    pub fn brackets(self) -> Self {
        self.surround('[', ']')
    }

    pub fn braces(self) -> Self {
        self.surround('{', '}')
    }

    pub fn quotes(self) -> Self {
        self.surround('\'', '\'')
    }

    pub fn double_quotes(self) -> Self {
        self.surround('"', '"')
    }

    pub fn curly_quotes(self) -> Self {
        self.surround('‘', '’')
    }

    pub fn curly_double_quotes(self) -> Self {
        self.surround('“', '”')
    }

    pub fn maybe_parens(self, on: bool) -> Self {
        if on {self.parens()} else {self}
    }

    pub fn maybe_brackets(self, on: bool) -> Self {
        if on {self.brackets()} else {self}
    }

    pub fn maybe_braces(self, on: bool) -> Self {
        if on {self.braces()} else {self}
    }

    pub fn maybe_quotes(self, on: bool) -> Self {
        if on {self.quotes()} else {self}
    }

    pub fn maybe_double_quotes(self, on: bool) -> Self {
        if on {self.double_quotes()} else {self}
    }

    pub fn maybe_curly_quotes(self, on: bool) -> Self {
        if on {self.curly_quotes()} else {self}
    }

    pub fn maybe_curly_double_quotes(self, on: bool) -> Self {
        if on {self.curly_double_quotes()} else {self}
    }

    pub fn beside(mut self, space: bool, other: Doc<T>) -> Self {
        match *self.0 {
            DocBase::Beside(_, ref mut rest) => {
                rest.push((space, other));
            }
            _ => {
                self = Doc(Box::new(DocBase::Beside(self, vec![(space, other)])))
            }
        }
        self
    }

    pub fn above(mut self, space: bool, other: Doc<T>) -> Self {
        match *self.0 {
            DocBase::Above(_, ref mut rest) => {
                rest.push((space, other));
            }
            _ => {
                self = Doc(Box::new(DocBase::Above(self, vec![(space, other)])))
            }
        }
        self
    }

    fn reduce(self) -> Self {
        self
    }
}

impl<T> AnnotDetails<T> {
    fn text_beside(self, doc: Doc<T>) -> Doc<T> {
        Doc(Box::new(DocBase::TextBeside(self, doc)))
    }
}
