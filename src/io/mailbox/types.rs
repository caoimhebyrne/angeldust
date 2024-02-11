use super::{Channel, Mailbox};
use crate::{print, println};
use core::{fmt::Debug, mem::size_of, ptr::read_volatile};

// TODO: Split up this file, it's way too big and annoying

/// A struct representing a message sent to/from the Raspberry Pi's peripheral mailbox.
///
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface
#[derive(Debug)]
#[repr(C, align(16))]
pub struct MessageHeader<T: Debug> {
    /// The entire size of the buffer, including header values, the end tag, and other padding.
    size: u32,

    /// The status of this message.
    ///
    /// This should start as [MessageStatus::Request], and will be changed by the Mailbox depending
    /// on whether it could handle the message or not.
    status: MessageStatus,

    /// The sequence of message tags.
    ///
    /// A single message can contain multiple tags, so let's treat this as an array of [T].
    tag: T,

    /// The end tag, to be ignored.
    end: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4))]
pub struct MessageTag<T: Debug> {
    /// The identifier used for this tag.
    identifier: TagIdentifier,

    /// The size of the value in bytes.
    value_size: u32,

    /// Request and response codes (mostly ignored).
    codes: u32,

    /// The data sent/returned within this tag.
    data: T,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum MessageStatus {
    Request = 0x0000_0000,
    Success = 0x8000_0000,
    Error = 0x8000_0001,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TagIdentifier {
    GetFirmwareVersion = 0x0_0001,
    GetBoardRevision = 0x1_0002,
    GetBoardMacAddress = 0x1_0003,
    GetArmMemory = 0x1_0005,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CombinedTag<A, B>(A, B);

pub struct MessageBuilder<C>(C);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GetFirmwareVersionMessage {
    firmware_version: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GetBoardMacAddress {
    bytes: [u8; 6],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GetArmMemory {
    base_address: u32,
    size: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GetBoardRevision {
    board_revision: u32,
}

impl<T: Debug> MessageHeader<T> {
    pub fn new(tag: T) -> MessageHeader<T> {
        MessageHeader {
            size: size_of::<MessageHeader<T>>() as u32,
            status: MessageStatus::Request,
            tag,
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

impl GetFirmwareVersionMessage {
    pub fn new() -> MessageTag<GetFirmwareVersionMessage> {
        MessageTag::new(
            TagIdentifier::GetFirmwareVersion,
            GetFirmwareVersionMessage {
                firmware_version: 0x0,
            },
        )
    }
}

impl GetBoardMacAddress {
    pub fn new() -> MessageTag<GetBoardMacAddress> {
        MessageTag::new(
            TagIdentifier::GetBoardMacAddress,
            GetBoardMacAddress { bytes: [0; 6] },
        )
    }
}

impl GetArmMemory {
    pub fn new() -> MessageTag<GetArmMemory> {
        MessageTag::new(
            TagIdentifier::GetArmMemory,
            GetArmMemory {
                base_address: 0,
                size: 0,
            },
        )
    }
}

impl GetBoardRevision {
    pub fn new() -> MessageTag<GetBoardRevision> {
        MessageTag::new(
            TagIdentifier::GetBoardRevision,
            GetBoardRevision { board_revision: 0 },
        )
    }
}

impl<C> MessageBuilder<C> {
    pub fn with<T>(self, tag: T) -> MessageBuilder<CombinedTag<C, T>> {
        MessageBuilder(CombinedTag(self.0, tag))
    }
}

pub fn test_mailbox(mailbox: &Mailbox) {
    let builder = MessageBuilder(GetBoardMacAddress::new())
        .with(GetFirmwareVersionMessage::new())
        .with(GetArmMemory::new())
        .with(GetBoardRevision::new());

    send_mailbox_request(
        mailbox,
        Channel::PropertyTags,
        MessageHeader::new(builder.0),
    )
}

// TODO: Improve this API
//       Right now it's very basic, and needs to be worked on.
//       - Messages are convoluted to construct
//       - Success/failure is not checked
//       - Bits of tag codes aren't checked to indicate a succesful tag response
pub fn send_mailbox_request<T: Debug>(
    mailbox: &Mailbox,
    channel: Channel,
    message: MessageHeader<T>,
) {
    let ptr = (&message as *const MessageHeader<T>) as u32;
    mailbox.write(ptr, channel);

    let response = unsafe { read_volatile(ptr as *const MessageHeader<T>) };
    println!(
        "[angeldust::mailbox::send_request] response: {:#0x?}",
        response
    );
}
