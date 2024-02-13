// TODO: Document these messages

// Not all messages will be used.
#![allow(dead_code)]

use super::types::MessageTag;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TagIdentifier {
    GetFirmwareVersion = 0x0_0001,
    GetBoardRevision = 0x1_0002,
    GetBoardMacAddress = 0x1_0003,
    GetArmMemory = 0x1_0005,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct GetFirmwareVersionMessage {
    pub firmware_version: u32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct GetBoardMacAddress {
    pub bytes: [u8; 6],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct GetArmMemory {
    pub base_address: u32,
    pub size: u32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct GetBoardRevision {
    pub board_revision: u32,
}

impl GetFirmwareVersionMessage {
    pub fn new() -> MessageTag<GetFirmwareVersionMessage> {
        MessageTag::new(
            TagIdentifier::GetFirmwareVersion,
            GetFirmwareVersionMessage::default(),
        )
    }
}

impl GetBoardMacAddress {
    pub fn new() -> MessageTag<GetBoardMacAddress> {
        MessageTag::new(
            TagIdentifier::GetBoardMacAddress,
            GetBoardMacAddress::default(),
        )
    }
}

impl GetArmMemory {
    pub fn new() -> MessageTag<GetArmMemory> {
        MessageTag::new(TagIdentifier::GetArmMemory, GetArmMemory::default())
    }
}

impl GetBoardRevision {
    pub fn new() -> MessageTag<GetBoardRevision> {
        MessageTag::new(TagIdentifier::GetBoardRevision, GetBoardRevision::default())
    }
}
