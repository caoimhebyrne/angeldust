use super::types::{Message, MessageStatus, MessageTag};
use crate::{cpu::RaspberryPi, print, println};
use bitflags::bitflags;
use core::{
    fmt::Debug,
    ptr::{read_volatile, write_volatile},
};

#[derive(Debug)]
pub enum MailboxError {
    /// Occurs when the mailbox receives [MessageStatus::Error] as a response.
    /// There's nothing that can be done to gain further information about this case.
    Errored,

    /// Occurs when the mailbox recieves [MessageStatus::Request] as a response.
    /// There's nothing that can be done to gain further information about this case.
    NotAcknowledged,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u32)]
pub enum Channel {
    /// https://github.com/raspberrypi/firmware/wiki/Mailbox-framebuffer-interface
    // Framebuffer = 1,

    /// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface
    PropertyTags = 8,
}

/// A [Mailbox] handles communication between the CPU and the VideoCore.
///
/// https://github.com/raspberrypi/firmware/wiki/Mailboxes
#[derive(Clone, Copy)]
pub struct Mailbox {
    registers: Registers,
}

/// https://github.com/raspberrypi/firmware/wiki/Mailboxes#mailbox-registers
#[derive(Clone, Copy)]
struct Registers {
    // Mailbox 0 is used for reading.
    read: *mut u32,
    read_status: *mut u32,

    // Mailbox 1 is used for writing.
    write: *mut u32,
    write_status: *mut u32,
}

bitflags! {
    struct Flags : u32 {
        const WriteMailboxFull = 0x80000000;
        const ReadMailboxEmpty = 0x40000000;
    }
}

impl Mailbox {
    /// Creates a new instance of the [Mailbox] for communication.
    pub fn new() -> Mailbox {
        let base_address = unsafe {
            RaspberryPi::instance()
                .peripheral_base_address()
                .byte_offset(0xb880)
        };

        Mailbox {
            registers: unsafe { Registers::new(base_address) },
        }
    }

    pub fn send<Request: Debug, Response: Debug>(
        &self,
        channel: Channel,
        request: Message<Request>,
    ) -> Result<Response, MailboxError> {
        // FIXME: Deal with memory caching when the MMU is enabled.
        let ptr = (&request as *const Message<Request>) as u32;
        self.write(ptr, channel);

        let response = unsafe { read_volatile(ptr as *const Message<Response>) };
        match response.status {
            MessageStatus::Success => Ok(response.data),
            MessageStatus::Error => Err(MailboxError::Errored),
            MessageStatus::Request => Err(MailboxError::NotAcknowledged),
        }
    }

    pub fn send_single<Request: Debug, Response: Debug>(
        &self,
        channel: Channel,
        request: Request,
    ) -> Result<Response, MailboxError> {
        self.send::<_, MessageTag<Response>>(channel, Message::new(request))
            .map(|it| it.data)
    }

    pub fn write(&self, value: u32, channel: Channel) {
        while self
            .get_write_status_register()
            .contains(Flags::WriteMailboxFull)
        {}

        // The upper 28 bits are the pointer to the data, while the lower 28 bits are
        // the channel's identifier.
        let request = (value & !0xF) | (channel as u32 & 0xF);
        println!(
            "[angeldust::mailbox] sending a request with data at: {:#0x}",
            request
        );
        unsafe { write_volatile(self.registers.write, request) }

        loop {
            // Wait until the mailbox has something to be
            while self
                .get_read_status_register()
                .contains(Flags::ReadMailboxEmpty)
            {}

            // The response should always be at the same address as the request,
            // if it's not, this message isn't for us.
            let response = unsafe { read_volatile(self.registers.read) };
            if request == response {
                println!(
                    "[angeldust::mailbox] mailbox replied with data at: {:#0x}",
                    response
                );

                break;
            }
        }
    }

    /// Reads and parses from the read status register.
    fn get_read_status_register(&self) -> Flags {
        let value = unsafe { read_volatile(self.registers.read_status) };
        Flags::from_bits_retain(value)
    }

    /// Reads and parses from the write status register.
    fn get_write_status_register(&self) -> Flags {
        let value = unsafe { read_volatile(self.registers.write_status) };
        Flags::from_bits_retain(value)
    }
}

impl Registers {
    /// Creates a new instance of [MailboxRegisters].
    ///
    /// # Safety
    /// - This assumes that the provided [mailbox_base] is valid.
    pub unsafe fn new(mailbox_base: *mut u8) -> Registers {
        let mailbox_0: *mut u32 = mailbox_base.cast();
        let mailbox_1: *mut u32 = mailbox_base.byte_offset(0x20).cast();

        Registers {
            read: mailbox_0,
            read_status: mailbox_0.byte_offset(0x18),

            write: mailbox_1,
            write_status: mailbox_1.byte_offset(0x18),
        }
    }
}

/// # Safety
/// - We always use [Mailbox] within a [crate::Mutex].
unsafe impl Send for Mailbox {}

/// # Safety
/// - We always use [Mailbox] within a [crate::Mutex].
unsafe impl Sync for Mailbox {}
