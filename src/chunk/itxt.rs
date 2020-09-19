// PNG Pong
//
// Copyright © 2019-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::Chunk;
use crate::{
    consts, decode::Error as DecoderError, decoder::Parser,
    encode::Error as EncoderError, encoder::Enc, zlib,
};
use std::io::{Read, Write};

/// International Text Chunk Data (iTXt)
#[derive(Clone, Debug)]
pub struct InternationalText {
    /// A keyword that gives a short description of what the text in `val`
    /// represents, e.g. Title, Author, Description, or anything else.  Minimum
    /// of 1 character, and maximum 79 characters long.
    pub key: String,
    /// Additional string "langtag"
    pub langtag: String,
    /// Additional string "transkey"
    pub transkey: String,
    /// The actual message.  It's discouraged to use a single line length longer
    /// than 79 characters
    pub val: String,
    /// If the text should be compressed
    pub compressed: bool,
}

impl InternationalText {
    /*international text chunk (iTXt)*/
    pub(crate) fn parse<R: Read>(
        parse: &mut Parser<R>,
    ) -> Result<Chunk, DecoderError> {
        let key = parse.str()?;
        if key.is_empty() || key.len() > 79 {
            return Err(DecoderError::TextSize(key.len()));
        }
        let compressed = parse.u8()? != 0;
        if parse.u8()? != 0 {
            return Err(DecoderError::CompressionMethod);
        }
        let langtag = parse.str()?;
        let transkey = parse.str()?;
        let data = parse.vec(
            parse.len() - (key.len() + langtag.len() + transkey.len() + 5),
        )?;

        let val = if compressed {
            String::from_utf8_lossy(&zlib::decompress(&data)?).to_string()
        } else {
            String::from_utf8_lossy(&data).to_string()
        };
        Ok(Chunk::InternationalText(InternationalText {
            key,
            langtag,
            transkey,
            val,
            compressed,
        }))
    }

    pub(crate) fn write<W: Write>(
        &self,
        enc: &mut Enc<W>,
    ) -> Result<(), EncoderError> {
        // Checks
        let k_len = self.key.len();
        if k_len < 1 || k_len > 79 {
            return Err(EncoderError::TextSize(k_len));
        }

        // Maybe compress
        let zdata = if self.compressed {
            let mut data = Vec::new();
            zlib::compress(&mut data, self.val.as_bytes(), enc.level());
            Some(data)
        } else {
            None
        };

        // Encode
        enc.prepare(
            self.key.len() + self.langtag.len() + self.transkey.len() + 5,
            consts::ITEXT,
        )?;
        enc.str(&self.key)?;
        enc.u8(self.compressed as u8)?;
        enc.u8(0)?;
        enc.str(&self.langtag)?;
        enc.str(&self.transkey)?;
        if let Some(zdata) = zdata {
            enc.raw(&zdata)?;
        } else {
            enc.raw(self.val.as_bytes())?;
        }
        enc.write_crc()
    }
}