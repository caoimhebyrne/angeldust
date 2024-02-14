use super::message::{
    AllocateBufferRequest, FramebufferInitializeRequest, FramebufferInitializeResponse, PixelOrder,
};
use crate::io::framebuffer::message::SetVirtualOffsetMessage;
use crate::mailbox::{types::Message, Channel, Mailbox, MailboxError};
use crate::{
    io::framebuffer::message::{
        GetPitchMessage, SetDepthMessage, SetDisplaySizeMessage, SetPixelOrderMessage,
    },
    print, println,
};

/// Represents an error that can occur during the [Framebuffer]'s operations.
#[derive(Debug)]
pub enum FramebufferError {
    /// Occurs when the framebuffer has already been initialized.
    AlreadyInitialized,

    /// Occurs when [Framebuffer::draw_pixel] is called when the framebuffer has not been
    /// initialized yet.
    NotInitialized,

    /// Occurs when the Pi does not respond to our [SetDisplaySizeMessage].
    InvalidDisplaySize {
        physical: SetDisplaySizeMessage,
        r#virtual: SetDisplaySizeMessage,
    },

    /// Occurs when the Pi does not set the pixel depth to the value that we requested.
    UnsupportedDepth(u32),

    /// Occurs when the Pi does not set its pixel order to the one we requested.
    UnsupportedPixelOrder(PixelOrder),

    /// Occurs when the Pi does not send a valid size or address for the framebuffer.
    /// This usually occurs when a display is not connected, or some other configuration
    /// issue occurs.
    FailedToAllocateBuffer,

    /// Occurs when the Pi does not return a valid value for the framebuffer's pitch.
    InvalidPitch(u32),

    /// Occurs when the mailbox returns an error that we can not recover from.
    Mailbox(MailboxError),
}

/// Used by [Framebuffer] to store important information received from the [FramebufferInitializeRequest].
#[derive(Debug, Clone, Copy)]
struct FramebufferInfo {
    /// The address of the framebuffer.
    address: *mut u32,

    /// The size of the framebuffer.
    // size: u32,

    // The bytes-per-line of the framebuffer.
    pitch: u32,
}

/// Represents the Raspberry Pi's framebuffer.
/// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#frame-buffer
#[derive(Debug, Default, Clone, Copy)]
pub struct Framebuffer {
    /// Contains the base address and size of the allocated framebuffer.
    info: Option<FramebufferInfo>,
}

impl Framebuffer {
    /// Attempts to initialize the [Framebuffer].
    pub fn initialize(&mut self, mailbox: &Mailbox) -> Result<(), FramebufferError> {
        // If the info is already set, we don't want to start allocating a new framebuffer,
        // as that will de-allocate the current one.
        if self.info.is_some() {
            return Err(FramebufferError::AlreadyInitialized);
        }

        // According to the mailbox property interface documentation, when initializing the framebuffer,
        // all tags must be sent in one operation.
        // Furthermore, if the allocate tag is omitted, no change occurs unless it can be accomodated
        // without changing the buffer size (which is not possible most of the time).
        let request = FramebufferInitializeRequest {
            set_physical_size_request: SetDisplaySizeMessage::new_physical(1280, 720),
            set_virtual_size_request: SetDisplaySizeMessage::new_virtual(1280, 720),
            set_virtual_offset_request: SetVirtualOffsetMessage::new(0, 0),
            set_depth_request: SetDepthMessage::new(32),
            set_pixel_order_request: SetPixelOrderMessage::new(PixelOrder::BGR),
            allocate_buffer_request: AllocateBufferRequest::new(4096),
            get_pitch_request: GetPitchMessage::new(),
        };

        let response: FramebufferInitializeResponse = mailbox
            .send(Channel::PropertyTags, Message::new(request))
            .map_err(|it| FramebufferError::Mailbox(it))?;

        // Checks to make sure that in the responses, we receive options that we can work with.
        // For example, if the Pi doesn't support RGB, we will throw an error.
        self.validate_response(&response)?;

        // If everything is valid, we can continue to set the info.
        self.info = Some(FramebufferInfo {
            address: (response.allocate_buffer_response().base_address & 0x3FFFFFFF) as *mut u32,
            // size: response.allocate_buffer_response().size,
            pitch: response.get_pitch_response().bytes_per_line,
        });

        println!(
            "[angeldust::framebuffer] initialized framebuffer at {:#0x}",
            response.allocate_buffer_response().base_address
        );

        Ok(())
    }

