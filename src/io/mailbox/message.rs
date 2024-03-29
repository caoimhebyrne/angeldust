// TODO: Document these messages

// Not all messages will be used.
#![allow(dead_code)]

use crate::io::mac::MacAddress;

use super::types::MessageTag;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TagIdentifier {
    GetFirmwareVersion = 0x0_0001,
    GetBoardRevision = 0x1_0002,
    GetBoardMacAddress = 0x1_0003,
    GetArmMemory = 0x1_0005,

    AllocateBuffer = 0x4_0001,
    SetPhysicalDisplaySize = 0x4_8003,
    SetVirtualDisplaySize = 0x4_8004,
    SetDepth = 0x4_8005,
    SetPixelOrder = 0x4_8006,
    GetPitch = 0x4_0008,
    SetVirtualOffset = 0x4_8009,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct GetFirmwareVersionMessage {
    pub firmware_version: u32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct GetBoardMacAddress {
    pub address: MacAddress,
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
