// Base handler
//
// Provides common utilities for token handlers.
use crate::parsers::lexer::{
    cursor::Cursor,
    utils::reader::TokenReader,
};

pub struct BaseHandler<'cursor, 'source> {
    pub reader: TokenReader<'cursor, 'source>,
}

impl<'cursor, 'source> BaseHandler<'cursor, 'source>
where
    'source: 'cursor,
{
    pub fn new(cursor: &'cursor mut Cursor<'source>) -> Self {
        Self {
            reader: TokenReader::new(cursor),
        }
    }
}