    /// Draws a pixel onto the framebuffer at (x, y).
    /// The [color] is in ABGR format.
    ///
    /// ## Errors
    /// - [FramebufferError::NotInitialized] if [Framebuffer::initialize] has not been called yet.
    ///
    /// ## Safety
    /// - This function assumes that [FramebufferInfo::address] is valid.
    pub fn draw_pixel(&self, x: u32, y: u32, color: u32) -> Result<(), FramebufferError> {
        let info = match self.info {
            Some(value) => value,
            None => return Err(FramebufferError::NotInitialized),
        };

        // FIXME: Make sure X and Y don't go out of bounds? Or is that too high-level for here?
        let offset = (x * 4) + (y * info.pitch);
        let ptr = unsafe { info.address.byte_offset(offset.try_into().unwrap()) };
        unsafe { *ptr = color };

        Ok(())
    }

    /// Fills an area on the framebuffer from (x, y) to (x + width, y + height).
    /// The [color] is in ABGR format.
    /// ## Errors
    /// - [FramebufferError::NotInitialized] if [Framebuffer::initialize] has not been called yet.
    ///
    /// ## Safety
    /// - This function assumes that [FramebufferInfo::address] is valid.
    pub fn fill_area(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: u32,
    ) -> Result<(), FramebufferError> {
        let info = match self.info {
            Some(value) => value,
            None => return Err(FramebufferError::NotInitialized),
        };

        let mut line = unsafe {
            info.address
                .byte_add(((y * info.pitch) + (x * 4)).try_into().unwrap())
        };

        let mut rect_y: u32 = 0;
        while rect_y < height - y {
            let mut rect_x = 0;
            let mut pixel = line;

            while rect_x < width - x {
                unsafe { (pixel as *mut u32).write(color) };

                rect_x += 1;
                pixel = unsafe { pixel.byte_add(4) };
            }

            rect_y += 1;
            line = unsafe { line.byte_add(info.pitch.try_into().unwrap()) }
        }

        Ok(())
    }

    /// Validates a [FramebufferInitializeResponse] by making sure values are either not 0, or
    /// set to their supported values.
    fn validate_response(
        &self,
        response: &FramebufferInitializeResponse,
    ) -> Result<(), FramebufferError> {
        // Ensure that the display size was set to something larger than 0.
        let physical_size = response.set_physical_size_response();
        let virtual_size = response.set_virtual_size_response();
        if physical_size.width == 0
            || physical_size.height == 0
            || virtual_size.width == 0
            || virtual_size.height == 0
        {
            return Err(FramebufferError::InvalidDisplaySize {
                physical: physical_size,
                r#virtual: virtual_size,
            });
        }

        // Ensure that the pixel order is RGB.
        let pixel_order = response.set_pixel_order_response().pixel_order;
        if pixel_order != PixelOrder::BGR {
            return Err(FramebufferError::UnsupportedPixelOrder(pixel_order));
        }

        // Ensure that the depth is 32 (R, G, B, A).
        let depth = response.set_depth_response().bits_per_pixel;
        if depth != 32 {
            return Err(FramebufferError::UnsupportedDepth(depth));
        }

        // Ensure that the buffer has been allocated somewhat-correctly.
        let buffer = response.allocate_buffer_response();
        if buffer.size == 0 || buffer.base_address == 0 {
            return Err(FramebufferError::FailedToAllocateBuffer);
        }

        // Ensure that the pitch is not 0.
        let pitch = response.get_pitch_response().bytes_per_line;
        if pitch == 0 {
            return Err(FramebufferError::InvalidPitch(pitch));
        }

        Ok(())
    }
}

/// # Safety
/// - We always use [Framebuffer] within a [crate::Mutex].
unsafe impl Send for Framebuffer {}

/// # Safety
/// - We always use [Framebuffer] within a [crate::Mutex].
unsafe impl Sync for Framebuffer {}
