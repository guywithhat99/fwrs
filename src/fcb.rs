//! FlexSPI configuration block tuned for the Teensy 4.1's 8 MiB QSPI flash.
//!
//! The contents are adapted from the `teensy4-fcb` crate but adjusted so the
//! reported flash size matches the Winbond W25Q64 part that ships on the
//! Teensy 4.1. TyTools uses this metadata when it determines board
//! compatibility, so providing the correct size keeps `tycmd` happy.

#![allow(dead_code)]

use imxrt_boot_gen::flexspi::{self, opcodes::sdr::*, *};
use imxrt_boot_gen::serial_flash::nor;

/// Instructions for the Winbond W25Q64 flash device used on the Teensy 4.1.
mod winbond {
    pub const FAST_READ_QUAD_IO: u8 = 0xEB;
    pub const READ_STATUS_REGISTER_1: u8 = 0x05;
    pub const WRITE_ENABLE: u8 = 0x06;
    pub const SECTOR_ERASE: u8 = 0x20;
    pub const PAGE_PROGRAM: u8 = 0x02;
    pub const CHIP_ERASE: u8 = 0x60;
}

use winbond::*;

const SEQ_READ: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, FAST_READ_QUAD_IO))
    .instr(Instr::new(RADDR, Pads::Four, 0x18))
    .instr(Instr::new(DUMMY, Pads::Four, 0x06))
    .instr(Instr::new(READ, Pads::Four, 0x04))
    .build();

const SEQ_READ_STATUS: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, READ_STATUS_REGISTER_1))
    .instr(Instr::new(READ, Pads::One, 0x04))
    .build();

const SEQ_WRITE_ENABLE: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, WRITE_ENABLE))
    .build();

const SEQ_ERASE_SECTOR: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, SECTOR_ERASE))
    .instr(Instr::new(RADDR, Pads::One, 0x18))
    .build();

const SEQ_PAGE_PROGRAM: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, PAGE_PROGRAM))
    .instr(Instr::new(RADDR, Pads::One, 0x18))
    .instr(Instr::new(WRITE, Pads::One, 0x04))
    .build();

const SEQ_CHIP_ERASE: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, CHIP_ERASE))
    .build();

const LUT: LookupTable = LookupTable::new()
    .command(Command::Read, SEQ_READ)
    .command(Command::ReadStatus, SEQ_READ_STATUS)
    .command(Command::WriteEnable, SEQ_WRITE_ENABLE)
    .command(Command::EraseSector, SEQ_ERASE_SECTOR)
    .command(Command::PageProgram, SEQ_PAGE_PROGRAM)
    .command(Command::ChipErase, SEQ_CHIP_ERASE);

const COMMON_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock =
    flexspi::ConfigurationBlock::new(LUT)
        .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
        .cs_hold_time(0x01)
        .cs_setup_time(0x02)
        .column_address_width(ColumnAddressWidth::OtherDevices)
        .device_mode_configuration(DeviceModeConfiguration::Disabled)
        .wait_time_cfg_commands(WaitTimeConfigurationCommands::disable())
        // 0x0080_0000 bytes = 8 MiB, matching the Teensy 4.1 flash size.
        .flash_size(SerialFlashRegion::A1, 0x0080_0000)
        .serial_clk_freq(SerialClockFrequency::MHz60)
        .serial_flash_pad_type(FlashPadType::Quad);

const SERIAL_NOR_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
    nor::ConfigurationBlock::new(COMMON_CONFIGURATION_BLOCK)
        .page_size(256)
        .sector_size(4096)
        .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::MHz30);

/// Linkable FCB symbol consumed by the boot ROM.
#[used]
#[unsafe(no_mangle)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), unsafe(link_section = ".fcb"))]
pub static FLEXSPI_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
    SERIAL_NOR_CONFIGURATION_BLOCK;
