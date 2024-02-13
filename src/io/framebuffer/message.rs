use crate::io::mailbox::{types::MessageTag, TagIdentifier};

/// Holds all of the tags sent during a [Framebuffer::initialize] call.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FramebufferInitializeRequest {
    pub set_physical_size_request: MessageTag<SetDisplaySizeMessage>,
    pub set_virtual_size_request: MessageTag<SetDisplaySizeMessage>,
    pub set_depth_request: MessageTag<SetDepthMessage>,
    pub set_pixel_order_request: MessageTag<SetPixelOrderMessage>,
    pub allocate_buffer_request: MessageTag<AllocateBufferRequest>,
    pub get_pitch_request: MessageTag<GetPitchMessage>,
}

/// Holds all of the tags received after a [FramebufferInitializeRequest] has been sent.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FramebufferInitializeResponse {
    set_physical_size_response: MessageTag<SetDisplaySizeMessage>,
    set_virtual_size_response: MessageTag<SetDisplaySizeMessage>,
    set_depth_response: MessageTag<SetDepthMessage>,
    set_pixel_order_response: MessageTag<SetPixelOrderMessage>,
    allocate_buffer_response: MessageTag<AllocateBufferResponse>,
    get_pitch_response: MessageTag<GetPitchMessage>,
}

/// A helper for accessing the [MessageTag::data] of the fields.
impl FramebufferInitializeResponse {
    pub fn allocate_buffer_response(&self) -> AllocateBufferResponse {
        self.allocate_buffer_response.data
    }

    pub fn set_physical_size_response(&self) -> SetDisplaySizeMessage {
        self.set_physical_size_response.data
    }

    pub fn set_virtual_size_response(&self) -> SetDisplaySizeMessage {
        self.set_virtual_size_response.data
    }

    pub fn set_depth_response(&self) -> SetDepthMessage {
        self.set_depth_response.data
    }

    pub fn set_pixel_order_response(&self) -> SetPixelOrderMessage {
        self.set_pixel_order_response.data
    }

    pub fn get_pitch_response(&self) -> GetPitchMessage {
        self.get_pitch_response.data
    }
}

/// Allocates a frame buffer using a certain alignment.
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#allocate-buffer
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AllocateBufferRequest {
    /// The alignment of the buffer address in bytes.
    pub alignment: u32,

    /// Our response needs to have two fields, but the request only has one, so we need to pad it.
    _padding: u32,
}

impl AllocateBufferRequest {
    /// A helper function for creating a [MessageTag] for this request.
    pub fn new(alignment: u32) -> MessageTag<AllocateBufferRequest> {
        MessageTag::new(
            TagIdentifier::AllocateBuffer,
            AllocateBufferRequest {
                alignment,
                _padding: 0,
            },
        )
    }
}

/// The response received after a [AllocateBufferRequest] has been sent.
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#allocate-buffer
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AllocateBufferResponse {
    pub base_address: u32,
    pub size: u32,
}

/// Sets the display's width and height, can be used for physical or virtual display size.
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#set-physical-display-widthheight
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#set-virtual-buffer-widthheight
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetDisplaySizeMessage {
    /// The width of the display.
    pub width: u32,

    /// The height of the display.
    pub height: u32,
}

impl SetDisplaySizeMessage {
    /// A helper function for creating a [MessageTag] for this request.
    pub fn new_physical(width: u32, height: u32) -> MessageTag<SetDisplaySizeMessage> {
        MessageTag::new(
            TagIdentifier::SetPhysicalDisplaySize,
            SetDisplaySizeMessage { width, height },
        )
    }

    /// A helper function for creating a [MessageTag] for this request.
    pub fn new_virtual(width: u32, height: u32) -> MessageTag<SetDisplaySizeMessage> {
        MessageTag::new(
            TagIdentifier::SetVirtualDisplaySize,
            SetDisplaySizeMessage { width, height },
        )
    }
}

/// Sets the bits-per-pixel for the framebuffer.
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#set-depth
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetDepthMessage {
    pub bits_per_pixel: u32,
}

impl SetDepthMessage {
    /// A helper function for creating a [MessageTag] for this request.
    pub fn new(depth: u32) -> MessageTag<SetDepthMessage> {
        MessageTag::new(
            TagIdentifier::SetDepth,
            SetDepthMessage {
                bits_per_pixel: depth,
            },
        )
    }
}

/// Represents the different pixel orders supported by the Raspberry Pi's framebuffer.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PixelOrder {
    // Blue, Green, Red.
    BGR = 0x0,

    // Red, Green, Blue.
    RGB = 0x1,

    // Usually returned when the mailbox's data has been mangled in some way.
    Unknown(u32),
}

/// Sets the [PixelOrder] for this framebuffer.
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#set-pixel-order
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetPixelOrderMessage {
    pub pixel_order: PixelOrder,
}

impl SetPixelOrderMessage {
    /// A helper function for creating a [MessageTag] for this request.
    pub fn new(pixel_order: PixelOrder) -> MessageTag<SetPixelOrderMessage> {
        MessageTag::new(
            TagIdentifier::SetPixelOrder,
            SetPixelOrderMessage { pixel_order },
        )
    }
}

/// Asks for the bytes-per-line for the framebuffer.
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#get-pitch
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GetPitchMessage {
    pub bytes_per_line: u32,
}

impl GetPitchMessage {
    /// A helper function for creating a [MessageTag] for this request.
    pub fn new() -> MessageTag<GetPitchMessage> {
        MessageTag::new(
            TagIdentifier::GetPitch,
            GetPitchMessage { bytes_per_line: 0 },
        )
    }
}
