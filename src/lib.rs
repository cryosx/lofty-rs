//! [![Crate](https://img.shields.io/crates/v/lofty.svg)](https://crates.io/crates/lofty)
//! [![Crate](https://img.shields.io/crates/d/lofty.svg)](https://crates.io/crates/lofty)
//! [![Crate](https://img.shields.io/crates/l/lofty.svg)](https://crates.io/crates/lofty)
//! [![Documentation](https://docs.rs/lofty/badge.svg)](https://docs.rs/lofty/)
//!
//! Parse, convert, and write metadata to audio files of different file types.
//!
//! This crate aims to provide a unified trait for parsers and writers of different audio file formats.
//! Without this crate, you would otherwise need to learn the different APIs in **id3**, **mp4ameta**, etc.
//! in order to parse metadata in different file formats.
//!
//! # Supported Formats
//!
//! | File Format   | Metadata Format | Backend                                                     |
//! |---------------|-----------------|-------------------------------------------------------------|
//! | `mp3`         | ID3v2.4         | [**id3**](https://github.com/polyfloyd/rust-id3)            |
//! | `wav`         | TODO            | TODO                                                        |
//! | `ape`         | TODO            | TODO                                                        |
//! | `opus`        | Vorbis Comment  | [**opus_headers**](https://github.com/zaethan/opus_headers) |
//! | `ogg`         | Vorbis Comment  | [**lewton**](https://github.com/RustAudio/lewton)           |
//! | `flac`        | Vorbis Comment  | [**metaflac**](https://github.com/jameshurst/rust-metaflac) |
//! | `m4a/mp4/...` | Vorbis Comment  | [**mp4ameta**](https://github.com/Saecki/rust-mp4ameta)     |
//!
//! # Examples
//!
//! ```
//! use lofty::{Tag, TagType};
//!
//! // Guess the format from the extension, in this case `mp3`
//! let mut tag = Tag::new().read_from_path("assets/a.mp3").unwrap();
//! tag.set_title("Foo");
//!
//! // You can convert the tag type and save the metadata to another file.
//! tag.to_dyn_tag(TagType::Mp4).write_to_path("assets/a.m4a");
//!
//! // You can specify the tag type, but when you want to do this
//! // also consider directly using the concrete type
//! let tag = Tag::new().with_tag_type(TagType::Mp4).read_from_path("assets/a.m4a").unwrap();
//! assert_eq!(tag.title(), Some("Foo"));
//! ```
//!
//! ## Performance
//!
//! Using lofty incurs a little overhead due to vtables if you want to guess the metadata format (from file extension).
//! Apart from this, the performance is almost the same as directly calling the function provided from those ‘specialized’ crates.
//!
//! No copies will be made if you only need to read and write metadata of one format. If you want to convert between tags, copying is
//! unavoidable, no matter if you use lofty or use getters and setters provided by specialized libraries. Lofty is not making additional
//! unnecessary copies.
//!
//! Theoretically, it is possible to achieve zero-copy conversions if all parsers can parse into a unified struct.
//! However, this is going to be a lot of work.

//#![forbid(unused_crate_dependencies, unused_import_braces)]
#![warn(clippy::pedantic)]
#![allow(
	clippy::too_many_lines,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::cast_possible_wrap,
	clippy::clippy::cast_possible_truncation,
	clippy::module_name_repetitions
)]

#[doc(hidden)]
mod macros;

mod anytag;
pub use crate::anytag::AnyTag;

mod types;
pub use crate::types::{Album, MimeType, Picture};

mod tag;
pub use crate::tag::{Tag, TagType};

mod error;
pub use crate::error::{Error, Result};

mod components;
pub use crate::components::{FlacTag, Id3v2Tag, Mp4Tag, OpusTag, VorbisTag};

mod traits;
pub use crate::traits::{AudioTag, AudioTagEdit, AudioTagWrite, ToAny, ToAnyTag};
