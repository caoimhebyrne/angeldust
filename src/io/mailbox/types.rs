use super::TagIdentifier;
use core::{fmt::Debug, mem::size_of};

/// A struct representing a message sent to/from the Raspberry Pi's peripheral mailbox.
///
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface
#[derive(Debug)]
#[repr(C, align(16))]
pub struct Message<T: Debug> {
    /// The entire size of the buffer, including header values, the end tag, and other padding.
    pub size: u32,

    /// The status of this message.
    ///
    /// This should start as [MessageStatus::Request], and will be changed by the Mailbox depending
    /// on whether it could handle the message or not.
    pub status: MessageStatus,

    /// The sequence of message tags.
    ///
    /// A single message can contain multiple tags, so let's treat this as an array of [T].
    pub data: T,

    /// The end tag, to be ignored.
    end: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4))]
pub struct MessageTag<T: Debug> {
    /// The identifier used for this tag.
    pub identifier: TagIdentifier,

    /// The size of the value in bytes.
    pub value_size: u32,

    /// Request and response codes (mostly ignored).
    pub codes: u32,

    /// The data sent/returned within this tag.
    pub data: T,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[allow(dead_code)]
pub enum MessageStatus {
    Request = 0x0000_0000,
    Success = 0x8000_0000,
    Error = 0x8000_0001,
}

impl<T: Debug> Message<T> {
    pub fn new(tag: T) -> Message<T> {
        Message {
            size: size_of::<Message<T>>() as u32,
            status: MessageStatus::Request,
            data: tag,
            end: 0x0,
        }
    }
}

impl<T: Debug> MessageTag<T> {
    pub fn new(identifier: TagIdentifier, data: T) -> MessageTag<T> {
        let data_size = size_of::<T>() as u32;
        MessageTag {
            identifier,
            value_size: data_size.next_multiple_of(4),
            codes: 0,
            data,
        }
    }
}
